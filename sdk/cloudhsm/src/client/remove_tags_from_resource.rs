// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RemoveTagsFromResource`](crate::operation::remove_tags_from_resource::builders::RemoveTagsFromResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::remove_tags_from_resource::builders::RemoveTagsFromResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::remove_tags_from_resource::builders::RemoveTagsFromResourceFluentBuilder::set_resource_arn): <p>The Amazon Resource Name (ARN) of the AWS CloudHSM resource.</p>
    ///   - [`tag_key_list(Vec<String>)`](crate::operation::remove_tags_from_resource::builders::RemoveTagsFromResourceFluentBuilder::tag_key_list) / [`set_tag_key_list(Option<Vec<String>>)`](crate::operation::remove_tags_from_resource::builders::RemoveTagsFromResourceFluentBuilder::set_tag_key_list): <p>The tag key or keys to remove.</p>  <p>Specify only the tag key to remove (not the value). To overwrite the value for an existing tag, use <code>AddTagsToResource</code>.</p>
    /// - On success, responds with [`RemoveTagsFromResourceOutput`](crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput) with field(s):
    ///   - [`status(Option<String>)`](crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput::status): <p>The status of the operation.</p>
    /// - On failure, responds with [`SdkError<RemoveTagsFromResourceError>`](crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError)
    pub fn remove_tags_from_resource(
        &self,
    ) -> crate::operation::remove_tags_from_resource::builders::RemoveTagsFromResourceFluentBuilder
    {
        crate::operation::remove_tags_from_resource::builders::RemoveTagsFromResourceFluentBuilder::new(self.handle.clone())
    }
}
