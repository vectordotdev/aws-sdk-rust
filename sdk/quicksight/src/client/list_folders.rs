// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListFolders`](crate::operation::list_folders::builders::ListFoldersFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::list_folders::builders::ListFoldersFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::list_folders::builders::ListFoldersFluentBuilder::set_aws_account_id): <p>The ID for the Amazon Web Services account that contains the folder.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_folders::builders::ListFoldersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_folders::builders::ListFoldersFluentBuilder::set_next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_folders::builders::ListFoldersFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_folders::builders::ListFoldersFluentBuilder::set_max_results): <p>The maximum number of results to be returned per request.</p>
    /// - On success, responds with [`ListFoldersOutput`](crate::operation::list_folders::ListFoldersOutput) with field(s):
    ///   - [`status(i32)`](crate::operation::list_folders::ListFoldersOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`folder_summary_list(Option<Vec<FolderSummary>>)`](crate::operation::list_folders::ListFoldersOutput::folder_summary_list): <p>A structure that contains all of the folders in the Amazon Web Services account. This structure provides basic information about the folders.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_folders::ListFoldersOutput::next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::list_folders::ListFoldersOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    /// - On failure, responds with [`SdkError<ListFoldersError>`](crate::operation::list_folders::ListFoldersError)
    pub fn list_folders(
        &self,
    ) -> crate::operation::list_folders::builders::ListFoldersFluentBuilder {
        crate::operation::list_folders::builders::ListFoldersFluentBuilder::new(self.handle.clone())
    }
}
