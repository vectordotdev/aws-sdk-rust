// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_accept_eulas_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AcceptEulasInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.eula_ids {
        let mut array_2 = object.key("eulaIds").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_launch_profile_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLaunchProfileInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_4) = &input.description {
        object.key("description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.ec2_subnet_ids {
        let mut array_6 = object.key("ec2SubnetIds").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.launch_profile_protocol_versions {
        let mut array_9 = object.key("launchProfileProtocolVersions").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.name {
        object.key("name").string(var_11.as_str());
    }
    if let Some(var_12) = &input.stream_configuration {
        let mut object_13 = object.key("streamConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_configuration_create(
            &mut object_13,
            var_12,
        )?;
        object_13.finish();
    }
    if let Some(var_14) = &input.studio_component_ids {
        let mut array_15 = object.key("studioComponentIds").start_array();
        for item_16 in var_14 {
            {
                array_15.value().string(item_16.as_str());
            }
        }
        array_15.finish();
    }
    if let Some(var_17) = &input.tags {
        let mut object_18 = object.key("tags").start_object();
        for (key_19, value_20) in var_17 {
            {
                object_18.key(key_19.as_str()).string(value_20.as_str());
            }
        }
        object_18.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_streaming_image_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStreamingImageInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_21) = &input.description {
        object.key("description").string(var_21.as_str());
    }
    if let Some(var_22) = &input.ec2_image_id {
        object.key("ec2ImageId").string(var_22.as_str());
    }
    if let Some(var_23) = &input.name {
        object.key("name").string(var_23.as_str());
    }
    if let Some(var_24) = &input.tags {
        let mut object_25 = object.key("tags").start_object();
        for (key_26, value_27) in var_24 {
            {
                object_25.key(key_26.as_str()).string(value_27.as_str());
            }
        }
        object_25.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_streaming_session_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStreamingSessionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.ec2_instance_type {
        object.key("ec2InstanceType").string(var_28.as_str());
    }
    if let Some(var_29) = &input.launch_profile_id {
        object.key("launchProfileId").string(var_29.as_str());
    }
    if let Some(var_30) = &input.owned_by {
        object.key("ownedBy").string(var_30.as_str());
    }
    if let Some(var_31) = &input.streaming_image_id {
        object.key("streamingImageId").string(var_31.as_str());
    }
    if let Some(var_32) = &input.tags {
        let mut object_33 = object.key("tags").start_object();
        for (key_34, value_35) in var_32 {
            {
                object_33.key(key_34.as_str()).string(value_35.as_str());
            }
        }
        object_33.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_streaming_session_stream_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStreamingSessionStreamInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.expiration_in_seconds {
        object.key("expirationInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_36).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_studio_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStudioInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_37) = &input.admin_role_arn {
        object.key("adminRoleArn").string(var_37.as_str());
    }
    if let Some(var_38) = &input.display_name {
        object.key("displayName").string(var_38.as_str());
    }
    if let Some(var_39) = &input.studio_encryption_configuration {
        let mut object_40 = object.key("studioEncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_studio_encryption_configuration(
            &mut object_40,
            var_39,
        )?;
        object_40.finish();
    }
    if let Some(var_41) = &input.studio_name {
        object.key("studioName").string(var_41.as_str());
    }
    if let Some(var_42) = &input.tags {
        let mut object_43 = object.key("tags").start_object();
        for (key_44, value_45) in var_42 {
            {
                object_43.key(key_44.as_str()).string(value_45.as_str());
            }
        }
        object_43.finish();
    }
    if let Some(var_46) = &input.user_role_arn {
        object.key("userRoleArn").string(var_46.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_studio_component_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStudioComponentInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_47) = &input.configuration {
        let mut object_48 = object.key("configuration").start_object();
        crate::json_ser::serialize_union_crate_model_studio_component_configuration(
            &mut object_48,
            var_47,
        )?;
        object_48.finish();
    }
    if let Some(var_49) = &input.description {
        object.key("description").string(var_49.as_str());
    }
    if let Some(var_50) = &input.ec2_security_group_ids {
        let mut array_51 = object.key("ec2SecurityGroupIds").start_array();
        for item_52 in var_50 {
            {
                array_51.value().string(item_52.as_str());
            }
        }
        array_51.finish();
    }
    if let Some(var_53) = &input.initialization_scripts {
        let mut array_54 = object.key("initializationScripts").start_array();
        for item_55 in var_53 {
            {
                let mut object_56 = array_54.value().start_object();
                crate::json_ser::serialize_structure_crate_model_studio_component_initialization_script(&mut object_56, item_55)?;
                object_56.finish();
            }
        }
        array_54.finish();
    }
    if let Some(var_57) = &input.name {
        object.key("name").string(var_57.as_str());
    }
    if let Some(var_58) = &input.runtime_role_arn {
        object.key("runtimeRoleArn").string(var_58.as_str());
    }
    if let Some(var_59) = &input.script_parameters {
        let mut array_60 = object.key("scriptParameters").start_array();
        for item_61 in var_59 {
            {
                let mut object_62 = array_60.value().start_object();
                crate::json_ser::serialize_structure_crate_model_script_parameter_key_value(
                    &mut object_62,
                    item_61,
                )?;
                object_62.finish();
            }
        }
        array_60.finish();
    }
    if let Some(var_63) = &input.secure_initialization_role_arn {
        object
            .key("secureInitializationRoleArn")
            .string(var_63.as_str());
    }
    if let Some(var_64) = &input.subtype {
        object.key("subtype").string(var_64.as_str());
    }
    if let Some(var_65) = &input.tags {
        let mut object_66 = object.key("tags").start_object();
        for (key_67, value_68) in var_65 {
            {
                object_66.key(key_67.as_str()).string(value_68.as_str());
            }
        }
        object_66.finish();
    }
    if let Some(var_69) = &input.r#type {
        object.key("type").string(var_69.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_launch_profile_members_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutLaunchProfileMembersInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.identity_store_id {
        object.key("identityStoreId").string(var_70.as_str());
    }
    if let Some(var_71) = &input.members {
        let mut array_72 = object.key("members").start_array();
        for item_73 in var_71 {
            {
                let mut object_74 = array_72.value().start_object();
                crate::json_ser::serialize_structure_crate_model_new_launch_profile_member(
                    &mut object_74,
                    item_73,
                )?;
                object_74.finish();
            }
        }
        array_72.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_studio_members_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutStudioMembersInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_75) = &input.identity_store_id {
        object.key("identityStoreId").string(var_75.as_str());
    }
    if let Some(var_76) = &input.members {
        let mut array_77 = object.key("members").start_array();
        for item_78 in var_76 {
            {
                let mut object_79 = array_77.value().start_object();
                crate::json_ser::serialize_structure_crate_model_new_studio_member(
                    &mut object_79,
                    item_78,
                )?;
                object_79.finish();
            }
        }
        array_77.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_80) = &input.tags {
        let mut object_81 = object.key("tags").start_object();
        for (key_82, value_83) in var_80 {
            {
                object_81.key(key_82.as_str()).string(value_83.as_str());
            }
        }
        object_81.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_launch_profile_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLaunchProfileInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_84) = &input.description {
        object.key("description").string(var_84.as_str());
    }
    if let Some(var_85) = &input.launch_profile_protocol_versions {
        let mut array_86 = object.key("launchProfileProtocolVersions").start_array();
        for item_87 in var_85 {
            {
                array_86.value().string(item_87.as_str());
            }
        }
        array_86.finish();
    }
    if let Some(var_88) = &input.name {
        object.key("name").string(var_88.as_str());
    }
    if let Some(var_89) = &input.stream_configuration {
        let mut object_90 = object.key("streamConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_configuration_create(
            &mut object_90,
            var_89,
        )?;
        object_90.finish();
    }
    if let Some(var_91) = &input.studio_component_ids {
        let mut array_92 = object.key("studioComponentIds").start_array();
        for item_93 in var_91 {
            {
                array_92.value().string(item_93.as_str());
            }
        }
        array_92.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_launch_profile_member_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLaunchProfileMemberInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_94) = &input.persona {
        object.key("persona").string(var_94.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_streaming_image_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateStreamingImageInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_95) = &input.description {
        object.key("description").string(var_95.as_str());
    }
    if let Some(var_96) = &input.name {
        object.key("name").string(var_96.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_studio_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateStudioInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_97) = &input.admin_role_arn {
        object.key("adminRoleArn").string(var_97.as_str());
    }
    if let Some(var_98) = &input.display_name {
        object.key("displayName").string(var_98.as_str());
    }
    if let Some(var_99) = &input.user_role_arn {
        object.key("userRoleArn").string(var_99.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_studio_component_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateStudioComponentInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_100) = &input.configuration {
        let mut object_101 = object.key("configuration").start_object();
        crate::json_ser::serialize_union_crate_model_studio_component_configuration(
            &mut object_101,
            var_100,
        )?;
        object_101.finish();
    }
    if let Some(var_102) = &input.description {
        object.key("description").string(var_102.as_str());
    }
    if let Some(var_103) = &input.ec2_security_group_ids {
        let mut array_104 = object.key("ec2SecurityGroupIds").start_array();
        for item_105 in var_103 {
            {
                array_104.value().string(item_105.as_str());
            }
        }
        array_104.finish();
    }
    if let Some(var_106) = &input.initialization_scripts {
        let mut array_107 = object.key("initializationScripts").start_array();
        for item_108 in var_106 {
            {
                let mut object_109 = array_107.value().start_object();
                crate::json_ser::serialize_structure_crate_model_studio_component_initialization_script(&mut object_109, item_108)?;
                object_109.finish();
            }
        }
        array_107.finish();
    }
    if let Some(var_110) = &input.name {
        object.key("name").string(var_110.as_str());
    }
    if let Some(var_111) = &input.runtime_role_arn {
        object.key("runtimeRoleArn").string(var_111.as_str());
    }
    if let Some(var_112) = &input.script_parameters {
        let mut array_113 = object.key("scriptParameters").start_array();
        for item_114 in var_112 {
            {
                let mut object_115 = array_113.value().start_object();
                crate::json_ser::serialize_structure_crate_model_script_parameter_key_value(
                    &mut object_115,
                    item_114,
                )?;
                object_115.finish();
            }
        }
        array_113.finish();
    }
    if let Some(var_116) = &input.secure_initialization_role_arn {
        object
            .key("secureInitializationRoleArn")
            .string(var_116.as_str());
    }
    if let Some(var_117) = &input.subtype {
        object.key("subtype").string(var_117.as_str());
    }
    if let Some(var_118) = &input.r#type {
        object.key("type").string(var_118.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_stream_configuration_create(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StreamConfigurationCreate,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_119) = &input.clipboard_mode {
        object.key("clipboardMode").string(var_119.as_str());
    }
    if let Some(var_120) = &input.ec2_instance_types {
        let mut array_121 = object.key("ec2InstanceTypes").start_array();
        for item_122 in var_120 {
            {
                array_121.value().string(item_122.as_str());
            }
        }
        array_121.finish();
    }
    if input.max_session_length_in_minutes != 0 {
        object.key("maxSessionLengthInMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_session_length_in_minutes).into()),
        );
    }
    if let Some(var_123) = &input.streaming_image_ids {
        let mut array_124 = object.key("streamingImageIds").start_array();
        for item_125 in var_123 {
            {
                array_124.value().string(item_125.as_str());
            }
        }
        array_124.finish();
    }
    if input.max_stopped_session_length_in_minutes != 0 {
        object.key("maxStoppedSessionLengthInMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_stopped_session_length_in_minutes).into()),
        );
    }
    if let Some(var_126) = &input.session_storage {
        let mut object_127 = object.key("sessionStorage").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_configuration_session_storage(
            &mut object_127,
            var_126,
        )?;
        object_127.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_studio_encryption_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StudioEncryptionConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_128) = &input.key_arn {
        object.key("keyArn").string(var_128.as_str());
    }
    if let Some(var_129) = &input.key_type {
        object.key("keyType").string(var_129.as_str());
    }
    Ok(())
}

pub fn serialize_union_crate_model_studio_component_configuration(
    object_48: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StudioComponentConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::StudioComponentConfiguration::ActiveDirectoryConfiguration(inner) => {
            let mut object_130 = object_48.key("activeDirectoryConfiguration").start_object();
            crate::json_ser::serialize_structure_crate_model_active_directory_configuration(
                &mut object_130,
                inner,
            )?;
            object_130.finish();
        }
        crate::model::StudioComponentConfiguration::ComputeFarmConfiguration(inner) => {
            let mut object_131 = object_48.key("computeFarmConfiguration").start_object();
            crate::json_ser::serialize_structure_crate_model_compute_farm_configuration(
                &mut object_131,
                inner,
            )?;
            object_131.finish();
        }
        crate::model::StudioComponentConfiguration::LicenseServiceConfiguration(inner) => {
            let mut object_132 = object_48.key("licenseServiceConfiguration").start_object();
            crate::json_ser::serialize_structure_crate_model_license_service_configuration(
                &mut object_132,
                inner,
            )?;
            object_132.finish();
        }
        crate::model::StudioComponentConfiguration::SharedFileSystemConfiguration(inner) => {
            let mut object_133 = object_48
                .key("sharedFileSystemConfiguration")
                .start_object();
            crate::json_ser::serialize_structure_crate_model_shared_file_system_configuration(
                &mut object_133,
                inner,
            )?;
            object_133.finish();
        }
        crate::model::StudioComponentConfiguration::Unknown => {
            return Err(
                aws_smithy_http::operation::error::SerializationError::unknown_variant(
                    "StudioComponentConfiguration",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_studio_component_initialization_script(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StudioComponentInitializationScript,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_134) = &input.launch_profile_protocol_version {
        object
            .key("launchProfileProtocolVersion")
            .string(var_134.as_str());
    }
    if let Some(var_135) = &input.platform {
        object.key("platform").string(var_135.as_str());
    }
    if let Some(var_136) = &input.run_context {
        object.key("runContext").string(var_136.as_str());
    }
    if let Some(var_137) = &input.script {
        object.key("script").string(var_137.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_script_parameter_key_value(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ScriptParameterKeyValue,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_138) = &input.key {
        object.key("key").string(var_138.as_str());
    }
    if let Some(var_139) = &input.value {
        object.key("value").string(var_139.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_new_launch_profile_member(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NewLaunchProfileMember,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_140) = &input.persona {
        object.key("persona").string(var_140.as_str());
    }
    if let Some(var_141) = &input.principal_id {
        object.key("principalId").string(var_141.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_new_studio_member(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NewStudioMember,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_142) = &input.persona {
        object.key("persona").string(var_142.as_str());
    }
    if let Some(var_143) = &input.principal_id {
        object.key("principalId").string(var_143.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_stream_configuration_session_storage(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StreamConfigurationSessionStorage,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_144) = &input.root {
        let mut object_145 = object.key("root").start_object();
        crate::json_ser::serialize_structure_crate_model_streaming_session_storage_root(
            &mut object_145,
            var_144,
        )?;
        object_145.finish();
    }
    if let Some(var_146) = &input.mode {
        let mut array_147 = object.key("mode").start_array();
        for item_148 in var_146 {
            {
                array_147.value().string(item_148.as_str());
            }
        }
        array_147.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_active_directory_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ActiveDirectoryConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_149) = &input.computer_attributes {
        let mut array_150 = object.key("computerAttributes").start_array();
        for item_151 in var_149 {
            {
                let mut object_152 = array_150.value().start_object();
                crate::json_ser::serialize_structure_crate_model_active_directory_computer_attribute(&mut object_152, item_151)?;
                object_152.finish();
            }
        }
        array_150.finish();
    }
    if let Some(var_153) = &input.directory_id {
        object.key("directoryId").string(var_153.as_str());
    }
    if let Some(var_154) = &input.organizational_unit_distinguished_name {
        object
            .key("organizationalUnitDistinguishedName")
            .string(var_154.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_compute_farm_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComputeFarmConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_155) = &input.active_directory_user {
        object.key("activeDirectoryUser").string(var_155.as_str());
    }
    if let Some(var_156) = &input.endpoint {
        object.key("endpoint").string(var_156.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_license_service_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LicenseServiceConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_157) = &input.endpoint {
        object.key("endpoint").string(var_157.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_shared_file_system_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SharedFileSystemConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_158) = &input.endpoint {
        object.key("endpoint").string(var_158.as_str());
    }
    if let Some(var_159) = &input.file_system_id {
        object.key("fileSystemId").string(var_159.as_str());
    }
    if let Some(var_160) = &input.linux_mount_point {
        object.key("linuxMountPoint").string(var_160.as_str());
    }
    if let Some(var_161) = &input.share_name {
        object.key("shareName").string(var_161.as_str());
    }
    if let Some(var_162) = &input.windows_mount_drive {
        object.key("windowsMountDrive").string(var_162.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_streaming_session_storage_root(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StreamingSessionStorageRoot,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_163) = &input.linux {
        object.key("linux").string(var_163.as_str());
    }
    if let Some(var_164) = &input.windows {
        object.key("windows").string(var_164.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_active_directory_computer_attribute(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ActiveDirectoryComputerAttribute,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_165) = &input.name {
        object.key("name").string(var_165.as_str());
    }
    if let Some(var_166) = &input.value {
        object.key("value").string(var_166.as_str());
    }
    Ok(())
}
