#![allow(missing_docs)]
use serde_json::json;
use hyper::{Body, Request};

// TODO: replace?
pub use goauth::scopes::Scope;
use goauth::{
    auth::{Token, TokenErr},
    credentials::Credentials,
};

use crate::{config::ProxyConfig, http::HttpClient};
use crate::gcp::{GcpError, self};  // Import the whole module
use snafu::{ResultExt, Snafu};
use serde::{Deserialize, Serialize};
use tokio::fs;


/*

TODO:
 1. Modify 'GcpAuthConfig::build()' to detect the creds file type and separate the handling
 2. Add new module 'token_source'
 3. Define new stuct/type 'ExternalCredentials' to hold the new 'external_account' creds file JSON
 4. Define new enum/type 'GcpAuthFileSource' to encapsulate both types of credentials files
 5. Define a 'fetch_service_account_token()' method to load the local ocp service account token
 6. Define a 'fetch_identity_token_from_sts()' method uses the local credentials to obtain an oidc identity token
 7. Define a 'fetch_impersonated_token()' method to exchange the identity token for an oauth access token

- Encapsulated "type" inside CredentialsType as a simple struct.  It serves as a pre-parsing step
to determine which full struct (Credentials or ExternalAccountCreds) to deserialize.
- The enum 'GcpAuthFileSource' separates the two credential types, while still ensuring all types are handled.
*/

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum TokenSourceError {
    #[snafu(display("Failed to parse external credentials: {}", source))]
    ExternalCredentialsParse { source: serde_json::Error },
    #[snafu(display("Service account token is missing"))]
    ServiceAccountToken { source: std::io::Error },
}

impl From<TokenSourceError> for GcpError {
    fn from(err: TokenSourceError) -> Self {
        match err {
            TokenSourceError::ExternalCredentialsParse {
                source } => GcpError::TokenJsonFromStr { source },
            TokenSourceError::ServiceAccountToken {
                source }  => GcpError::FileReadError { source },
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CredentialsType {
    #[serde(rename = "type")]
    t: String, // holds "service_account" or "external_account"
}

impl CredentialsType {
    pub fn get_type(&self) -> &str {
        self.t.as_ref()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExternalAccountCreds {
    #[serde(rename = "type")]
    t: String,
    audience: String,
    subject_token_type: String,
    token_url: String,
    service_account_impersonation_url: String,
    credential_source: CredentialSource,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct CredentialSource {
    file: String,
    format: CredentialSourceFormat,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CredentialSourceFormat {
    #[serde(rename = "type")]
    format_type: String,
}

#[derive(Clone, Debug)]
pub enum GcpAuthFileSource {
    External(ExternalAccountCreds), // new type and impl
    ServiceAccount(Credentials), // existing logic
}

impl GcpAuthFileSource {
    pub(crate) async fn from_file(path: &str) -> Result<GcpAuthFileSource, GcpError> {
        // match on the credentials_type string to determine our source file type
        let creds_file: CredentialsType = Self::get_creds_type(path).await?;

        match creds_file.get_type() {
            "service_account" => fs::read_to_string(path).await.context(gcp::FileReadSnafu)
                .and_then(|json_str| serde_json::from_str(&json_str).context(gcp::TokenJsonFromStrSnafu))
                .map(Self::ServiceAccount),

            "external_account" => fs::read_to_string(path).await.context(gcp::FileReadSnafu)
                .and_then(|json_str| serde_json::from_str(&json_str).context(gcp::TokenJsonFromStrSnafu))
                .map(Self::External),

            other => {
                debug!("Unknown credential type found: {:?}", other);
                Err(GcpError::InvalidCredentials {
                    source: "Unsupported or missing credential type".into(),
                }.into())
            }
        }
    }

    async fn get_creds_type(path: &str) -> Result<CredentialsType, GcpError> {
        let content = fs::read_to_string(path).await
            .context(gcp::FileReadSnafu)?;
        let creds_file: CredentialsType = serde_json::from_str(&content)
            .context(gcp::TokenJsonFromStrSnafu)?;

        match creds_file.get_type() {
            "service_account" | "external_account" => Ok(creds_file),
            other => {
                debug!("Unknown credential type found: {:?}", other);
                Err(GcpError::InvalidCredentials {
                    source: "Unsupported or missing credential type".into(),
                }.into())
            }
        }
    }
}

impl ExternalAccountCreds {
    pub fn url(&self) -> String {
        self.service_account_impersonation_url.clone()
    }

    async fn fetch_bound_sa_token_as_string(token_file_path: &str) -> crate::Result<String> {
        let token = fs::read_to_string(token_file_path).await
            .context(ServiceAccountTokenSnafu)?;
        Ok(token.trim().to_string())
    }

    // async fn fetch_bound_sa_token_from_field_path(token_file_path: &str) -> crate::Result<String> {
    //     let creds = fs::read_to_string(token_file_path).await
    //         .context(ServiceAccountTokenSnafu)?;
    //     // .context(FileReadSnafu)
    //     // .map_err(Into::<GcpError>::into)?;
    //     let bound_token_path =
    //     Ok(token.trim().to_string())
    // }

    async fn fetch_identity_token_from_sts(creds: &ExternalAccountCreds, bound_token: &str) -> Result<Token, GcpError> {
        // get token using sts service
        let token_url = &creds.token_url;

        let request_body = json!({
            "grant_type": "urn:ietf:params:oauth:grant-type:token-exchange",
            "requested_token_type": "urn:ietf:params:oauth:token-type:access_token",
            "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
            "subject_token": bound_token,
            "audience": &creds.audience
        });

        let req = Request::post(token_url)
            .header("Content-Type", "application/json")
            .body(Body::from(request_body.to_string()))
            .unwrap();
        eprintln!("Full Request: {:?}", req);
        eprintln!("Request URL: {}", token_url);
        eprintln!("Request Headers:");
        for (key, value) in req.headers() {
            eprintln!("  {}: {:?}", key, value);
        }
        if let Some(body) = req.body().as_ref() {
            if let Ok(body_str) = std::str::from_utf8(body) {
                eprintln!("Request Body: {}", body_str);
            } else {
                eprintln!("Request Body: [Binary data]");
            }
        }

        let client = HttpClient::new(None, &ProxyConfig::from_env())
            .context(gcp::BuildHttpClientSnafu)?;
        let response = client.send(req).await
            .context(gcp::GetTokenRequestSnafu)?;
        let body = hyper::body::to_bytes(response.into_body()).await
            .context(gcp::GetTokenBytesSnafu)?;
        eprintln!("===== fetch_identity_token_from_sts() -- response.into_body(): {:?}", body);
        Self::parse_token_response(&body)
    }

    async fn fetch_impersonated_token(creds: &ExternalAccountCreds, identity_token: &str) -> Result<Token, GcpError> {
        debug!("Fetching GCP access token via impersonation");
        let impersonate_url = &creds.url();

        let request_body = json!({
            "scope": "https://www.googleapis.com/auth/cloud-platform",
        });

        let req = Request::post(impersonate_url)
            .header("Authorization", format!("Bearer {:?}", identity_token))
            .header("Content-Type", "application/json")
            .body(Body::from(request_body.to_string()))
            .unwrap();

        let client = HttpClient::new(None, &ProxyConfig::from_env()).context(gcp::BuildHttpClientSnafu)?;
        let response = client.send(req).await.context(gcp::GetTokenRequestSnafu)?;
        let body = hyper::body::to_bytes(response.into_body())
            .await
            .context(gcp::GetTokenBytesSnafu)?;

        Self::parse_token_response(&body)
    }

    fn parse_token_response(body: &[u8]) -> Result<Token, GcpError> {
        match serde_json::from_slice::<Token>(body) {
            Ok(token) => Ok(token),
            Err(error) => Err(match serde_json::from_slice::<TokenErr>(body) {
                Ok(token_error) => GcpError::TokenFromJson { source: token_error },
                Err(_) => GcpError::TokenJsonFromStr { source: error },
            }),
        }
    }

    fn parse_token_from_response(token: Token) -> Result<Token, GcpError> {
        match token.access_token().is_empty() {
            true => Err(GcpError::GetToken {
                source: "Unable to get access token from response body".into(),
            }),
            false => Ok(token),
        }
    }

    pub(crate) async fn fetch_external_token(creds: &ExternalAccountCreds) -> Result<Token, GcpError> {
        // Fetch service account token (handling errors)
        let sa_token = Self::fetch_bound_sa_token_as_string(&creds.credential_source.file).await.unwrap();
        debug!("fetch_service_account_token() - service_account_token: {}", sa_token);

        // Fetch identity token using STS (handling errors)
        let identity_token = Self::fetch_identity_token_from_sts(&creds, &sa_token).await?;
        debug!("fetch_identity_token_from_sts() - identity_token: {}", identity_token.access_token());

        // Fetch impersonated token (handling errors)
        let access_token = Self::fetch_impersonated_token(&creds, &identity_token.access_token()).await?;
        debug!("fetch_impersonated_token() - access_token: {}", access_token.access_token());

        Self::parse_token_from_response(access_token)
        // Ok(access_token)
    }


    /* original simple -- not tested but compiles */
    // pub(crate) async fn fetch_external_token(creds: &ExternalAccountCreds) -> Result<Token, GcpError> {
    //     let sa_token = Self::fetch_service_account_token(&creds.credential_source.file).await.unwrap();
    //     debug!("fetch external - service_account_token: {}", sa_token);
    //     let identity_token = Self::fetch_identity_token_from_sts(&creds, &sa_token).await?;
    //     debug!("fetch external - identity_token: {}", identity_token.access_token().to_string());
    //     let access_token = Self::fetch_impersonated_token(&creds, &identity_token.to_string()).await?;
    //     debug!("fetch external - access_token: {}", access_token.access_token().to_string());
    //
    //     // TODO: handle errors
    //     Ok(access_token)
    // }

    //// simple
    // fn read_ocp_token(token_path: &str) -> crate::Result<String> {
    //     let token = std::fs::read_to_string(token_path)?.trim().to_string();
    //     Ok(token)
    // }

    //// was working at one point
    // fn from_json_file(path: &str) -> crate::Result<Self> {
    //     let json_file_path = std::path::Path::new(path);
    //     let file = std::fs::File::open(json_file_path).expect("file not found");
    //     let creds:Vec<ExternalAccount> = serde_json::from_reader(file).expect("error while reading");
    //     for cred in creds {
    //         println!("audience: {} \nimpersonation_url: {} \n", cred.audience, cred.service_account_impersonation_url);
    //         println!("credentials_path: {} ", cred.credential_source);
    //     }
    //     Ok(creds.to_string())
    //     // match creds.to_string() {
    //     //     Ok(creds) => Ok(creds.to_string()),
    //     //     Err(error) => Err(GcpError::TokenJsonFromStr { source: error }),
    //     // }
    // }
}

/* ------------------------------------------------------------------------------
External Account Details

Workload Identity Federation (WIF):
A cluster admin has configured an Identity Pool and Provider in a cloud storage bucket.
The OIDC configuration allows access to cloud resources using an external identity.
When configured in the workload identity pool, an external identity is authorized
by exchanging the cluster token for a short-lived GCP access token.

IAM Policy and RBAC:
Service account impersonation is used to provide your workload with access to Google Cloud resources.
The Openshift service account (external identity) is configured in the identity pool and bound
to a GCP role.   This role and configuration grants the external identity permission to impersonate
the cloud service account via an Oauth exchange.

RSA Key:
When a GCP cluster is configured to use Workload Identity Federation, the OIDC configurations
and public key are served via a GCP storage bucket (identity provider).
A private key is used by the Openshift API server to sign and manage cluster service account tokens.

GCP Credentials Config File:
A local credentials JSON file must be specified in the config and includes:
* "type": Must be "external_account".
* "audience": The identity provider (workload identity pool and provider)
* "subject_token_type": "urn:ietf:params:oauth:token-type:jwt"
* "token_url": The sts service endpoint ("https://sts.googleapis.com/v1/token")
* "service_account_impersonation_url": URL of the GCP service account to impersonate
* "credential_source.file": Specifies the path to the projected JWT token.

Logging Deployment:
A projected service account token is mounted inside the pod. This token
is a short-lived JWT that provides access to the service account inside the OpenShift cluster.
This "bound" sa token can be exchanged for an OIDC access token to be used for GCP authentication.

STS Token Exchange:
The local sa token is sent to the Google STS API to be exchanged for an OIDC token.

SA Impersonation Usage:
The identity token is sent via impersonation API and exchanged for a short-lived access token

------------------------------------------------------------------------------------
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{self, Read};
    use std::error::Error;
    use std::str::FromStr;

    //WIP
    /* ------------------------------------------------ */
    // ExternalAccountCreds::fetch_service_account_token()
    async fn fetch_bound_sa_token_helper(token_path: &str) -> crate::Result<String> {
        let bound_token = ExternalAccountCreds::fetch_bound_sa_token_as_string(token_path).await?;
        Ok(bound_token)
    }

    #[tokio::test]
    async fn test_fetch_bound_sa_token() {
        let path = "tests/data/gcp/external_account2.json";
        let token_text = "This is okay.  <>  I've done a thing.";
        let source = from_file_helper(path).await;

        let Ok(GcpAuthFileSource::External(creds)) = source else {
            unreachable!("Expected 'ExternalAccountCreds' variant");
        };

        let file_path = creds.credential_source.file;
        eprintln!("------ test_fetch_bound_sa_token() -- file_path: {:?}", file_path);

        let bound = fetch_bound_sa_token_helper(&file_path).await.unwrap();
        eprintln!("------ test_fetch_bound_sa_token() -- fetch_bound_sa_token_helper() - token: {:?}", bound);
        assert_eq!(bound, token_text);

        /*
        // match source {
        //     GcpAuthFileSource::External(creds) => {
        //         let file_path = creds.credential_source.file;
        //         println!("Credential file path: {}", file_path);
        //     }
        //     GcpAuthFileSource::ServiceAccount(_) => {
        //         println!("Not an external account, skipping.");
        //     }
        // }

        // if let GcpAuthFileSource::External(creds) = source {
        //     let file_path = creds.credential_source.file;
        //     println!("Credential file path: {}", file_path);
        // }

        // let file_path = match source {
        //     GcpAuthFileSource::External(
        //         ExternalAccountCreds { credential_source: CredentialSource { file }, .. }) => file,
        //     _ => unreachable!("Expected ExternalAccountCreds variant"),
        // };
        */
    }

    //WIP
    /* ------------------------------------------------ */
    // ExternalAccountCreds::fetch_identity_token_from_sts()
    async fn fetch_identity_token_from_sts_helper(creds: &ExternalAccountCreds, token_as_string_for_now: &String) -> Result<Token, GcpError> {
        let bound_token = ExternalAccountCreds::fetch_identity_token_from_sts(creds, token_as_string_for_now).await?;
        Ok(bound_token)
    }

    #[tokio::test]
    async fn test_fetch_identity_token_from_sts() {
        let path = "tests/data/gcp/external_account.json";
        let source = from_file_helper(path).await;
        let Ok(GcpAuthFileSource::External(creds)) = source else {
            unreachable!("Expected 'ExternalAccountCreds' variant");
        };
        let file_path = creds.credential_source.file.to_string();
        eprintln!("---- test_fetch_identity_token_from_sts() -- creds_file_path: {:?}", file_path);
        let bound = fetch_bound_sa_token_helper(&file_path).await.unwrap();
        eprintln!("---- test_fetch_identity_token_from_sts() -- bound_token_contents: {:?}", bound);
        let identity_token = fetch_identity_token_from_sts_helper(&creds, &bound).await;
        eprintln!("---- test_fetch_identity_token_from_sts() -- identity_token: {:?}", identity_token);
        panic!("panic!!! -- success?? -- identity_token: {:?}", identity_token);
    }

    #[tokio::test]
    async fn test_foo() {
        let token_path = "tests/data/gcp/test_token";
        let foofoo = do_the_things(token_path).await;
        eprintln!("--== attempting to parse the jwt - token/error: {:?}", foofoo);
        panic!("---== forced failure -- : {:?}", foofoo)
    }

    async fn do_the_things(token_file_path: &str) -> Result<Token, Box<dyn Error>> {
        match read_token_as_str_from_file(token_file_path).await {
            Ok(token_str) => match parse_jwt(&token_str).await {
                Ok(token) => {
                    println!("Parsed Token: {:?}", token);
                    Ok(token)
                }
                Err(e) => {
                    eprintln!("Failed to parse JWT: {}", e);
                    Err(e)
                }
            },
            Err(e) => {
                eprintln!("Failed to read token file: {}", e);
                Err(Box::new(e))
            }
        }
        // let token_str = read_token_from_file(token_file_path).await?;
        // let token = parse_jwt(&token_str).await;
        // println!("Parsed Token: {:?}", token);
        // Ok(token)

    }
    async fn read_token_as_str_from_file(path: &str) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut token_string = String::new();
        file.read_to_string(&mut token_string)?;
        println!(" -- read_token_as_str_from_file() - token: {:?}", token_string);
        Ok(token_string)
    }

    async fn parse_jwt(token_str: &str) -> Result<Token, Box<dyn Error>> {
        let token = Token::from_str(token_str)?;
        println!(" -- parse_jwt() - token: {:?}", token);
        Ok(token)
    }
    /*
        let serialized_jwt =
            "eyJhbGciOiJSUzI1NiJ9.eyJpc3MiOiJodHRwczovL2V4YW1wbGUuY29tIiwiYXVkIjpbIm15X2NsaWVudCJdL\
             CJleHAiOjE1NDQ5MzIxNDksImlhdCI6MTU0NDkyODU0OSwiYXV0aF90aW1lIjoxNTQ0OTI4NTQ4LCJub25jZSI\
             6InRoZV9ub25jZSIsImFjciI6InRoZV9hY3IiLCJzdWIiOiJzdWJqZWN0In0.gb5HuuyDMu-LvYvG-jJNIJPEZ\
             823qNwvgNjdAtW0HJpgwJWhJq0hOHUuZz6lvf8ud5xbg5GOo0Q37v3Ke08TvGu6E1USWjecZzp1aYVm9BiMvw5\
             EBRUrwAaOCG2XFjuOKUVfglSMJnRnoNqVVIWpCAr1ETjZzRIbkU3n5GQRguC5CwN5n45I3dtjoKuNGc2Ni-IMl\
             J2nRiCJOl2FtStdgs-doc-A9DHtO01x-5HCwytXvcE28Snur1JnqpUgmWrQ8gZMGuijKirgNnze2Dd5BsZRHZ2\
             CLGIwBsCnauBrJy_NNlQg4hUcSlGsuTa0dmZY7mCf4BN2WCpyOh0wgtkAgQ";
        let id_token = serde_json::from_value::<CoreIdToken>(serde_json::Value::String(
            serialized_jwt.to_string(),
        ))
        .unwrap();

        // --------------

        let corrupted_jwt_str = TEST_JWT
            .to_string()
            .chars()
            .take(TEST_JWT.len() - 1)
            .collect::<String>()
            + "f";
        let jwt: JsonWebToken<
            CoreJweContentEncryptionAlgorithm,
            CoreJwsSigningAlgorithm,
            CoreJsonWebKeyType,
            String,
            JsonWebTokenStringPayloadSerde,
        > = serde_json::from_value(serde_json::Value::String(corrupted_jwt_str))
            .expect("failed to deserialize");
    */




    //WIP
    /* ----------------------------------- */
    // GcpAuthFileSource::get_creds_type()
    async fn get_credentials_type_helper(file_path: &str) -> crate::Result<String> {
        let cred_type = GcpAuthFileSource::get_creds_type(file_path).await?;
        debug!("test of creds file 'type': {:?}", cred_type);
        Ok(cred_type.get_type().to_string())
    }

    #[tokio::test]
    async fn test_get_creds_type_service_account() {
        let path = "tests/data/gcp/service_account.json";
        let result = get_credentials_type_helper(path).await.unwrap();
        debug!("test of creds file 'type': {:?}", result);
        assert_eq!(result, "service_account");
    }

    #[tokio::test]
    async fn test_get_creds_type_external_account() {
        let path = "tests/data/gcp/external_account.json";
        let result = get_credentials_type_helper(path).await.unwrap();
        debug!("test of creds file 'type': {:?}", result);
        assert_eq!(result, "external_account");
    }

    #[tokio::test]
    async fn test_get_creds_type_invalid() {
        let path = "tests/data/gcp/invalid_type.json";
        let result = get_credentials_type_helper(path).await;
        debug!("test of creds file 'type': {:?}", result);
        assert!(result.is_err(), "Expected an error for invalid credentials");
    }

    #[tokio::test]
    async fn test_get_creds_type_empty() {
        let path = "tests/data/gcp/empty_object.json";
        let result = get_credentials_type_helper(path).await;
        debug!("creds file 'type': {:?}", result);
        assert!(result.is_err(), "Expected an error for invalid credentials");
    }

    #[tokio::test]
    async fn test_get_creds_type_invalid_file() {
        let path = "tests/data/gcp/invalid_file.txt";
        let result = get_credentials_type_helper(path).await;
        debug!("creds file 'type': {:?}", result);
        assert!(result.is_err(), "Expected an error for invalid credentials");
    }

    //WIP
    /* ------------------------------- */
    // GcpAuthFileSource::file_source()
    async fn from_file_helper(file_path: &str) -> Result<GcpAuthFileSource, GcpError> {
        GcpAuthFileSource::from_file(file_path).await
    }

    #[tokio::test]
    async fn test_external_account_file_source() {
        let path = "tests/data/gcp/external_account.json";
        let source = from_file_helper(path).await;
        eprintln!("external_account file 'source': {:?}", source);
        assert!(matches!(source, Ok(GcpAuthFileSource::External(_))), "Expected 'External' variant");
    }

    #[tokio::test]
    async fn test_service_account_file_source() {
        let path = "tests/data/gcp/service_account.json";
        let source = from_file_helper(path).await;
        eprintln!("service_account file 'source': {:?}", source);
        assert!(matches!(source, Ok(GcpAuthFileSource::ServiceAccount(_))), "Expected 'ServiceAccount' variant");
    }

    #[tokio::test]
    async fn test_invalid_type_file_source() {
        let path = "tests/data/gcp/invalid_creds.json";
        let result = from_file_helper(path).await;
        assert!(result.is_err(), "Expected an error for invalid credentials");
    }

    /* ======= Notes and TODOs ======*/
    // #[tokio::test]
    // async fn test_external_account_auth() {
    //     // define the path to the sample credentials file
    //     let val = "tests/data/gcp/sample_credentials.json";
    //     let path = std::path::Path::new(val);
    //
    //     // create the directory
    //     if let Some(parent) = path.parent() {
    //         tokio::fs::create_dir_all(parent).await.unwrap();
    //     }

    // sample credentials json
    // let sample_credentials = json!({
    //     "type": "external_account",
    //     "audience": "//iam.googleapis.com/projects/1234567890/locations/global/workloadIdentityPools/test-0206/providers/test-0206",
    //     "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
    //     "token_url": "https://sts.googleapis.com/v1/token",
    //     "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/test-020-logging-gcp-2lnh6@test-account.iam.gserviceaccount.com:generateAccessToken",
    //     "credential_source": {
    //       "file": "/var/run/ocp-collector/serviceaccount/token",
    //       "format": {
    //         "type": "text"
    //       }
    //     },
    // });

    // let sample_credentials = json!({
    //       "type": "service_account",
    //       "project_id": "openshift-observability",
    //       "private_key_id": "0f3c7cc1cc3aa5555555555c68cc2c318a2",
    //       "private_key": "-----BEGIN PRIVATE KEY-----\ngktest9w0BAQEFAASCBKcwggSjAgEAAoIBAQDC1fJlMIIEvQIBADANBZNWjVHik\nYV+uaKOiJIGjlsN1MxA=\n-----END PRIVATE KEY-----\n",
    //       "client_email": "test-0123-gcp-testing@openshift-observability.iam.gserviceaccount.com",
    //       "client_id": "1234567890123456",
    //       "auth_uri": "https://accounts.google.com/o/oauth2/auth",
    //       "token_uri": "https://oauth2.googleapis.com/token",
    //       "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
    //       "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/test-0123-gcp-testing%40openshift-observability.iam.gserviceaccount.com",
    //       "universe_domain": "googleapis.com"
    // });

    // // write the sample credentials to the file
    // tokio::fs::write(&path, sample_credentials.to_string()).await.unwrap();



    // let config = GcpAuthConfig {
    //     credentials_path: Some(path.to_str().unwrap().to_string()),
    //     api_key: None,
    //     skip_authentication: false,
    // };
    // let result = config.build(Scope::Compute).await;
    // // assert!(result.is_ok(), "External Account authentication should succeed");
    // assert!(matches!(result, Ok(..)));


    // let val = path.to_str().unwrap().to_string();
    // let auth = build_auth(&format!(r#"credentials_path = "{val}""#))
    // let auth = build_auth(
    //     r#"
    //        credentials_path = "tests/data/sample_credentials.json"
    //    "#,
    // )
    //     .await
    //     .expect("build_auth failed");
    //
    // assert!(matches!(auth, GcpAuthenticator::Credentials(..)));

    //     // clean up credentials file
    //     tokio::fs::remove_file(&path).await.unwrap();
    // }
}
