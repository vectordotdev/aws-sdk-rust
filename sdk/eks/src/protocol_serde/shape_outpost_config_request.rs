// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_outpost_config_request(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::OutpostConfigRequest,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.outpost_arns {
        let mut array_2 = object.key("outpostArns").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.control_plane_instance_type {
        object
            .key("controlPlaneInstanceType")
            .string(var_4.as_str());
    }
    if let Some(var_5) = &input.control_plane_placement {
        #[allow(unused_mut)]
        let mut object_6 = object.key("controlPlanePlacement").start_object();
        crate::protocol_serde::shape_control_plane_placement_request::ser_control_plane_placement_request(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
