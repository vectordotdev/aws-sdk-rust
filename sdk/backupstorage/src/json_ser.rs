// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_start_object_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartObjectInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.throw_on_duplicate {
        object
            .key("ThrowOnDuplicate")
            .boolean(input.throw_on_duplicate);
    }
    Ok(())
}
