// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_stream_encryption_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::stop_stream_encryption::StopStreamEncryptionInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.stream_name {
        object.key("StreamName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.encryption_type {
        object.key("EncryptionType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.key_id {
        object.key("KeyId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.stream_arn {
        object.key("StreamARN").string(var_4.as_str());
    }
    Ok(())
}
