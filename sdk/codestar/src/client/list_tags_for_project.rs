// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTagsForProject`](crate::operation::list_tags_for_project::builders::ListTagsForProjectFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::list_tags_for_project::builders::ListTagsForProjectFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::list_tags_for_project::builders::ListTagsForProjectFluentBuilder::set_id): <p>The ID of the project to get tags for.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_tags_for_project::builders::ListTagsForProjectFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_tags_for_project::builders::ListTagsForProjectFluentBuilder::set_next_token): <p>Reserved for future use.</p>
    ///   - [`max_results(i32)`](crate::operation::list_tags_for_project::builders::ListTagsForProjectFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_tags_for_project::builders::ListTagsForProjectFluentBuilder::set_max_results): <p>Reserved for future use.</p>
    /// - On success, responds with [`ListTagsForProjectOutput`](crate::operation::list_tags_for_project::ListTagsForProjectOutput) with field(s):
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::list_tags_for_project::ListTagsForProjectOutput::tags): <p>The tags for the project.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_tags_for_project::ListTagsForProjectOutput::next_token): <p>Reserved for future use.</p>
    /// - On failure, responds with [`SdkError<ListTagsForProjectError>`](crate::operation::list_tags_for_project::ListTagsForProjectError)
    pub fn list_tags_for_project(
        &self,
    ) -> crate::operation::list_tags_for_project::builders::ListTagsForProjectFluentBuilder {
        crate::operation::list_tags_for_project::builders::ListTagsForProjectFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
