// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_managed_agent_state_change(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ManagedAgentStateChange,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.container_name {
        object.key("containerName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.managed_agent_name {
        object.key("managedAgentName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.status {
        object.key("status").string(var_3.as_str());
    }
    if let Some(var_4) = &input.reason {
        object.key("reason").string(var_4.as_str());
    }
    Ok(())
}
