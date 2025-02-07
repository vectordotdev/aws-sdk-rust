// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_tle_data(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TleData,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.tle_line1 {
        object.key("tleLine1").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tle_line2 {
        object.key("tleLine2").string(var_2.as_str());
    }
    if let Some(var_3) = &input.valid_time_range {
        #[allow(unused_mut)]
        let mut object_4 = object.key("validTimeRange").start_object();
        crate::protocol_serde::shape_time_range::ser_time_range(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
