#![allow(missing_docs)]
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use base64::prelude::{BASE64_URL_SAFE, Engine as _};
use google_cloud_auth::credentials::{AccessTokenCredentials, Builder};
use http::Uri;
use hyper::header::AUTHORIZATION;
use snafu::{ResultExt, Snafu};
use tokio::sync::watch;
use vector_lib::configurable::configurable_component;
use vector_lib::sensitive_string::SensitiveString;

// See https://cloud.google.com/compute/docs/access/authenticate-workloads#applications
const TOKEN_REFRESH_INTERVAL_SECS: u64 = 3300; // 55 minutes (tokens last 1 hour)

pub const PUBSUB_URL: &str = "https://pubsub.googleapis.com";

pub static PUBSUB_ADDRESS: LazyLock<String> = LazyLock::new(|| {
    std::env::var("EMULATOR_ADDRESS").unwrap_or_else(|_| "http://localhost:8681".into())
});

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum GcpError {
    #[snafu(display("This requires one of api_key or credentials_path to be defined"))]
    MissingAuth,
    #[snafu(display("Invalid GCP credentials: {}", source))]
    InvalidCredentials {
        source: google_cloud_auth::build_errors::Error,
    },
    #[snafu(display("Invalid GCP API key: {}", source))]
    InvalidApiKey { source: base64::DecodeError },
    #[snafu(display("Healthcheck endpoint forbidden"))]
    HealthcheckForbidden,
    #[snafu(display("Failed to get OAuth token: {}", source))]
    GetToken {
        source: google_cloud_auth::errors::CredentialsError,
    },
}

pub mod scopes {
    pub const CLOUD_STORAGE: &str = "https://www.googleapis.com/auth/devstorage.read_write";
    pub const PUBSUB: &str = "https://www.googleapis.com/auth/pubsub";
    pub const MONITORING_WRITE: &str = "https://www.googleapis.com/auth/monitoring.write";
    pub const LOGGING_WRITE: &str = "https://www.googleapis.com/auth/logging.write";
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    pub const MALACHITE_INGESTION: &str = "https://www.googleapis.com/auth/malachite-ingestion";
}

/// Configuration of the authentication strategy for interacting with GCP services.
///
/// Supports multiple authentication methods in priority order:
/// 1. API Key authentication (if `api_key` is set)
/// 2. Service Account credentials (if `credentials_path` points to a service account JSON file)
/// 3. External Account credentials for Workload Identity Federation (if `credentials_path` points to a WIF config)
/// 4. Application Default Credentials (ADC) - automatic fallback when neither `api_key` nor `credentials_path` is set
///
/// ## Application Default Credentials (ADC) Fallback
///
/// When neither `api_key` nor `credentials_path` is explicitly configured, Vector automatically
/// attempts to use Application Default Credentials. ADC searches for credentials in this order:
///
/// 1. `GOOGLE_APPLICATION_CREDENTIALS` environment variable pointing to a credentials file
/// 2. gcloud CLI credentials (`~/.config/gcloud/application_default_credentials.json`)
/// 3. GCE/GKE metadata server (when running on Google Cloud infrastructure)
///
/// This ADC fallback is the **recommended approach** for production deployments as it:
/// - Eliminates the need to manage credential files in configuration
/// - Supports Workload Identity Federation on GKE automatically
/// - Works seamlessly across development and production environments
/// - Follows Google Cloud security best practices
#[configurable_component]
#[derive(Clone, Debug, Default)]
pub struct GcpAuthConfig {
    /// An [API key][gcp_api_key].
    ///
    /// Either an API key or a path to a credentials JSON file can be specified.
    ///
    /// If both are unset, the `GOOGLE_APPLICATION_CREDENTIALS` environment variable is checked for a filename. If no
    /// filename is named, an attempt is made to fetch an instance service account for the compute instance the program is
    /// running on. If this is not on a GCE instance, then you must define it with an API key or credentials JSON file.
    ///
    /// [gcp_api_key]: https://cloud.google.com/docs/authentication/api-keys
    pub api_key: Option<SensitiveString>,

    /// Path to a credentials JSON file.
    ///
    /// This can be either:
    /// - A [service account][gcp_service_account_credentials] credentials file
    /// - An [external account][gcp_external_account] credentials file for Workload Identity Federation
    ///
    /// Either an API key or a path to a credentials JSON file can be specified.
    ///
    /// If both are unset, the `GOOGLE_APPLICATION_CREDENTIALS` environment variable is checked for a filename. If no
    /// filename is named, an attempt is made to fetch an instance service account for the compute instance the program is
    /// running on. If this is not on a GCE instance, then you must define it with an API key or credentials JSON file.
    ///
    /// [gcp_service_account_credentials]: https://cloud.google.com/docs/authentication/production#manually
    /// [gcp_external_account]: https://cloud.google.com/iam/docs/workload-identity-federation
    pub credentials_path: Option<String>,

    /// Skip all authentication handling. For use with integration tests only.
    #[serde(default, skip_serializing)]
    #[configurable(metadata(docs::hidden))]
    pub skip_authentication: bool,
}

impl GcpAuthConfig {
    pub async fn build(&self, scopes: &[&str]) -> crate::Result<GcpAuthenticator> {
        Ok(if self.skip_authentication {
            GcpAuthenticator::None
        } else {
            match (&self.credentials_path, &self.api_key) {
                (Some(path), _) => GcpAuthenticator::from_file(path, scopes).await?,
                (None, Some(api_key)) => GcpAuthenticator::from_api_key(api_key.inner())?,
                (None, None) => GcpAuthenticator::from_adc(scopes).await?,
            }
        })
    }
}

#[derive(Clone)]
pub enum GcpAuthenticator {
    Credentials(Arc<AccessTokenCredentials>),
    ApiKey(Box<str>),
    None,
}

impl std::fmt::Debug for GcpAuthenticator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Credentials(_) => f
                .debug_tuple("Credentials")
                .field(&"<credentials>")
                .finish(),
            Self::ApiKey(_) => f.debug_tuple("ApiKey").field(&"<redacted>").finish(),
            Self::None => write!(f, "None"),
        }
    }
}

impl GcpAuthenticator {
    /// create authenticator from a credentials file.
    async fn from_file(path: &str, scopes: &[&str]) -> crate::Result<Self> {
        debug!(
            message = "Loading GCP credentials from file.",
            path = ?path,
        );

        // The google-cloud-auth Builder has no with_credentials_file() method.
        // The GOOGLE_APPLICATION_CREDENTIALS env var is the only way to pass
        // a credentials path. ScopedEnv restores the original value on drop.
        let _guard = ScopedEnv::set("GOOGLE_APPLICATION_CREDENTIALS", path);

        let scopes_vec: Vec<String> = scopes.iter().map(|s| s.to_string()).collect();
        let credentials = Builder::default()
            .with_scopes(scopes_vec)
            .build_access_token_credentials()
            .context(InvalidCredentialsSnafu)?;

        Ok(Self::Credentials(Arc::new(credentials)))
    }

    async fn from_adc(scopes: &[&str]) -> crate::Result<Self> {
        debug!("Loading GCP credentials using Application Default Credentials (ADC).");

        let scopes_vec: Vec<String> = scopes.iter().map(|s| s.to_string()).collect();
        let credentials = Builder::default()
            .with_scopes(scopes_vec)
            .build_access_token_credentials()
            .context(InvalidCredentialsSnafu)?;

        Ok(Self::Credentials(Arc::new(credentials)))
    }

    fn from_api_key(api_key: &str) -> crate::Result<Self> {
        BASE64_URL_SAFE
            .decode(api_key)
            .context(InvalidApiKeySnafu)?;
        Ok(Self::ApiKey(api_key.into()))
    }

    pub fn make_token(&self) -> Option<String> {
        match self {
            Self::Credentials(creds) => {
                let token = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current()
                        .block_on(async { creds.access_token().await.ok() })
                });
                token.map(|t| format!("Bearer {}", t.token))
            }
            Self::ApiKey(_) | Self::None => None,
        }
    }

    pub fn apply<T>(&self, request: &mut http::Request<T>) {
        if let Some(token) = self.make_token() {
            request
                .headers_mut()
                .insert(AUTHORIZATION, token.parse().unwrap());
        }
        self.apply_uri(request.uri_mut());
    }

    pub fn apply_uri(&self, uri: &mut Uri) {
        match self {
            Self::Credentials(_) | Self::None => (),
            Self::ApiKey(api_key) => {
                let mut parts = uri.clone().into_parts();
                let path = parts.path_and_query.as_ref().map_or("/", |pq| pq.path());
                let paq = format!("{path}?key={api_key}");
                // The API key is verified above to only contain
                // URL-safe characters. That key is added to a path
                // that came from a successfully parsed URI. As such,
                // re-parsing the string cannot fail.
                parts.path_and_query =
                    Some(paq.parse().expect("Could not re-parse path and query"));
                *uri = Uri::from_parts(parts).expect("Could not re-parse URL");
            }
        }
    }

    /// Spawn periodic token refresh to keep credentials fresh.
    pub fn spawn_regenerate_token(&self) -> watch::Receiver<()> {
        let (sender, receiver) = watch::channel(());
        tokio::spawn(self.clone().token_regenerator(sender));
        receiver
    }

    async fn token_regenerator(self, sender: watch::Sender<()>) {
        match self {
            Self::Credentials(creds) => loop {
                let deadline = Duration::from_secs(TOKEN_REFRESH_INTERVAL_SECS);
                debug!(
                    deadline = deadline.as_secs(),
                    "Sleeping before refreshing GCP authentication token.",
                );
                tokio::time::sleep(deadline).await;

                match creds.access_token().await {
                    Ok(_) => {
                        sender.send_replace(());
                        debug!("GCP authentication token refreshed.");
                    }
                    Err(error) => {
                        error!(
                            message = "Failed to refresh GCP authentication token.",
                            %error
                        );
                    }
                }
            },
            Self::ApiKey(_) | Self::None => {
                // This keeps the sender end of the watch open without
                // actually sending anything, effectively creating an
                // empty watch stream.
                sender.closed().await
            }
        }
    }
}

/// Temporarily set an environment variable, restoring the original value on drop.
///
/// Used to pass `credentials_path` to `google-cloud-auth` which only reads
/// credentials via the `GOOGLE_APPLICATION_CREDENTIALS` environment variable.
struct ScopedEnv {
    key: &'static str,
    old_value: Option<String>,
}

impl ScopedEnv {
    fn set(key: &'static str, value: &str) -> Self {
        let old_value = std::env::var(key).ok();
        unsafe {
            std::env::set_var(key, value);
        }
        Self { key, old_value }
    }
}

impl Drop for ScopedEnv {
    fn drop(&mut self) {
        // restoring env var to its original state
        unsafe {
            match &self.old_value {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn skip_authentication() {
        let auth = build_auth(
            r#"
                skip_authentication = true
                api_key = "testing"
            "#,
        )
        .await
        .expect("build_auth failed");
        assert!(matches!(auth, GcpAuthenticator::None));
    }

    #[tokio::test]
    async fn uses_api_key() {
        let key = crate::test_util::random_string(16);

        let auth = build_auth(&format!(r#"api_key = "{key}""#))
            .await
            .expect("build_auth failed");
        assert!(matches!(auth, GcpAuthenticator::ApiKey(..)));

        assert_eq!(
            apply_uri(&auth, "http://example.com"),
            format!("http://example.com/?key={key}")
        );
        assert_eq!(
            apply_uri(&auth, "http://example.com/"),
            format!("http://example.com/?key={key}")
        );
        assert_eq!(
            apply_uri(&auth, "http://example.com/path"),
            format!("http://example.com/path?key={key}")
        );
        assert_eq!(
            apply_uri(&auth, "http://example.com/path1/"),
            format!("http://example.com/path1/?key={key}")
        );
    }

    #[tokio::test]
    async fn fails_bad_api_key() {
        let error = build_auth(r#"api_key = "abc%xyz""#)
            .await
            .expect_err("build failed to error");
        assert!(matches!(
            error.downcast_ref::<GcpError>(),
            Some(GcpError::InvalidApiKey { .. })
        ));
    }

    #[tokio::test]
    async fn falls_back_to_adc() {
        // With no credentials configured, build() attempts ADC.
        // The result is environment-dependent -- just verify no panic.
        let _ = build_auth("").await;
    }

    fn apply_uri(auth: &GcpAuthenticator, uri: &str) -> String {
        let mut uri: Uri = uri.parse().unwrap();
        auth.apply_uri(&mut uri);
        uri.to_string()
    }

    async fn build_auth(toml: &str) -> crate::Result<GcpAuthenticator> {
        let config: GcpAuthConfig = toml::from_str(toml).expect("Invalid TOML");
        config.build(&[scopes::CLOUD_PLATFORM]).await
    }
}
