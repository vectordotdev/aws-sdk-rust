// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_lifecycle_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLifecyclePolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.policy_details {
        let mut object_4 = object.key("PolicyDetails").start_object();
        crate::json_ser::serialize_structure_crate_model_policy_details(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.state {
        object.key("State").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        let mut object_7 = object.key("Tags").start_object();
        for (key_8, value_9) in var_6 {
            {
                object_7.key(key_8).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_10) = &input.tags {
        let mut object_11 = object.key("Tags").start_object();
        for (key_12, value_13) in var_10 {
            {
                object_11.key(key_12).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_lifecycle_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLifecyclePolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_14) = &input.description {
        object.key("Description").string(var_14.as_str());
    }
    if let Some(var_15) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_15.as_str());
    }
    if let Some(var_16) = &input.policy_details {
        let mut object_17 = object.key("PolicyDetails").start_object();
        crate::json_ser::serialize_structure_crate_model_policy_details(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.state {
        object.key("State").string(var_18.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_policy_details(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PolicyDetails,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_19) = &input.policy_type {
        object.key("PolicyType").string(var_19.as_str());
    }
    if let Some(var_20) = &input.resource_types {
        let mut array_21 = object.key("ResourceTypes").start_array();
        for item_22 in var_20 {
            {
                array_21.value().string(item_22.as_str());
            }
        }
        array_21.finish();
    }
    if let Some(var_23) = &input.resource_locations {
        let mut array_24 = object.key("ResourceLocations").start_array();
        for item_25 in var_23 {
            {
                array_24.value().string(item_25.as_str());
            }
        }
        array_24.finish();
    }
    if let Some(var_26) = &input.target_tags {
        let mut array_27 = object.key("TargetTags").start_array();
        for item_28 in var_26 {
            {
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_29, item_28)?;
                object_29.finish();
            }
        }
        array_27.finish();
    }
    if let Some(var_30) = &input.schedules {
        let mut array_31 = object.key("Schedules").start_array();
        for item_32 in var_30 {
            {
                let mut object_33 = array_31.value().start_object();
                crate::json_ser::serialize_structure_crate_model_schedule(&mut object_33, item_32)?;
                object_33.finish();
            }
        }
        array_31.finish();
    }
    if let Some(var_34) = &input.parameters {
        let mut object_35 = object.key("Parameters").start_object();
        crate::json_ser::serialize_structure_crate_model_parameters(&mut object_35, var_34)?;
        object_35.finish();
    }
    if let Some(var_36) = &input.event_source {
        let mut object_37 = object.key("EventSource").start_object();
        crate::json_ser::serialize_structure_crate_model_event_source(&mut object_37, var_36)?;
        object_37.finish();
    }
    if let Some(var_38) = &input.actions {
        let mut array_39 = object.key("Actions").start_array();
        for item_40 in var_38 {
            {
                let mut object_41 = array_39.value().start_object();
                crate::json_ser::serialize_structure_crate_model_action(&mut object_41, item_40)?;
                object_41.finish();
            }
        }
        array_39.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_42) = &input.key {
        object.key("Key").string(var_42.as_str());
    }
    if let Some(var_43) = &input.value {
        object.key("Value").string(var_43.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_schedule(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Schedule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.name {
        object.key("Name").string(var_44.as_str());
    }
    if input.copy_tags {
        object.key("CopyTags").boolean(input.copy_tags);
    }
    if let Some(var_45) = &input.tags_to_add {
        let mut array_46 = object.key("TagsToAdd").start_array();
        for item_47 in var_45 {
            {
                let mut object_48 = array_46.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_48, item_47)?;
                object_48.finish();
            }
        }
        array_46.finish();
    }
    if let Some(var_49) = &input.variable_tags {
        let mut array_50 = object.key("VariableTags").start_array();
        for item_51 in var_49 {
            {
                let mut object_52 = array_50.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_52, item_51)?;
                object_52.finish();
            }
        }
        array_50.finish();
    }
    if let Some(var_53) = &input.create_rule {
        let mut object_54 = object.key("CreateRule").start_object();
        crate::json_ser::serialize_structure_crate_model_create_rule(&mut object_54, var_53)?;
        object_54.finish();
    }
    if let Some(var_55) = &input.retain_rule {
        let mut object_56 = object.key("RetainRule").start_object();
        crate::json_ser::serialize_structure_crate_model_retain_rule(&mut object_56, var_55)?;
        object_56.finish();
    }
    if let Some(var_57) = &input.fast_restore_rule {
        let mut object_58 = object.key("FastRestoreRule").start_object();
        crate::json_ser::serialize_structure_crate_model_fast_restore_rule(&mut object_58, var_57)?;
        object_58.finish();
    }
    if let Some(var_59) = &input.cross_region_copy_rules {
        let mut array_60 = object.key("CrossRegionCopyRules").start_array();
        for item_61 in var_59 {
            {
                let mut object_62 = array_60.value().start_object();
                crate::json_ser::serialize_structure_crate_model_cross_region_copy_rule(
                    &mut object_62,
                    item_61,
                )?;
                object_62.finish();
            }
        }
        array_60.finish();
    }
    if let Some(var_63) = &input.share_rules {
        let mut array_64 = object.key("ShareRules").start_array();
        for item_65 in var_63 {
            {
                let mut object_66 = array_64.value().start_object();
                crate::json_ser::serialize_structure_crate_model_share_rule(
                    &mut object_66,
                    item_65,
                )?;
                object_66.finish();
            }
        }
        array_64.finish();
    }
    if let Some(var_67) = &input.deprecate_rule {
        let mut object_68 = object.key("DeprecateRule").start_object();
        crate::json_ser::serialize_structure_crate_model_deprecate_rule(&mut object_68, var_67)?;
        object_68.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_parameters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Parameters,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_69) = &input.exclude_boot_volume {
        object.key("ExcludeBootVolume").boolean(*var_69);
    }
    if let Some(var_70) = &input.no_reboot {
        object.key("NoReboot").boolean(*var_70);
    }
    if let Some(var_71) = &input.exclude_data_volume_tags {
        let mut array_72 = object.key("ExcludeDataVolumeTags").start_array();
        for item_73 in var_71 {
            {
                let mut object_74 = array_72.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_74, item_73)?;
                object_74.finish();
            }
        }
        array_72.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_event_source(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EventSource,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_75) = &input.r#type {
        object.key("Type").string(var_75.as_str());
    }
    if let Some(var_76) = &input.parameters {
        let mut object_77 = object.key("Parameters").start_object();
        crate::json_ser::serialize_structure_crate_model_event_parameters(&mut object_77, var_76)?;
        object_77.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Action,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_78) = &input.name {
        object.key("Name").string(var_78.as_str());
    }
    if let Some(var_79) = &input.cross_region_copy {
        let mut array_80 = object.key("CrossRegionCopy").start_array();
        for item_81 in var_79 {
            {
                let mut object_82 = array_80.value().start_object();
                crate::json_ser::serialize_structure_crate_model_cross_region_copy_action(
                    &mut object_82,
                    item_81,
                )?;
                object_82.finish();
            }
        }
        array_80.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_create_rule(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateRule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_83) = &input.location {
        object.key("Location").string(var_83.as_str());
    }
    if input.interval != 0 {
        object.key("Interval").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.interval).into()),
        );
    }
    if let Some(var_84) = &input.interval_unit {
        object.key("IntervalUnit").string(var_84.as_str());
    }
    if let Some(var_85) = &input.times {
        let mut array_86 = object.key("Times").start_array();
        for item_87 in var_85 {
            {
                array_86.value().string(item_87.as_str());
            }
        }
        array_86.finish();
    }
    if let Some(var_88) = &input.cron_expression {
        object.key("CronExpression").string(var_88.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_retain_rule(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RetainRule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.count != 0 {
        object.key("Count").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.count).into()),
        );
    }
    if input.interval != 0 {
        object.key("Interval").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.interval).into()),
        );
    }
    if let Some(var_89) = &input.interval_unit {
        object.key("IntervalUnit").string(var_89.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_fast_restore_rule(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FastRestoreRule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.count != 0 {
        object.key("Count").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.count).into()),
        );
    }
    if input.interval != 0 {
        object.key("Interval").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.interval).into()),
        );
    }
    if let Some(var_90) = &input.interval_unit {
        object.key("IntervalUnit").string(var_90.as_str());
    }
    if let Some(var_91) = &input.availability_zones {
        let mut array_92 = object.key("AvailabilityZones").start_array();
        for item_93 in var_91 {
            {
                array_92.value().string(item_93.as_str());
            }
        }
        array_92.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cross_region_copy_rule(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CrossRegionCopyRule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_94) = &input.target_region {
        object.key("TargetRegion").string(var_94.as_str());
    }
    if let Some(var_95) = &input.target {
        object.key("Target").string(var_95.as_str());
    }
    if let Some(var_96) = &input.encrypted {
        object.key("Encrypted").boolean(*var_96);
    }
    if let Some(var_97) = &input.cmk_arn {
        object.key("CmkArn").string(var_97.as_str());
    }
    if let Some(var_98) = &input.copy_tags {
        object.key("CopyTags").boolean(*var_98);
    }
    if let Some(var_99) = &input.retain_rule {
        let mut object_100 = object.key("RetainRule").start_object();
        crate::json_ser::serialize_structure_crate_model_cross_region_copy_retain_rule(
            &mut object_100,
            var_99,
        )?;
        object_100.finish();
    }
    if let Some(var_101) = &input.deprecate_rule {
        let mut object_102 = object.key("DeprecateRule").start_object();
        crate::json_ser::serialize_structure_crate_model_cross_region_copy_deprecate_rule(
            &mut object_102,
            var_101,
        )?;
        object_102.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_share_rule(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ShareRule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_103) = &input.target_accounts {
        let mut array_104 = object.key("TargetAccounts").start_array();
        for item_105 in var_103 {
            {
                array_104.value().string(item_105.as_str());
            }
        }
        array_104.finish();
    }
    if input.unshare_interval != 0 {
        object.key("UnshareInterval").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.unshare_interval).into()),
        );
    }
    if let Some(var_106) = &input.unshare_interval_unit {
        object.key("UnshareIntervalUnit").string(var_106.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_deprecate_rule(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeprecateRule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.count != 0 {
        object.key("Count").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.count).into()),
        );
    }
    if input.interval != 0 {
        object.key("Interval").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.interval).into()),
        );
    }
    if let Some(var_107) = &input.interval_unit {
        object.key("IntervalUnit").string(var_107.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_event_parameters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EventParameters,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_108) = &input.event_type {
        object.key("EventType").string(var_108.as_str());
    }
    if let Some(var_109) = &input.snapshot_owner {
        let mut array_110 = object.key("SnapshotOwner").start_array();
        for item_111 in var_109 {
            {
                array_110.value().string(item_111.as_str());
            }
        }
        array_110.finish();
    }
    if let Some(var_112) = &input.description_regex {
        object.key("DescriptionRegex").string(var_112.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cross_region_copy_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CrossRegionCopyAction,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_113) = &input.target {
        object.key("Target").string(var_113.as_str());
    }
    if let Some(var_114) = &input.encryption_configuration {
        let mut object_115 = object.key("EncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_configuration(
            &mut object_115,
            var_114,
        )?;
        object_115.finish();
    }
    if let Some(var_116) = &input.retain_rule {
        let mut object_117 = object.key("RetainRule").start_object();
        crate::json_ser::serialize_structure_crate_model_cross_region_copy_retain_rule(
            &mut object_117,
            var_116,
        )?;
        object_117.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cross_region_copy_retain_rule(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CrossRegionCopyRetainRule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.interval != 0 {
        object.key("Interval").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.interval).into()),
        );
    }
    if let Some(var_118) = &input.interval_unit {
        object.key("IntervalUnit").string(var_118.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cross_region_copy_deprecate_rule(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CrossRegionCopyDeprecateRule,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.interval != 0 {
        object.key("Interval").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.interval).into()),
        );
    }
    if let Some(var_119) = &input.interval_unit {
        object.key("IntervalUnit").string(var_119.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_encryption_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EncryptionConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_120) = &input.encrypted {
        object.key("Encrypted").boolean(*var_120);
    }
    if let Some(var_121) = &input.cmk_arn {
        object.key("CmkArn").string(var_121.as_str());
    }
    Ok(())
}
