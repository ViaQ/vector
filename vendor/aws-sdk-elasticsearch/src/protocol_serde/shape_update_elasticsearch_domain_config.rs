// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_elasticsearch_domain_config_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigOutput,
    crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BaseException" => crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::BaseException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BaseExceptionBuilder::default();
                output = crate::protocol_serde::shape_base_exception::de_base_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalException" => crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::InternalException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidTypeException" => crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::InvalidTypeException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidTypeExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_type_exception::de_invalid_type_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "LimitExceededException" => {
            crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::LimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
                        .map_err(crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceNotFoundException" => {
            crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ValidationException" => crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_elasticsearch_domain_config_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigOutput,
    crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_elasticsearch_domain_config::builders::UpdateElasticsearchDomainConfigOutputBuilder::default();
        output = crate::protocol_serde::shape_update_elasticsearch_domain_config::de_update_elasticsearch_domain_config(_response_body, output)
            .map_err(crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::update_elasticsearch_domain_config_output_output_correct_errors(output).build()
    })
}

pub fn ser_update_elasticsearch_domain_config_input(
    input: &crate::operation::update_elasticsearch_domain_config::UpdateElasticsearchDomainConfigInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_elasticsearch_domain_config_input::ser_update_elasticsearch_domain_config_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_update_elasticsearch_domain_config(
    value: &[u8],
    mut builder: crate::operation::update_elasticsearch_domain_config::builders::UpdateElasticsearchDomainConfigOutputBuilder,
) -> Result<
    crate::operation::update_elasticsearch_domain_config::builders::UpdateElasticsearchDomainConfigOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "DomainConfig" => {
                    builder = builder.set_domain_config(crate::protocol_serde::shape_elasticsearch_domain_config::de_elasticsearch_domain_config(
                        tokens,
                    )?);
                }
                "DryRunResults" => {
                    builder = builder.set_dry_run_results(crate::protocol_serde::shape_dry_run_results::de_dry_run_results(tokens)?);
                }
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
