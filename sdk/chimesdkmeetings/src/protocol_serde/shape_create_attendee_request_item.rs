// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_attendee_request_item(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CreateAttendeeRequestItem,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.external_user_id {
        object.key("ExternalUserId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.capabilities {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Capabilities").start_object();
        crate::protocol_serde::shape_attendee_capabilities::ser_attendee_capabilities(
            &mut object_3,
            var_2,
        )?;
        object_3.finish();
    }
    Ok(())
}
