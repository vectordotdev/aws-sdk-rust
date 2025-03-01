// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_streaming_session_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::stop_streaming_session::StopStreamingSessionInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.volume_retention_mode {
        object.key("volumeRetentionMode").string(var_1.as_str());
    }
    Ok(())
}
