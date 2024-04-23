// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_accept_inbound_cross_cluster_search_connection_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionOutput,
    crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DisabledOperationException" => crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionError::DisabledOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DisabledOperationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_disabled_operation_exception::de_disabled_operation_exception_json_err(_response_body, output).map_err(crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "LimitExceededException" => crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output).map_err(crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output).map_err(crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        _ => crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_accept_inbound_cross_cluster_search_connection_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionOutput,
    crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::accept_inbound_cross_cluster_search_connection::builders::AcceptInboundCrossClusterSearchConnectionOutputBuilder::default();
        output = crate::protocol_serde::shape_accept_inbound_cross_cluster_search_connection::de_accept_inbound_cross_cluster_search_connection(
            _response_body,
            output,
        )
        .map_err(crate::operation::accept_inbound_cross_cluster_search_connection::AcceptInboundCrossClusterSearchConnectionError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_accept_inbound_cross_cluster_search_connection(
    value: &[u8],
    mut builder: crate::operation::accept_inbound_cross_cluster_search_connection::builders::AcceptInboundCrossClusterSearchConnectionOutputBuilder,
) -> Result<
    crate::operation::accept_inbound_cross_cluster_search_connection::builders::AcceptInboundCrossClusterSearchConnectionOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "CrossClusterSearchConnection" => {
                    builder = builder.set_cross_cluster_search_connection(
                        crate::protocol_serde::shape_inbound_cross_cluster_search_connection::de_inbound_cross_cluster_search_connection(tokens)?,
                    );
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
