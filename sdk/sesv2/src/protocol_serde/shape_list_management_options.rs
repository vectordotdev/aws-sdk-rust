// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_management_options(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ListManagementOptions,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.contact_list_name {
        object.key("ContactListName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.topic_name {
        object.key("TopicName").string(var_2.as_str());
    }
    Ok(())
}
