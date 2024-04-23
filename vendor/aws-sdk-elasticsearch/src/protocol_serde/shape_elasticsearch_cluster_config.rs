// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_elasticsearch_cluster_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ElasticsearchClusterConfig,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.instance_type {
        object.key("InstanceType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.instance_count {
        object.key("InstanceCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.dedicated_master_enabled {
        object.key("DedicatedMasterEnabled").boolean(*var_3);
    }
    if let Some(var_4) = &input.zone_awareness_enabled {
        object.key("ZoneAwarenessEnabled").boolean(*var_4);
    }
    if let Some(var_5) = &input.zone_awareness_config {
        #[allow(unused_mut)]
        let mut object_6 = object.key("ZoneAwarenessConfig").start_object();
        crate::protocol_serde::shape_zone_awareness_config::ser_zone_awareness_config(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.dedicated_master_type {
        object.key("DedicatedMasterType").string(var_7.as_str());
    }
    if let Some(var_8) = &input.dedicated_master_count {
        object.key("DedicatedMasterCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.warm_enabled {
        object.key("WarmEnabled").boolean(*var_9);
    }
    if let Some(var_10) = &input.warm_type {
        object.key("WarmType").string(var_10.as_str());
    }
    if let Some(var_11) = &input.warm_count {
        object.key("WarmCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    if let Some(var_12) = &input.cold_storage_options {
        #[allow(unused_mut)]
        let mut object_13 = object.key("ColdStorageOptions").start_object();
        crate::protocol_serde::shape_cold_storage_options::ser_cold_storage_options(&mut object_13, var_12)?;
        object_13.finish();
    }
    Ok(())
}

pub(crate) fn de_elasticsearch_cluster_config<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::ElasticsearchClusterConfig>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ElasticsearchClusterConfigBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "InstanceType" => {
                            builder = builder.set_instance_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EsPartitionInstanceType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "InstanceCount" => {
                            builder = builder.set_instance_count(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "DedicatedMasterEnabled" => {
                            builder =
                                builder.set_dedicated_master_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "ZoneAwarenessEnabled" => {
                            builder = builder.set_zone_awareness_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "ZoneAwarenessConfig" => {
                            builder = builder
                                .set_zone_awareness_config(crate::protocol_serde::shape_zone_awareness_config::de_zone_awareness_config(tokens)?);
                        }
                        "DedicatedMasterType" => {
                            builder = builder.set_dedicated_master_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EsPartitionInstanceType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "DedicatedMasterCount" => {
                            builder = builder.set_dedicated_master_count(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "WarmEnabled" => {
                            builder = builder.set_warm_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "WarmType" => {
                            builder = builder.set_warm_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EsWarmPartitionInstanceType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "WarmCount" => {
                            builder = builder.set_warm_count(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "ColdStorageOptions" => {
                            builder =
                                builder.set_cold_storage_options(crate::protocol_serde::shape_cold_storage_options::de_cold_storage_options(tokens)?);
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
