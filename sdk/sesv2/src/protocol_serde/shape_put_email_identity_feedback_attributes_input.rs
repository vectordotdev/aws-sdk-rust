// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_email_identity_feedback_attributes_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_email_identity_feedback_attributes::PutEmailIdentityFeedbackAttributesInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.email_forwarding_enabled {
        object.key("EmailForwardingEnabled").boolean(*var_1);
    }
    Ok(())
}
