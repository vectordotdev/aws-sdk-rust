// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_start_next_pending_job_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartNextPendingJobExecutionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.status_details {
        let mut object_2 = object.key("statusDetails").start_object();
        for (key_3, value_4) in var_1 {
            {
                object_2.key(key_3.as_str()).string(value_4.as_str());
            }
        }
        object_2.finish();
    }
    if let Some(var_5) = &input.step_timeout_in_minutes {
        object.key("stepTimeoutInMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_job_execution_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateJobExecutionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_6) = &input.execution_number {
        object.key("executionNumber").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.expected_version {
        object.key("expectedVersion").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.include_job_document {
        object.key("includeJobDocument").boolean(*var_8);
    }
    if let Some(var_9) = &input.include_job_execution_state {
        object.key("includeJobExecutionState").boolean(*var_9);
    }
    if let Some(var_10) = &input.status {
        object.key("status").string(var_10.as_str());
    }
    if let Some(var_11) = &input.status_details {
        let mut object_12 = object.key("statusDetails").start_object();
        for (key_13, value_14) in var_11 {
            {
                object_12.key(key_13.as_str()).string(value_14.as_str());
            }
        }
        object_12.finish();
    }
    if let Some(var_15) = &input.step_timeout_in_minutes {
        object.key("stepTimeoutInMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_15).into()),
        );
    }
    Ok(())
}
