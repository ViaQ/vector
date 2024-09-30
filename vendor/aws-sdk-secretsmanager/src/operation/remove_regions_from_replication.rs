// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `RemoveRegionsFromReplication`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RemoveRegionsFromReplication;
impl RemoveRegionsFromReplication {
    /// Creates a new `RemoveRegionsFromReplication`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationInput,
    ) -> ::std::result::Result<
        crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
            "secretsmanager",
            "RemoveRegionsFromReplication",
            input,
            runtime_plugins,
            stop_point,
        )
        .await
    }

    pub(crate) fn operation_runtime_plugins(
        client_runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        client_config: &crate::config::Config,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());
        runtime_plugins = runtime_plugins.with_client_plugin(crate::auth_plugin::DefaultAuthOptionsPlugin::new(vec![
            ::aws_runtime::auth::sigv4::SCHEME_ID,
        ]));
        if let ::std::option::Option::Some(config_override) = config_override {
            for plugin in config_override.runtime_plugins.iter().cloned() {
                runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
            }
            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
                config_override,
                client_config.config.clone(),
                &client_config.runtime_components,
            ));
        }
        runtime_plugins
    }
}
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for RemoveRegionsFromReplication {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("RemoveRegionsFromReplication");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            RemoveRegionsFromReplicationRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            RemoveRegionsFromReplicationResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolverParams::new(),
        ));

        cfg.store_put(::aws_smithy_http::operation::Metadata::new(
            "RemoveRegionsFromReplication",
            "secretsmanager",
        ));
        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
        signing_options.double_uri_encode = true;
        signing_options.content_sha256_header = false;
        signing_options.normalize_uri_path = true;
        signing_options.payload_override = None;

        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
            signing_options,
            ..::std::default::Default::default()
        });

        ::std::option::Option::Some(cfg.freeze())
    }

    fn runtime_components(
        &self,
        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
        #[allow(unused_mut)]
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("RemoveRegionsFromReplication")
            .with_interceptor(
                ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::new(
                    ::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptorKind::ResponseBody,
                ),
            )
            .with_interceptor(RemoveRegionsFromReplicationEndpointParamsInterceptor)
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError,
            >::new())
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError,
            >::new())
            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError,
            >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct RemoveRegionsFromReplicationResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for RemoveRegionsFromReplicationResponseDeserializer {
    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().bytes().expect("body loaded");
        #[allow(unused_mut)]
        let mut force_error = false;
        ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
        let parse_result = if !success && status != 200 || force_error {
            crate::protocol_serde::shape_remove_regions_from_replication::de_remove_regions_from_replication_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_remove_regions_from_replication::de_remove_regions_from_replication_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct RemoveRegionsFromReplicationRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for RemoveRegionsFromReplicationRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(builder, ::http::header::CONTENT_TYPE, "application/x-amz-json-1.1");
            builder = _header_serialization_settings.set_default_header(
                builder,
                ::http::header::HeaderName::from_static("x-amz-target"),
                "secretsmanager.RemoveRegionsFromReplication",
            );
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from(
            crate::protocol_serde::shape_remove_regions_from_replication::ser_remove_regions_from_replication_input(&input)?,
        );
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct RemoveRegionsFromReplicationEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for RemoveRegionsFromReplicationEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "RemoveRegionsFromReplicationEndpointParamsInterceptor"
    }

    fn read_before_execution(
        &self,
        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
            '_,
            ::aws_smithy_runtime_api::client::interceptors::context::Input,
            ::aws_smithy_runtime_api::client::interceptors::context::Output,
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
        >,
        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let _input = context
            .input()
            .downcast_ref::<RemoveRegionsFromReplicationInput>()
            .ok_or("failed to downcast to RemoveRegionsFromReplicationInput")?;

        let params = crate::config::endpoint::Params::builder()
            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
            .build()
            .map_err(|err| {
                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
            })?;
        cfg.interceptor_state()
            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
        ::std::result::Result::Ok(())
    }
}

/// Error type for the `RemoveRegionsFromReplicationError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum RemoveRegionsFromReplicationError {
    /// <p>An error occurred on the server side.</p>
    InternalServiceError(crate::types::error::InternalServiceError),
    /// <p>The parameter name or value is invalid.</p>
    InvalidParameterException(crate::types::error::InvalidParameterException),
    /// <p>A parameter value is not valid for the current state of the resource.</p>
    /// <p>Possible causes:</p>
    /// <ul>
    /// <li> <p>The secret is scheduled for deletion.</p> </li>
    /// <li> <p>You tried to enable rotation on a secret that doesn't already have a Lambda function ARN configured and you didn't include such an ARN as a parameter in this call. </p> </li>
    /// <li> <p>The secret is managed by another service, and you must use that service to update it. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/service-linked-secrets.html">Secrets managed by other Amazon Web Services services</a>.</p> </li>
    /// </ul>
    InvalidRequestException(crate::types::error::InvalidRequestException),
    /// <p>Secrets Manager can't find the resource that you asked for.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-RemoveRegionsFromReplicationError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl RemoveRegionsFromReplicationError {
    /// Creates the `RemoveRegionsFromReplicationError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `RemoveRegionsFromReplicationError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.clone().into(),
            meta: err,
        })
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InternalServiceError(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidParameterException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidRequestException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ResourceNotFoundException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `RemoveRegionsFromReplicationError::InternalServiceError`.
    pub fn is_internal_service_error(&self) -> bool {
        matches!(self, Self::InternalServiceError(_))
    }
    /// Returns `true` if the error kind is `RemoveRegionsFromReplicationError::InvalidParameterException`.
    pub fn is_invalid_parameter_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterException(_))
    }
    /// Returns `true` if the error kind is `RemoveRegionsFromReplicationError::InvalidRequestException`.
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(self, Self::InvalidRequestException(_))
    }
    /// Returns `true` if the error kind is `RemoveRegionsFromReplicationError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
}
impl ::std::error::Error for RemoveRegionsFromReplicationError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::InternalServiceError(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidParameterException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidRequestException(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for RemoveRegionsFromReplicationError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::InternalServiceError(_inner) => _inner.fmt(f),
            Self::InvalidParameterException(_inner) => _inner.fmt(f),
            Self::InvalidRequestException(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            }
        }
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for RemoveRegionsFromReplicationError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for RemoveRegionsFromReplicationError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InternalServiceError(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidParameterException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ResourceNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for RemoveRegionsFromReplicationError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source,
            meta: meta.unwrap_or_default(),
        })
    }
}
impl ::aws_types::request_id::RequestId for crate::operation::remove_regions_from_replication::RemoveRegionsFromReplicationError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::remove_regions_from_replication::_remove_regions_from_replication_output::RemoveRegionsFromReplicationOutput;

pub use crate::operation::remove_regions_from_replication::_remove_regions_from_replication_input::RemoveRegionsFromReplicationInput;

mod _remove_regions_from_replication_input;

mod _remove_regions_from_replication_output;

/// Builders
pub mod builders;
