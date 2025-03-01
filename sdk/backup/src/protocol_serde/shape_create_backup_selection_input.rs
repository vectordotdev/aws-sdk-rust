// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_backup_selection_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_backup_selection::CreateBackupSelectionInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.backup_selection {
        #[allow(unused_mut)]
        let mut object_2 = object.key("BackupSelection").start_object();
        crate::protocol_serde::shape_backup_selection::ser_backup_selection(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.creator_request_id {
        object.key("CreatorRequestId").string(var_3.as_str());
    }
    Ok(())
}
