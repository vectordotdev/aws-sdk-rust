// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_registry_catalog_data_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_registry_catalog_data::PutRegistryCatalogDataInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.display_name {
        object.key("displayName").string(var_1.as_str());
    }
    Ok(())
}
