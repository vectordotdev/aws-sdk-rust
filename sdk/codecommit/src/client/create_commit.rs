// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateCommit`](crate::operation::create_commit::builders::CreateCommitFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`repository_name(impl ::std::convert::Into<String>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::repository_name) / [`set_repository_name(Option<String>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::set_repository_name): <p>The name of the repository where you create the commit.</p>
    ///   - [`branch_name(impl ::std::convert::Into<String>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::branch_name) / [`set_branch_name(Option<String>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::set_branch_name): <p>The name of the branch where you create the commit.</p>
    ///   - [`parent_commit_id(impl ::std::convert::Into<String>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::parent_commit_id) / [`set_parent_commit_id(Option<String>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::set_parent_commit_id): <p>The ID of the commit that is the parent of the commit you create. Not required if this is an empty repository.</p>
    ///   - [`author_name(impl ::std::convert::Into<String>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::author_name) / [`set_author_name(Option<String>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::set_author_name): <p>The name of the author who created the commit. This information is used as both the author and committer for the commit.</p>
    ///   - [`email(impl ::std::convert::Into<String>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::email) / [`set_email(Option<String>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::set_email): <p>The email address of the person who created the commit.</p>
    ///   - [`commit_message(impl ::std::convert::Into<String>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::commit_message) / [`set_commit_message(Option<String>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::set_commit_message): <p>The commit message you want to include in the commit. Commit messages are limited to 256 KB. If no message is specified, a default message is used.</p>
    ///   - [`keep_empty_folders(bool)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::keep_empty_folders) / [`set_keep_empty_folders(bool)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::set_keep_empty_folders): <p>If the commit contains deletions, whether to keep a folder or folder structure if the changes leave the folders empty. If true, a ..gitkeep file is created for empty folders. The default is false.</p>
    ///   - [`put_files(Vec<PutFileEntry>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::put_files) / [`set_put_files(Option<Vec<PutFileEntry>>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::set_put_files): <p>The files to add or update in this commit.</p>
    ///   - [`delete_files(Vec<DeleteFileEntry>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::delete_files) / [`set_delete_files(Option<Vec<DeleteFileEntry>>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::set_delete_files): <p>The files to delete in this commit. These files still exist in earlier commits.</p>
    ///   - [`set_file_modes(Vec<SetFileModeEntry>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::set_file_modes) / [`set_set_file_modes(Option<Vec<SetFileModeEntry>>)`](crate::operation::create_commit::builders::CreateCommitFluentBuilder::set_set_file_modes): <p>The file modes to update for files in this commit.</p>
    /// - On success, responds with [`CreateCommitOutput`](crate::operation::create_commit::CreateCommitOutput) with field(s):
    ///   - [`commit_id(Option<String>)`](crate::operation::create_commit::CreateCommitOutput::commit_id): <p>The full commit ID of the commit that contains your committed file changes.</p>
    ///   - [`tree_id(Option<String>)`](crate::operation::create_commit::CreateCommitOutput::tree_id): <p>The full SHA-1 pointer of the tree information for the commit that contains the commited file changes.</p>
    ///   - [`files_added(Option<Vec<FileMetadata>>)`](crate::operation::create_commit::CreateCommitOutput::files_added): <p>The files added as part of the committed file changes.</p>
    ///   - [`files_updated(Option<Vec<FileMetadata>>)`](crate::operation::create_commit::CreateCommitOutput::files_updated): <p>The files updated as part of the commited file changes.</p>
    ///   - [`files_deleted(Option<Vec<FileMetadata>>)`](crate::operation::create_commit::CreateCommitOutput::files_deleted): <p>The files deleted as part of the committed file changes.</p>
    /// - On failure, responds with [`SdkError<CreateCommitError>`](crate::operation::create_commit::CreateCommitError)
    pub fn create_commit(
        &self,
    ) -> crate::operation::create_commit::builders::CreateCommitFluentBuilder {
        crate::operation::create_commit::builders::CreateCommitFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
