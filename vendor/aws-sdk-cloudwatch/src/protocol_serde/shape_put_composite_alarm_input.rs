// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_composite_alarm_input_input_input(
    input: &crate::operation::put_composite_alarm::PutCompositeAlarmInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "PutCompositeAlarm", "2010-08-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ActionsEnabled");
    if let Some(var_2) = &input.actions_enabled {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("AlarmActions");
    if let Some(var_4) = &input.alarm_actions {
        let mut list_6 = scope_3.start_list(false, None);
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("AlarmDescription");
    if let Some(var_9) = &input.alarm_description {
        scope_8.string(var_9);
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("AlarmName");
    if let Some(var_11) = &input.alarm_name {
        scope_10.string(var_11);
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("AlarmRule");
    if let Some(var_13) = &input.alarm_rule {
        scope_12.string(var_13);
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("InsufficientDataActions");
    if let Some(var_15) = &input.insufficient_data_actions {
        let mut list_17 = scope_14.start_list(false, None);
        for item_16 in var_15 {
            #[allow(unused_mut)]
            let mut entry_18 = list_17.entry();
            entry_18.string(item_16);
        }
        list_17.finish();
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("OKActions");
    if let Some(var_20) = &input.ok_actions {
        let mut list_22 = scope_19.start_list(false, None);
        for item_21 in var_20 {
            #[allow(unused_mut)]
            let mut entry_23 = list_22.entry();
            entry_23.string(item_21);
        }
        list_22.finish();
    }
    #[allow(unused_mut)]
    let mut scope_24 = writer.prefix("Tags");
    if let Some(var_25) = &input.tags {
        let mut list_27 = scope_24.start_list(false, None);
        for item_26 in var_25 {
            #[allow(unused_mut)]
            let mut entry_28 = list_27.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_28, item_26)?;
        }
        list_27.finish();
    }
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("ActionsSuppressor");
    if let Some(var_30) = &input.actions_suppressor {
        scope_29.string(var_30);
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("ActionsSuppressorWaitPeriod");
    if let Some(var_32) = &input.actions_suppressor_wait_period {
        scope_31.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_32).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("ActionsSuppressorExtensionPeriod");
    if let Some(var_34) = &input.actions_suppressor_extension_period {
        scope_33.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_34).into()),
        );
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
