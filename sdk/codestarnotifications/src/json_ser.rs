// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_notification_rule_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateNotificationRuleInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.detail_type {
        object.key("DetailType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.event_type_ids {
        let mut array_4 = object.key("EventTypeIds").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.name {
        object.key("Name").string(var_6.as_str());
    }
    if let Some(var_7) = &input.resource {
        object.key("Resource").string(var_7.as_str());
    }
    if let Some(var_8) = &input.status {
        object.key("Status").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        let mut object_10 = object.key("Tags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    if let Some(var_13) = &input.targets {
        let mut array_14 = object.key("Targets").start_array();
        for item_15 in var_13 {
            {
                let mut object_16 = array_14.value().start_object();
                crate::json_ser::serialize_structure_crate_model_target(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_notification_rule_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteNotificationRuleInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_17) = &input.arn {
        object.key("Arn").string(var_17.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_target_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteTargetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.force_unsubscribe_all {
        object
            .key("ForceUnsubscribeAll")
            .boolean(input.force_unsubscribe_all);
    }
    if let Some(var_18) = &input.target_address {
        object.key("TargetAddress").string(var_18.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_notification_rule_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeNotificationRuleInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_19) = &input.arn {
        object.key("Arn").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_event_types_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListEventTypesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.filters {
        let mut array_21 = object.key("Filters").start_array();
        for item_22 in var_20 {
            {
                let mut object_23 = array_21.value().start_object();
                crate::json_ser::serialize_structure_crate_model_list_event_types_filter(
                    &mut object_23,
                    item_22,
                )?;
                object_23.finish();
            }
        }
        array_21.finish();
    }
    if let Some(var_24) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_24).into()),
        );
    }
    if let Some(var_25) = &input.next_token {
        object.key("NextToken").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_notification_rules_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListNotificationRulesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.filters {
        let mut array_27 = object.key("Filters").start_array();
        for item_28 in var_26 {
            {
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_crate_model_list_notification_rules_filter(
                    &mut object_29,
                    item_28,
                )?;
                object_29.finish();
            }
        }
        array_27.finish();
    }
    if let Some(var_30) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_30).into()),
        );
    }
    if let Some(var_31) = &input.next_token {
        object.key("NextToken").string(var_31.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_32) = &input.arn {
        object.key("Arn").string(var_32.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_targets_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTargetsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_33) = &input.filters {
        let mut array_34 = object.key("Filters").start_array();
        for item_35 in var_33 {
            {
                let mut object_36 = array_34.value().start_object();
                crate::json_ser::serialize_structure_crate_model_list_targets_filter(
                    &mut object_36,
                    item_35,
                )?;
                object_36.finish();
            }
        }
        array_34.finish();
    }
    if let Some(var_37) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_37).into()),
        );
    }
    if let Some(var_38) = &input.next_token {
        object.key("NextToken").string(var_38.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_subscribe_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SubscribeInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_39) = &input.arn {
        object.key("Arn").string(var_39.as_str());
    }
    if let Some(var_40) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_40.as_str());
    }
    if let Some(var_41) = &input.target {
        let mut object_42 = object.key("Target").start_object();
        crate::json_ser::serialize_structure_crate_model_target(&mut object_42, var_41)?;
        object_42.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_43) = &input.arn {
        object.key("Arn").string(var_43.as_str());
    }
    if let Some(var_44) = &input.tags {
        let mut object_45 = object.key("Tags").start_object();
        for (key_46, value_47) in var_44 {
            {
                object_45.key(key_46.as_str()).string(value_47.as_str());
            }
        }
        object_45.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_unsubscribe_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UnsubscribeInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_48) = &input.arn {
        object.key("Arn").string(var_48.as_str());
    }
    if let Some(var_49) = &input.target_address {
        object.key("TargetAddress").string(var_49.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_notification_rule_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateNotificationRuleInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.arn {
        object.key("Arn").string(var_50.as_str());
    }
    if let Some(var_51) = &input.detail_type {
        object.key("DetailType").string(var_51.as_str());
    }
    if let Some(var_52) = &input.event_type_ids {
        let mut array_53 = object.key("EventTypeIds").start_array();
        for item_54 in var_52 {
            {
                array_53.value().string(item_54.as_str());
            }
        }
        array_53.finish();
    }
    if let Some(var_55) = &input.name {
        object.key("Name").string(var_55.as_str());
    }
    if let Some(var_56) = &input.status {
        object.key("Status").string(var_56.as_str());
    }
    if let Some(var_57) = &input.targets {
        let mut array_58 = object.key("Targets").start_array();
        for item_59 in var_57 {
            {
                let mut object_60 = array_58.value().start_object();
                crate::json_ser::serialize_structure_crate_model_target(&mut object_60, item_59)?;
                object_60.finish();
            }
        }
        array_58.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_target(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Target,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_61) = &input.target_type {
        object.key("TargetType").string(var_61.as_str());
    }
    if let Some(var_62) = &input.target_address {
        object.key("TargetAddress").string(var_62.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_list_event_types_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ListEventTypesFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.name {
        object.key("Name").string(var_63.as_str());
    }
    if let Some(var_64) = &input.value {
        object.key("Value").string(var_64.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_list_notification_rules_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ListNotificationRulesFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_65) = &input.name {
        object.key("Name").string(var_65.as_str());
    }
    if let Some(var_66) = &input.value {
        object.key("Value").string(var_66.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_list_targets_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ListTargetsFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_67) = &input.name {
        object.key("Name").string(var_67.as_str());
    }
    if let Some(var_68) = &input.value {
        object.key("Value").string(var_68.as_str());
    }
    Ok(())
}
