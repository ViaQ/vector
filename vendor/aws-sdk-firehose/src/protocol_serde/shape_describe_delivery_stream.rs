// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_delivery_stream_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_delivery_stream::DescribeDeliveryStreamOutput,
    crate::operation::describe_delivery_stream::DescribeDeliveryStreamError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_delivery_stream::DescribeDeliveryStreamError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::describe_delivery_stream::DescribeDeliveryStreamError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => crate::operation::describe_delivery_stream::DescribeDeliveryStreamError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_delivery_stream::DescribeDeliveryStreamError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::describe_delivery_stream::DescribeDeliveryStreamError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_delivery_stream_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_delivery_stream::DescribeDeliveryStreamOutput,
    crate::operation::describe_delivery_stream::DescribeDeliveryStreamError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_delivery_stream::de_describe_delivery_stream(_response_body, output)
            .map_err(crate::operation::describe_delivery_stream::DescribeDeliveryStreamError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::describe_delivery_stream_output_output_correct_errors(output).build()
    })
}

pub fn ser_describe_delivery_stream_input(
    input: &crate::operation::describe_delivery_stream::DescribeDeliveryStreamInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_delivery_stream_input::ser_describe_delivery_stream_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_describe_delivery_stream(
    value: &[u8],
    mut builder: crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamOutputBuilder,
) -> Result<
    crate::operation::describe_delivery_stream::builders::DescribeDeliveryStreamOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "DeliveryStreamDescription" => {
                    builder = builder.set_delivery_stream_description(
                        crate::protocol_serde::shape_delivery_stream_description::de_delivery_stream_description(tokens)?,
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
