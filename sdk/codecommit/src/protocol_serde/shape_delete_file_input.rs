// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_file_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_file::DeleteFileInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.repository_name {
        object.key("repositoryName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.branch_name {
        object.key("branchName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.file_path {
        object.key("filePath").string(var_3.as_str());
    }
    if let Some(var_4) = &input.parent_commit_id {
        object.key("parentCommitId").string(var_4.as_str());
    }
    if input.keep_empty_folders {
        object
            .key("keepEmptyFolders")
            .boolean(input.keep_empty_folders);
    }
    if let Some(var_5) = &input.commit_message {
        object.key("commitMessage").string(var_5.as_str());
    }
    if let Some(var_6) = &input.name {
        object.key("name").string(var_6.as_str());
    }
    if let Some(var_7) = &input.email {
        object.key("email").string(var_7.as_str());
    }
    Ok(())
}
