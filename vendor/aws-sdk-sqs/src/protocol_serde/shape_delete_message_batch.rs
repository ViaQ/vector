// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_message_batch_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_message_batch::DeleteMessageBatchOutput,
    crate::operation::delete_message_batch::DeleteMessageBatchError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_message_batch::DeleteMessageBatchError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AWS.SimpleQueueService.BatchEntryIdsNotDistinct" => {
            crate::operation::delete_message_batch::DeleteMessageBatchError::BatchEntryIdsNotDistinct({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::BatchEntryIdsNotDistinctBuilder::default();
                    output =
                        crate::protocol_serde::shape_batch_entry_ids_not_distinct::de_batch_entry_ids_not_distinct_json_err(_response_body, output)
                            .map_err(crate::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "AWS.SimpleQueueService.EmptyBatchRequest" => crate::operation::delete_message_batch::DeleteMessageBatchError::EmptyBatchRequest({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EmptyBatchRequestBuilder::default();
                output = crate::protocol_serde::shape_empty_batch_request::de_empty_batch_request_json_err(_response_body, output)
                    .map_err(crate::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidAddress" => crate::operation::delete_message_batch::DeleteMessageBatchError::InvalidAddress({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidAddressBuilder::default();
                output = crate::protocol_serde::shape_invalid_address::de_invalid_address_json_err(_response_body, output)
                    .map_err(crate::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "AWS.SimpleQueueService.InvalidBatchEntryId" => crate::operation::delete_message_batch::DeleteMessageBatchError::InvalidBatchEntryId({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidBatchEntryIdBuilder::default();
                output = crate::protocol_serde::shape_invalid_batch_entry_id::de_invalid_batch_entry_id_json_err(_response_body, output)
                    .map_err(crate::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidSecurity" => crate::operation::delete_message_batch::DeleteMessageBatchError::InvalidSecurity({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidSecurityBuilder::default();
                output = crate::protocol_serde::shape_invalid_security::de_invalid_security_json_err(_response_body, output)
                    .map_err(crate::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "AWS.SimpleQueueService.NonExistentQueue" => crate::operation::delete_message_batch::DeleteMessageBatchError::QueueDoesNotExist({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::QueueDoesNotExistBuilder::default();
                output = crate::protocol_serde::shape_queue_does_not_exist::de_queue_does_not_exist_json_err(_response_body, output)
                    .map_err(crate::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RequestThrottled" => crate::operation::delete_message_batch::DeleteMessageBatchError::RequestThrottled({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RequestThrottledBuilder::default();
                output = crate::protocol_serde::shape_request_throttled::de_request_throttled_json_err(_response_body, output)
                    .map_err(crate::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "AWS.SimpleQueueService.TooManyEntriesInBatchRequest" => {
            crate::operation::delete_message_batch::DeleteMessageBatchError::TooManyEntriesInBatchRequest({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyEntriesInBatchRequestBuilder::default();
                    output = crate::protocol_serde::shape_too_many_entries_in_batch_request::de_too_many_entries_in_batch_request_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "AWS.SimpleQueueService.UnsupportedOperation" => crate::operation::delete_message_batch::DeleteMessageBatchError::UnsupportedOperation({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnsupportedOperationBuilder::default();
                output = crate::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(_response_body, output)
                    .map_err(crate::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_message_batch::DeleteMessageBatchError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_message_batch_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_message_batch::DeleteMessageBatchOutput,
    crate::operation::delete_message_batch::DeleteMessageBatchError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_message_batch::builders::DeleteMessageBatchOutputBuilder::default();
        output = crate::protocol_serde::shape_delete_message_batch::de_delete_message_batch(_response_body, output)
            .map_err(crate::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::delete_message_batch_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::delete_message_batch::DeleteMessageBatchError::unhandled)?
    })
}

pub fn ser_delete_message_batch_input(
    input: &crate::operation::delete_message_batch::DeleteMessageBatchInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_message_batch_input::ser_delete_message_batch_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_delete_message_batch(
    value: &[u8],
    mut builder: crate::operation::delete_message_batch::builders::DeleteMessageBatchOutputBuilder,
) -> Result<crate::operation::delete_message_batch::builders::DeleteMessageBatchOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
{
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "Successful" => {
                    builder = builder.set_successful(
                        crate::protocol_serde::shape_delete_message_batch_result_entry_list::de_delete_message_batch_result_entry_list(tokens)?,
                    );
                }
                "Failed" => {
                    builder =
                        builder.set_failed(crate::protocol_serde::shape_batch_result_error_entry_list::de_batch_result_error_entry_list(tokens)?);
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
