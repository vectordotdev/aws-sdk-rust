// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_transcription_job_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_transcription_job::DeleteTranscriptionJobInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.transcription_job_name {
        object.key("TranscriptionJobName").string(var_1.as_str());
    }
    Ok(())
}
