// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_resource_arn): <p>The Amazon Resource Name (ARN) string that uniquely identifies the DMS resource to list tags for. This returns a list of keys (names of tags) created for the resource and their associated tag values.</p>
    ///   - [`resource_arn_list(Vec<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::resource_arn_list) / [`set_resource_arn_list(Option<Vec<String>>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_resource_arn_list): <p>List of ARNs that identify multiple DMS resources that you want to list tags for. This returns a list of keys (tag names) and their associated tag values. It also returns each tag's associated <code>ResourceArn</code> value, which is the ARN of the resource for which each listed tag is created. </p>
    /// - On success, responds with [`ListTagsForResourceOutput`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput) with field(s):
    ///   - [`tag_list(Option<Vec<Tag>>)`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput::tag_list): <p>A list of tags for the resource.</p>
    /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::operation::list_tags_for_resource::ListTagsForResourceError)
    pub fn list_tags_for_resource(
        &self,
    ) -> crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder {
        crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
