// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_amazonopensearchservice_destination_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AmazonopensearchserviceDestinationConfiguration,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("RoleARN").string(input.role_arn.as_str());
    }
    if let Some(var_1) = &input.domain_arn {
        object.key("DomainARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.cluster_endpoint {
        object.key("ClusterEndpoint").string(var_2.as_str());
    }
    {
        object.key("IndexName").string(input.index_name.as_str());
    }
    if let Some(var_3) = &input.type_name {
        object.key("TypeName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.index_rotation_period {
        object.key("IndexRotationPeriod").string(var_4.as_str());
    }
    if let Some(var_5) = &input.buffering_hints {
        #[allow(unused_mut)]
        let mut object_6 = object.key("BufferingHints").start_object();
        crate::protocol_serde::shape_amazonopensearchservice_buffering_hints::ser_amazonopensearchservice_buffering_hints(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.retry_options {
        #[allow(unused_mut)]
        let mut object_8 = object.key("RetryOptions").start_object();
        crate::protocol_serde::shape_amazonopensearchservice_retry_options::ser_amazonopensearchservice_retry_options(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.s3_backup_mode {
        object.key("S3BackupMode").string(var_9.as_str());
    }
    if let Some(var_10) = &input.s3_configuration {
        #[allow(unused_mut)]
        let mut object_11 = object.key("S3Configuration").start_object();
        crate::protocol_serde::shape_s3_destination_configuration::ser_s3_destination_configuration(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.processing_configuration {
        #[allow(unused_mut)]
        let mut object_13 = object.key("ProcessingConfiguration").start_object();
        crate::protocol_serde::shape_processing_configuration::ser_processing_configuration(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.cloud_watch_logging_options {
        #[allow(unused_mut)]
        let mut object_15 = object.key("CloudWatchLoggingOptions").start_object();
        crate::protocol_serde::shape_cloud_watch_logging_options::ser_cloud_watch_logging_options(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.vpc_configuration {
        #[allow(unused_mut)]
        let mut object_17 = object.key("VpcConfiguration").start_object();
        crate::protocol_serde::shape_vpc_configuration::ser_vpc_configuration(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.document_id_options {
        #[allow(unused_mut)]
        let mut object_19 = object.key("DocumentIdOptions").start_object();
        crate::protocol_serde::shape_document_id_options::ser_document_id_options(&mut object_19, var_18)?;
        object_19.finish();
    }
    Ok(())
}
