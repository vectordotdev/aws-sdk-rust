// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_gateway_to_server_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateGatewayToServerInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.gateway_arn {
        object.key("GatewayArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.server_arn {
        object.key("ServerArn").string(var_2.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_gateway_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateGatewayInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_3) = &input.activation_key {
        object.key("ActivationKey").string(var_3.as_str());
    }
    if let Some(var_4) = &input.gateway_display_name {
        object.key("GatewayDisplayName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.gateway_type {
        object.key("GatewayType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        let mut array_7 = object.key("Tags").start_array();
        for item_8 in var_6 {
            {
                let mut object_9 = array_7.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_gateway_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteGatewayInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_10) = &input.gateway_arn {
        object.key("GatewayArn").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_hypervisor_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteHypervisorInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_11) = &input.hypervisor_arn {
        object.key("HypervisorArn").string(var_11.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disassociate_gateway_from_server_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateGatewayFromServerInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.gateway_arn {
        object.key("GatewayArn").string(var_12.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_gateway_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetGatewayInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_13) = &input.gateway_arn {
        object.key("GatewayArn").string(var_13.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_virtual_machine_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetVirtualMachineInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_14) = &input.resource_arn {
        object.key("ResourceArn").string(var_14.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_import_hypervisor_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ImportHypervisorConfigurationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_15) = &input.name {
        object.key("Name").string(var_15.as_str());
    }
    if let Some(var_16) = &input.host {
        object.key("Host").string(var_16.as_str());
    }
    if let Some(var_17) = &input.username {
        object.key("Username").string(var_17.as_str());
    }
    if let Some(var_18) = &input.password {
        object.key("Password").string(var_18.as_str());
    }
    if let Some(var_19) = &input.kms_key_arn {
        object.key("KmsKeyArn").string(var_19.as_str());
    }
    if let Some(var_20) = &input.tags {
        let mut array_21 = object.key("Tags").start_array();
        for item_22 in var_20 {
            {
                let mut object_23 = array_21.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_23, item_22)?;
                object_23.finish();
            }
        }
        array_21.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_gateways_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListGatewaysInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
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

pub fn serialize_structure_crate_input_list_hypervisors_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListHypervisorsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_26).into()),
        );
    }
    if let Some(var_27) = &input.next_token {
        object.key("NextToken").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.resource_arn {
        object.key("ResourceArn").string(var_28.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_virtual_machines_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListVirtualMachinesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_29) = &input.hypervisor_arn {
        object.key("HypervisorArn").string(var_29.as_str());
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

pub fn serialize_structure_crate_input_put_maintenance_start_time_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutMaintenanceStartTimeInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_32) = &input.gateway_arn {
        object.key("GatewayArn").string(var_32.as_str());
    }
    if let Some(var_33) = &input.hour_of_day {
        object.key("HourOfDay").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_33).into()),
        );
    }
    if let Some(var_34) = &input.minute_of_hour {
        object.key("MinuteOfHour").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_34).into()),
        );
    }
    if let Some(var_35) = &input.day_of_week {
        object.key("DayOfWeek").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_35).into()),
        );
    }
    if let Some(var_36) = &input.day_of_month {
        object.key("DayOfMonth").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_36).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_37) = &input.resource_arn {
        object.key("ResourceARN").string(var_37.as_str());
    }
    if let Some(var_38) = &input.tags {
        let mut array_39 = object.key("Tags").start_array();
        for item_40 in var_38 {
            {
                let mut object_41 = array_39.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_41, item_40)?;
                object_41.finish();
            }
        }
        array_39.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_test_hypervisor_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TestHypervisorConfigurationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_42) = &input.gateway_arn {
        object.key("GatewayArn").string(var_42.as_str());
    }
    if let Some(var_43) = &input.host {
        object.key("Host").string(var_43.as_str());
    }
    if let Some(var_44) = &input.username {
        object.key("Username").string(var_44.as_str());
    }
    if let Some(var_45) = &input.password {
        object.key("Password").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_46) = &input.resource_arn {
        object.key("ResourceARN").string(var_46.as_str());
    }
    if let Some(var_47) = &input.tag_keys {
        let mut array_48 = object.key("TagKeys").start_array();
        for item_49 in var_47 {
            {
                array_48.value().string(item_49.as_str());
            }
        }
        array_48.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_gateway_information_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateGatewayInformationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.gateway_arn {
        object.key("GatewayArn").string(var_50.as_str());
    }
    if let Some(var_51) = &input.gateway_display_name {
        object.key("GatewayDisplayName").string(var_51.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_gateway_software_now_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateGatewaySoftwareNowInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_52) = &input.gateway_arn {
        object.key("GatewayArn").string(var_52.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_hypervisor_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateHypervisorInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_53) = &input.hypervisor_arn {
        object.key("HypervisorArn").string(var_53.as_str());
    }
    if let Some(var_54) = &input.host {
        object.key("Host").string(var_54.as_str());
    }
    if let Some(var_55) = &input.username {
        object.key("Username").string(var_55.as_str());
    }
    if let Some(var_56) = &input.password {
        object.key("Password").string(var_56.as_str());
    }
    if let Some(var_57) = &input.name {
        object.key("Name").string(var_57.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_58) = &input.key {
        object.key("Key").string(var_58.as_str());
    }
    if let Some(var_59) = &input.value {
        object.key("Value").string(var_59.as_str());
    }
    Ok(())
}
