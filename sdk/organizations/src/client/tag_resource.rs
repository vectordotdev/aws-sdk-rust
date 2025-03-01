// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TagResource`](crate::operation::tag_resource::builders::TagResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_id(impl ::std::convert::Into<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_resource_id): <p>The ID of the resource to add a tag to.</p>  <p>You can specify any of the following taggable resources.</p>  <ul>   <li> <p>Amazon Web Services account – specify the account ID number.</p> </li>   <li> <p>Organizational unit – specify the OU ID that begins with <code>ou-</code> and looks similar to: <code>ou-<i>1a2b-34uvwxyz</i> </code> </p> </li>   <li> <p>Root – specify the root ID that begins with <code>r-</code> and looks similar to: <code>r-<i>1a2b</i> </code> </p> </li>   <li> <p>Policy – specify the policy ID that begins with <code>p-</code> andlooks similar to: <code>p-<i>12abcdefg3</i> </code> </p> </li>  </ul>
    ///   - [`tags(Vec<Tag>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_tags): <p>A list of tags to add to the specified resource.</p>  <p>For each tag in the list, you must specify both a tag key and a value. The value can be an empty string, but you can't set it to <code>null</code>.</p> <note>   <p>If any one of the tags is not valid or if you exceed the maximum allowed number of tags for a resource, then the entire request fails.</p>  </note>
    /// - On success, responds with [`TagResourceOutput`](crate::operation::tag_resource::TagResourceOutput)
    /// - On failure, responds with [`SdkError<TagResourceError>`](crate::operation::tag_resource::TagResourceError)
    pub fn tag_resource(
        &self,
    ) -> crate::operation::tag_resource::builders::TagResourceFluentBuilder {
        crate::operation::tag_resource::builders::TagResourceFluentBuilder::new(self.handle.clone())
    }
}
