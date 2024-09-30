// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_replica_region_type(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ReplicaRegionType,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.region {
        object.key("Region").string(var_1.as_str());
    }
    if let Some(var_2) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_2.as_str());
    }
    Ok(())
}
