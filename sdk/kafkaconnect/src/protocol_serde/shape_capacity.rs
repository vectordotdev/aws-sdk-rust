// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_capacity(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Capacity,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.auto_scaling {
        #[allow(unused_mut)]
        let mut object_2 = object.key("autoScaling").start_object();
        crate::protocol_serde::shape_auto_scaling::ser_auto_scaling(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.provisioned_capacity {
        #[allow(unused_mut)]
        let mut object_4 = object.key("provisionedCapacity").start_object();
        crate::protocol_serde::shape_provisioned_capacity::ser_provisioned_capacity(
            &mut object_4,
            var_3,
        )?;
        object_4.finish();
    }
    Ok(())
}
