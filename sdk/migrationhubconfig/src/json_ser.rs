// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_home_region_control_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateHomeRegionControlInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.home_region {
        object.key("HomeRegion").string(var_1.as_str());
    }
    if let Some(var_2) = &input.target {
        let mut object_3 = object.key("Target").start_object();
        crate::json_ser::serialize_structure_crate_model_target(&mut object_3, var_2)?;
        object_3.finish();
    }
    if input.dry_run {
        object.key("DryRun").boolean(input.dry_run);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_home_region_controls_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeHomeRegionControlsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_4) = &input.control_id {
        object.key("ControlId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.home_region {
        object.key("HomeRegion").string(var_5.as_str());
    }
    if let Some(var_6) = &input.target {
        let mut object_7 = object.key("Target").start_object();
        crate::json_ser::serialize_structure_crate_model_target(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.next_token {
        object.key("NextToken").string(var_9.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_target(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Target,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_10) = &input.r#type {
        object.key("Type").string(var_10.as_str());
    }
    if let Some(var_11) = &input.id {
        object.key("Id").string(var_11.as_str());
    }
    Ok(())
}
