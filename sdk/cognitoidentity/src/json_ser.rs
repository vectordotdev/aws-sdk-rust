// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_identity_pool_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateIdentityPoolInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.identity_pool_name {
        object.key("IdentityPoolName").string(var_1.as_str());
    }
    {
        object
            .key("AllowUnauthenticatedIdentities")
            .boolean(input.allow_unauthenticated_identities);
    }
    if let Some(var_2) = &input.allow_classic_flow {
        object.key("AllowClassicFlow").boolean(*var_2);
    }
    if let Some(var_3) = &input.supported_login_providers {
        let mut object_4 = object.key("SupportedLoginProviders").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    if let Some(var_7) = &input.developer_provider_name {
        object.key("DeveloperProviderName").string(var_7.as_str());
    }
    if let Some(var_8) = &input.open_id_connect_provider_ar_ns {
        let mut array_9 = object.key("OpenIdConnectProviderARNs").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.cognito_identity_providers {
        let mut array_12 = object.key("CognitoIdentityProviders").start_array();
        for item_13 in var_11 {
            {
                let mut object_14 = array_12.value().start_object();
                crate::json_ser::serialize_structure_crate_model_cognito_identity_provider(
                    &mut object_14,
                    item_13,
                )?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.saml_provider_ar_ns {
        let mut array_16 = object.key("SamlProviderARNs").start_array();
        for item_17 in var_15 {
            {
                array_16.value().string(item_17.as_str());
            }
        }
        array_16.finish();
    }
    if let Some(var_18) = &input.identity_pool_tags {
        let mut object_19 = object.key("IdentityPoolTags").start_object();
        for (key_20, value_21) in var_18 {
            {
                object_19.key(key_20.as_str()).string(value_21.as_str());
            }
        }
        object_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_identities_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteIdentitiesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.identity_ids_to_delete {
        let mut array_23 = object.key("IdentityIdsToDelete").start_array();
        for item_24 in var_22 {
            {
                array_23.value().string(item_24.as_str());
            }
        }
        array_23.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_identity_pool_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteIdentityPoolInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_25) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_identity_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeIdentityInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.identity_id {
        object.key("IdentityId").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_identity_pool_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeIdentityPoolInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_27) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_credentials_for_identity_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetCredentialsForIdentityInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.identity_id {
        object.key("IdentityId").string(var_28.as_str());
    }
    if let Some(var_29) = &input.logins {
        let mut object_30 = object.key("Logins").start_object();
        for (key_31, value_32) in var_29 {
            {
                object_30.key(key_31.as_str()).string(value_32.as_str());
            }
        }
        object_30.finish();
    }
    if let Some(var_33) = &input.custom_role_arn {
        object.key("CustomRoleArn").string(var_33.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_id_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetIdInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_34) = &input.account_id {
        object.key("AccountId").string(var_34.as_str());
    }
    if let Some(var_35) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_35.as_str());
    }
    if let Some(var_36) = &input.logins {
        let mut object_37 = object.key("Logins").start_object();
        for (key_38, value_39) in var_36 {
            {
                object_37.key(key_38.as_str()).string(value_39.as_str());
            }
        }
        object_37.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_identity_pool_roles_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetIdentityPoolRolesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_40) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_40.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_open_id_token_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetOpenIdTokenInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_41) = &input.identity_id {
        object.key("IdentityId").string(var_41.as_str());
    }
    if let Some(var_42) = &input.logins {
        let mut object_43 = object.key("Logins").start_object();
        for (key_44, value_45) in var_42 {
            {
                object_43.key(key_44.as_str()).string(value_45.as_str());
            }
        }
        object_43.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_open_id_token_for_developer_identity_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetOpenIdTokenForDeveloperIdentityInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_46) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_46.as_str());
    }
    if let Some(var_47) = &input.identity_id {
        object.key("IdentityId").string(var_47.as_str());
    }
    if let Some(var_48) = &input.logins {
        let mut object_49 = object.key("Logins").start_object();
        for (key_50, value_51) in var_48 {
            {
                object_49.key(key_50.as_str()).string(value_51.as_str());
            }
        }
        object_49.finish();
    }
    if let Some(var_52) = &input.principal_tags {
        let mut object_53 = object.key("PrincipalTags").start_object();
        for (key_54, value_55) in var_52 {
            {
                object_53.key(key_54.as_str()).string(value_55.as_str());
            }
        }
        object_53.finish();
    }
    if let Some(var_56) = &input.token_duration {
        object.key("TokenDuration").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_56).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_principal_tag_attribute_map_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetPrincipalTagAttributeMapInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_57) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_57.as_str());
    }
    if let Some(var_58) = &input.identity_provider_name {
        object.key("IdentityProviderName").string(var_58.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_identities_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListIdentitiesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_59) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_59.as_str());
    }
    {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_60) = &input.next_token {
        object.key("NextToken").string(var_60.as_str());
    }
    if input.hide_disabled {
        object.key("HideDisabled").boolean(input.hide_disabled);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_identity_pools_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListIdentityPoolsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_61) = &input.next_token {
        object.key("NextToken").string(var_61.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_62) = &input.resource_arn {
        object.key("ResourceArn").string(var_62.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_lookup_developer_identity_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::LookupDeveloperIdentityInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_63.as_str());
    }
    if let Some(var_64) = &input.identity_id {
        object.key("IdentityId").string(var_64.as_str());
    }
    if let Some(var_65) = &input.developer_user_identifier {
        object
            .key("DeveloperUserIdentifier")
            .string(var_65.as_str());
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_66) = &input.next_token {
        object.key("NextToken").string(var_66.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_merge_developer_identities_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::MergeDeveloperIdentitiesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_67) = &input.source_user_identifier {
        object.key("SourceUserIdentifier").string(var_67.as_str());
    }
    if let Some(var_68) = &input.destination_user_identifier {
        object
            .key("DestinationUserIdentifier")
            .string(var_68.as_str());
    }
    if let Some(var_69) = &input.developer_provider_name {
        object.key("DeveloperProviderName").string(var_69.as_str());
    }
    if let Some(var_70) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_70.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_set_identity_pool_roles_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SetIdentityPoolRolesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_71) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_71.as_str());
    }
    if let Some(var_72) = &input.roles {
        let mut object_73 = object.key("Roles").start_object();
        for (key_74, value_75) in var_72 {
            {
                object_73.key(key_74.as_str()).string(value_75.as_str());
            }
        }
        object_73.finish();
    }
    if let Some(var_76) = &input.role_mappings {
        let mut object_77 = object.key("RoleMappings").start_object();
        for (key_78, value_79) in var_76 {
            {
                let mut object_80 = object_77.key(key_78.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_role_mapping(
                    &mut object_80,
                    value_79,
                )?;
                object_80.finish();
            }
        }
        object_77.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_set_principal_tag_attribute_map_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SetPrincipalTagAttributeMapInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_81) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_81.as_str());
    }
    if let Some(var_82) = &input.identity_provider_name {
        object.key("IdentityProviderName").string(var_82.as_str());
    }
    if let Some(var_83) = &input.use_defaults {
        object.key("UseDefaults").boolean(*var_83);
    }
    if let Some(var_84) = &input.principal_tags {
        let mut object_85 = object.key("PrincipalTags").start_object();
        for (key_86, value_87) in var_84 {
            {
                object_85.key(key_86.as_str()).string(value_87.as_str());
            }
        }
        object_85.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_88) = &input.resource_arn {
        object.key("ResourceArn").string(var_88.as_str());
    }
    if let Some(var_89) = &input.tags {
        let mut object_90 = object.key("Tags").start_object();
        for (key_91, value_92) in var_89 {
            {
                object_90.key(key_91.as_str()).string(value_92.as_str());
            }
        }
        object_90.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_unlink_developer_identity_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UnlinkDeveloperIdentityInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_93) = &input.identity_id {
        object.key("IdentityId").string(var_93.as_str());
    }
    if let Some(var_94) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_94.as_str());
    }
    if let Some(var_95) = &input.developer_provider_name {
        object.key("DeveloperProviderName").string(var_95.as_str());
    }
    if let Some(var_96) = &input.developer_user_identifier {
        object
            .key("DeveloperUserIdentifier")
            .string(var_96.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_unlink_identity_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UnlinkIdentityInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_97) = &input.identity_id {
        object.key("IdentityId").string(var_97.as_str());
    }
    if let Some(var_98) = &input.logins {
        let mut object_99 = object.key("Logins").start_object();
        for (key_100, value_101) in var_98 {
            {
                object_99.key(key_100.as_str()).string(value_101.as_str());
            }
        }
        object_99.finish();
    }
    if let Some(var_102) = &input.logins_to_remove {
        let mut array_103 = object.key("LoginsToRemove").start_array();
        for item_104 in var_102 {
            {
                array_103.value().string(item_104.as_str());
            }
        }
        array_103.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_105) = &input.resource_arn {
        object.key("ResourceArn").string(var_105.as_str());
    }
    if let Some(var_106) = &input.tag_keys {
        let mut array_107 = object.key("TagKeys").start_array();
        for item_108 in var_106 {
            {
                array_107.value().string(item_108.as_str());
            }
        }
        array_107.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_identity_pool_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateIdentityPoolInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_109) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_109.as_str());
    }
    if let Some(var_110) = &input.identity_pool_name {
        object.key("IdentityPoolName").string(var_110.as_str());
    }
    {
        object
            .key("AllowUnauthenticatedIdentities")
            .boolean(input.allow_unauthenticated_identities);
    }
    if let Some(var_111) = &input.allow_classic_flow {
        object.key("AllowClassicFlow").boolean(*var_111);
    }
    if let Some(var_112) = &input.supported_login_providers {
        let mut object_113 = object.key("SupportedLoginProviders").start_object();
        for (key_114, value_115) in var_112 {
            {
                object_113.key(key_114.as_str()).string(value_115.as_str());
            }
        }
        object_113.finish();
    }
    if let Some(var_116) = &input.developer_provider_name {
        object.key("DeveloperProviderName").string(var_116.as_str());
    }
    if let Some(var_117) = &input.open_id_connect_provider_ar_ns {
        let mut array_118 = object.key("OpenIdConnectProviderARNs").start_array();
        for item_119 in var_117 {
            {
                array_118.value().string(item_119.as_str());
            }
        }
        array_118.finish();
    }
    if let Some(var_120) = &input.cognito_identity_providers {
        let mut array_121 = object.key("CognitoIdentityProviders").start_array();
        for item_122 in var_120 {
            {
                let mut object_123 = array_121.value().start_object();
                crate::json_ser::serialize_structure_crate_model_cognito_identity_provider(
                    &mut object_123,
                    item_122,
                )?;
                object_123.finish();
            }
        }
        array_121.finish();
    }
    if let Some(var_124) = &input.saml_provider_ar_ns {
        let mut array_125 = object.key("SamlProviderARNs").start_array();
        for item_126 in var_124 {
            {
                array_125.value().string(item_126.as_str());
            }
        }
        array_125.finish();
    }
    if let Some(var_127) = &input.identity_pool_tags {
        let mut object_128 = object.key("IdentityPoolTags").start_object();
        for (key_129, value_130) in var_127 {
            {
                object_128.key(key_129.as_str()).string(value_130.as_str());
            }
        }
        object_128.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cognito_identity_provider(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CognitoIdentityProvider,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_131) = &input.provider_name {
        object.key("ProviderName").string(var_131.as_str());
    }
    if let Some(var_132) = &input.client_id {
        object.key("ClientId").string(var_132.as_str());
    }
    if let Some(var_133) = &input.server_side_token_check {
        object.key("ServerSideTokenCheck").boolean(*var_133);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_role_mapping(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RoleMapping,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_134) = &input.r#type {
        object.key("Type").string(var_134.as_str());
    }
    if let Some(var_135) = &input.ambiguous_role_resolution {
        object
            .key("AmbiguousRoleResolution")
            .string(var_135.as_str());
    }
    if let Some(var_136) = &input.rules_configuration {
        let mut object_137 = object.key("RulesConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_rules_configuration_type(
            &mut object_137,
            var_136,
        )?;
        object_137.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_rules_configuration_type(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RulesConfigurationType,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_138) = &input.rules {
        let mut array_139 = object.key("Rules").start_array();
        for item_140 in var_138 {
            {
                let mut object_141 = array_139.value().start_object();
                crate::json_ser::serialize_structure_crate_model_mapping_rule(
                    &mut object_141,
                    item_140,
                )?;
                object_141.finish();
            }
        }
        array_139.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_mapping_rule(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MappingRule,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_142) = &input.claim {
        object.key("Claim").string(var_142.as_str());
    }
    if let Some(var_143) = &input.match_type {
        object.key("MatchType").string(var_143.as_str());
    }
    if let Some(var_144) = &input.value {
        object.key("Value").string(var_144.as_str());
    }
    if let Some(var_145) = &input.role_arn {
        object.key("RoleARN").string(var_145.as_str());
    }
    Ok(())
}
