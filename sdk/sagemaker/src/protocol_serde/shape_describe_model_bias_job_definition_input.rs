// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_model_bias_job_definition_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_model_bias_job_definition::DescribeModelBiasJobDefinitionInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.job_definition_name {
        object.key("JobDefinitionName").string(var_1.as_str());
    }
    Ok(())
}
