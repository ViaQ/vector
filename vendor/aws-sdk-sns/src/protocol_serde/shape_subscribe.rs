// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_subscribe_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::subscribe::SubscribeOutput, crate::operation::subscribe::SubscribeError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::subscribe::SubscribeError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::subscribe::SubscribeError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AuthorizationError" => crate::operation::subscribe::SubscribeError::AuthorizationErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AuthorizationErrorExceptionBuilder::default();
                output = crate::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::subscribe::SubscribeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "FilterPolicyLimitExceeded" => crate::operation::subscribe::SubscribeError::FilterPolicyLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::FilterPolicyLimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_filter_policy_limit_exceeded_exception::de_filter_policy_limit_exceeded_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::subscribe::SubscribeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalError" => crate::operation::subscribe::SubscribeError::InternalErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalErrorExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_error_exception::de_internal_error_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::subscribe::SubscribeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameter" => crate::operation::subscribe::SubscribeError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::subscribe::SubscribeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidSecurity" => crate::operation::subscribe::SubscribeError::InvalidSecurityException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidSecurityExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_security_exception::de_invalid_security_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::subscribe::SubscribeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NotFound" => crate::operation::subscribe::SubscribeError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::subscribe::SubscribeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ReplayLimitExceeded" => crate::operation::subscribe::SubscribeError::ReplayLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ReplayLimitExceededExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_replay_limit_exceeded_exception::de_replay_limit_exceeded_exception_xml_err(_response_body, output)
                        .map_err(crate::operation::subscribe::SubscribeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "SubscriptionLimitExceeded" => crate::operation::subscribe::SubscribeError::SubscriptionLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::SubscriptionLimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_subscription_limit_exceeded_exception::de_subscription_limit_exceeded_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::subscribe::SubscribeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::subscribe::SubscribeError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_subscribe_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::subscribe::SubscribeOutput, crate::operation::subscribe::SubscribeError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::subscribe::builders::SubscribeOutputBuilder::default();
        output = crate::protocol_serde::shape_subscribe::de_subscribe(_response_body, output)
            .map_err(crate::operation::subscribe::SubscribeError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_subscribe(
    inp: &[u8],
    mut builder: crate::operation::subscribe::builders::SubscribeOutputBuilder,
) -> Result<crate::operation::subscribe::builders::SubscribeOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("SubscribeResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected SubscribeResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("SubscribeResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected SubscribeResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("SubscriptionArn") /* SubscriptionArn com.amazonaws.sns.synthetic#SubscribeOutput$SubscriptionArn */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subscription_arn(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected SubscribeResult tag"));
    };
    Ok(builder)
}
