// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_deployment_config_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_deployment_config::GetDeploymentConfigInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.deployment_config_name {
        object.key("deploymentConfigName").string(var_1.as_str());
    }
    Ok(())
}
