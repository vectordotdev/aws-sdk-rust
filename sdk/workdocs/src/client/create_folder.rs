// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateFolder`](crate::operation::create_folder::builders::CreateFolderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`authentication_token(impl ::std::convert::Into<String>)`](crate::operation::create_folder::builders::CreateFolderFluentBuilder::authentication_token) / [`set_authentication_token(Option<String>)`](crate::operation::create_folder::builders::CreateFolderFluentBuilder::set_authentication_token): <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_folder::builders::CreateFolderFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_folder::builders::CreateFolderFluentBuilder::set_name): <p>The name of the new folder.</p>
    ///   - [`parent_folder_id(impl ::std::convert::Into<String>)`](crate::operation::create_folder::builders::CreateFolderFluentBuilder::parent_folder_id) / [`set_parent_folder_id(Option<String>)`](crate::operation::create_folder::builders::CreateFolderFluentBuilder::set_parent_folder_id): <p>The ID of the parent folder.</p>
    /// - On success, responds with [`CreateFolderOutput`](crate::operation::create_folder::CreateFolderOutput) with field(s):
    ///   - [`metadata(Option<FolderMetadata>)`](crate::operation::create_folder::CreateFolderOutput::metadata): <p>The metadata of the folder.</p>
    /// - On failure, responds with [`SdkError<CreateFolderError>`](crate::operation::create_folder::CreateFolderError)
    pub fn create_folder(
        &self,
    ) -> crate::operation::create_folder::builders::CreateFolderFluentBuilder {
        crate::operation::create_folder::builders::CreateFolderFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
