// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_remove_storage_system_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::remove_storage_system::RemoveStorageSystemInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.storage_system_arn {
        object.key("StorageSystemArn").string(var_1.as_str());
    }
    Ok(())
}
