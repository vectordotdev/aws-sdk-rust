// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_import_game_configuration_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::import_game_configuration::ImportGameConfigurationInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.import_source {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ImportSource").start_object();
        crate::protocol_serde::shape_import_game_configuration_source::ser_import_game_configuration_source(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
