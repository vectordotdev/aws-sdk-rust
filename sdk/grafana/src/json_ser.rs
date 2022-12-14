// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_workspace_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateWorkspaceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_access_type {
        object.key("accountAccessType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.authentication_providers {
        let mut array_3 = object.key("authenticationProviders").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.client_token {
        object.key("clientToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.organization_role_name {
        object.key("organizationRoleName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.permission_type {
        object.key("permissionType").string(var_7.as_str());
    }
    if let Some(var_8) = &input.stack_set_name {
        object.key("stackSetName").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        let mut object_10 = object.key("tags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    if let Some(var_13) = &input.workspace_data_sources {
        let mut array_14 = object.key("workspaceDataSources").start_array();
        for item_15 in var_13 {
            {
                array_14.value().string(item_15.as_str());
            }
        }
        array_14.finish();
    }
    if let Some(var_16) = &input.workspace_description {
        object.key("workspaceDescription").string(var_16.as_str());
    }
    if let Some(var_17) = &input.workspace_name {
        object.key("workspaceName").string(var_17.as_str());
    }
    if let Some(var_18) = &input.workspace_notification_destinations {
        let mut array_19 = object
            .key("workspaceNotificationDestinations")
            .start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20.as_str());
            }
        }
        array_19.finish();
    }
    if let Some(var_21) = &input.workspace_organizational_units {
        let mut array_22 = object.key("workspaceOrganizationalUnits").start_array();
        for item_23 in var_21 {
            {
                array_22.value().string(item_23.as_str());
            }
        }
        array_22.finish();
    }
    if let Some(var_24) = &input.workspace_role_arn {
        object.key("workspaceRoleArn").string(var_24.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_workspace_api_key_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateWorkspaceApiKeyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_25) = &input.key_name {
        object.key("keyName").string(var_25.as_str());
    }
    if let Some(var_26) = &input.key_role {
        object.key("keyRole").string(var_26.as_str());
    }
    if let Some(var_27) = &input.seconds_to_live {
        object.key("secondsToLive").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_27).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.tags {
        let mut object_29 = object.key("tags").start_object();
        for (key_30, value_31) in var_28 {
            {
                object_29.key(key_30.as_str()).string(value_31.as_str());
            }
        }
        object_29.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_permissions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePermissionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_32) = &input.update_instruction_batch {
        let mut array_33 = object.key("updateInstructionBatch").start_array();
        for item_34 in var_32 {
            {
                let mut object_35 = array_33.value().start_object();
                crate::json_ser::serialize_structure_crate_model_update_instruction(
                    &mut object_35,
                    item_34,
                )?;
                object_35.finish();
            }
        }
        array_33.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_workspace_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateWorkspaceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.account_access_type {
        object.key("accountAccessType").string(var_36.as_str());
    }
    if let Some(var_37) = &input.organization_role_name {
        object.key("organizationRoleName").string(var_37.as_str());
    }
    if let Some(var_38) = &input.permission_type {
        object.key("permissionType").string(var_38.as_str());
    }
    if let Some(var_39) = &input.stack_set_name {
        object.key("stackSetName").string(var_39.as_str());
    }
    if let Some(var_40) = &input.workspace_data_sources {
        let mut array_41 = object.key("workspaceDataSources").start_array();
        for item_42 in var_40 {
            {
                array_41.value().string(item_42.as_str());
            }
        }
        array_41.finish();
    }
    if let Some(var_43) = &input.workspace_description {
        object.key("workspaceDescription").string(var_43.as_str());
    }
    if let Some(var_44) = &input.workspace_name {
        object.key("workspaceName").string(var_44.as_str());
    }
    if let Some(var_45) = &input.workspace_notification_destinations {
        let mut array_46 = object
            .key("workspaceNotificationDestinations")
            .start_array();
        for item_47 in var_45 {
            {
                array_46.value().string(item_47.as_str());
            }
        }
        array_46.finish();
    }
    if let Some(var_48) = &input.workspace_organizational_units {
        let mut array_49 = object.key("workspaceOrganizationalUnits").start_array();
        for item_50 in var_48 {
            {
                array_49.value().string(item_50.as_str());
            }
        }
        array_49.finish();
    }
    if let Some(var_51) = &input.workspace_role_arn {
        object.key("workspaceRoleArn").string(var_51.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_workspace_authentication_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateWorkspaceAuthenticationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_52) = &input.authentication_providers {
        let mut array_53 = object.key("authenticationProviders").start_array();
        for item_54 in var_52 {
            {
                array_53.value().string(item_54.as_str());
            }
        }
        array_53.finish();
    }
    if let Some(var_55) = &input.saml_configuration {
        let mut object_56 = object.key("samlConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_saml_configuration(
            &mut object_56,
            var_55,
        )?;
        object_56.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_update_instruction(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateInstruction,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_57) = &input.action {
        object.key("action").string(var_57.as_str());
    }
    if let Some(var_58) = &input.role {
        object.key("role").string(var_58.as_str());
    }
    if let Some(var_59) = &input.users {
        let mut array_60 = object.key("users").start_array();
        for item_61 in var_59 {
            {
                let mut object_62 = array_60.value().start_object();
                crate::json_ser::serialize_structure_crate_model_user(&mut object_62, item_61)?;
                object_62.finish();
            }
        }
        array_60.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_saml_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SamlConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.idp_metadata {
        let mut object_64 = object.key("idpMetadata").start_object();
        crate::json_ser::serialize_union_crate_model_idp_metadata(&mut object_64, var_63)?;
        object_64.finish();
    }
    if let Some(var_65) = &input.assertion_attributes {
        let mut object_66 = object.key("assertionAttributes").start_object();
        crate::json_ser::serialize_structure_crate_model_assertion_attributes(
            &mut object_66,
            var_65,
        )?;
        object_66.finish();
    }
    if let Some(var_67) = &input.role_values {
        let mut object_68 = object.key("roleValues").start_object();
        crate::json_ser::serialize_structure_crate_model_role_values(&mut object_68, var_67)?;
        object_68.finish();
    }
    if let Some(var_69) = &input.allowed_organizations {
        let mut array_70 = object.key("allowedOrganizations").start_array();
        for item_71 in var_69 {
            {
                array_70.value().string(item_71.as_str());
            }
        }
        array_70.finish();
    }
    if input.login_validity_duration != 0 {
        object.key("loginValidityDuration").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.login_validity_duration).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_user(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::User,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_72) = &input.id {
        object.key("id").string(var_72.as_str());
    }
    if let Some(var_73) = &input.r#type {
        object.key("type").string(var_73.as_str());
    }
    Ok(())
}

pub fn serialize_union_crate_model_idp_metadata(
    object_64: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IdpMetadata,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::IdpMetadata::Url(inner) => {
            object_64.key("url").string(inner.as_str());
        }
        crate::model::IdpMetadata::Xml(inner) => {
            object_64.key("xml").string(inner.as_str());
        }
        crate::model::IdpMetadata::Unknown => {
            return Err(
                aws_smithy_http::operation::error::SerializationError::unknown_variant(
                    "IdpMetadata",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_assertion_attributes(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AssertionAttributes,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_74) = &input.name {
        object.key("name").string(var_74.as_str());
    }
    if let Some(var_75) = &input.login {
        object.key("login").string(var_75.as_str());
    }
    if let Some(var_76) = &input.email {
        object.key("email").string(var_76.as_str());
    }
    if let Some(var_77) = &input.groups {
        object.key("groups").string(var_77.as_str());
    }
    if let Some(var_78) = &input.role {
        object.key("role").string(var_78.as_str());
    }
    if let Some(var_79) = &input.org {
        object.key("org").string(var_79.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_role_values(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RoleValues,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_80) = &input.editor {
        let mut array_81 = object.key("editor").start_array();
        for item_82 in var_80 {
            {
                array_81.value().string(item_82.as_str());
            }
        }
        array_81.finish();
    }
    if let Some(var_83) = &input.admin {
        let mut array_84 = object.key("admin").start_array();
        for item_85 in var_83 {
            {
                array_84.value().string(item_85.as_str());
            }
        }
        array_84.finish();
    }
    Ok(())
}
