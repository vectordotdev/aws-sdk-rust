// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_replication_group_member_action(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DeleteReplicationGroupMemberAction,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.region_name {
        object.key("RegionName").string(var_1.as_str());
    }
    Ok(())
}
