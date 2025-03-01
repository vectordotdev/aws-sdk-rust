// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_simple_email_part(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SimpleEmailPart,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.charset {
        object.key("Charset").string(var_1.as_str());
    }
    if let Some(var_2) = &input.data {
        object.key("Data").string(var_2.as_str());
    }
    Ok(())
}
