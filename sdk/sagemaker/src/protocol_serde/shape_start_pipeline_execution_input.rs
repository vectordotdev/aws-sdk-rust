// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_pipeline_execution_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_pipeline_execution::StartPipelineExecutionInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pipeline_name {
        object.key("PipelineName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.pipeline_execution_display_name {
        object
            .key("PipelineExecutionDisplayName")
            .string(var_2.as_str());
    }
    if let Some(var_3) = &input.pipeline_parameters {
        let mut array_4 = object.key("PipelineParameters").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_parameter::ser_parameter(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.pipeline_execution_description {
        object
            .key("PipelineExecutionDescription")
            .string(var_7.as_str());
    }
    if let Some(var_8) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_8.as_str());
    }
    if let Some(var_9) = &input.parallelism_configuration {
        #[allow(unused_mut)]
        let mut object_10 = object.key("ParallelismConfiguration").start_object();
        crate::protocol_serde::shape_parallelism_configuration::ser_parallelism_configuration(
            &mut object_10,
            var_9,
        )?;
        object_10.finish();
    }
    Ok(())
}
