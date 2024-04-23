// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_saml_options_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SamlOptionsInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.enabled {
        object.key("Enabled").boolean(*var_1);
    }
    if let Some(var_2) = &input.idp {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Idp").start_object();
        crate::protocol_serde::shape_saml_idp::ser_saml_idp(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.master_user_name {
        object.key("MasterUserName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.master_backend_role {
        object.key("MasterBackendRole").string(var_5.as_str());
    }
    if let Some(var_6) = &input.subject_key {
        object.key("SubjectKey").string(var_6.as_str());
    }
    if let Some(var_7) = &input.roles_key {
        object.key("RolesKey").string(var_7.as_str());
    }
    if let Some(var_8) = &input.session_timeout_minutes {
        object.key("SessionTimeoutMinutes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    Ok(())
}
