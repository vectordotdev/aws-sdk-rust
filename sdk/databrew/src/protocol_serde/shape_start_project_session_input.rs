// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_project_session_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_project_session::StartProjectSessionInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if input.assume_control {
        object.key("AssumeControl").boolean(input.assume_control);
    }
    Ok(())
}
