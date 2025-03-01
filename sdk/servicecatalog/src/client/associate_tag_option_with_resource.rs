// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateTagOptionWithResource`](crate::operation::associate_tag_option_with_resource::builders::AssociateTagOptionWithResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_id(impl ::std::convert::Into<String>)`](crate::operation::associate_tag_option_with_resource::builders::AssociateTagOptionWithResourceFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::associate_tag_option_with_resource::builders::AssociateTagOptionWithResourceFluentBuilder::set_resource_id): <p>The resource identifier.</p>
    ///   - [`tag_option_id(impl ::std::convert::Into<String>)`](crate::operation::associate_tag_option_with_resource::builders::AssociateTagOptionWithResourceFluentBuilder::tag_option_id) / [`set_tag_option_id(Option<String>)`](crate::operation::associate_tag_option_with_resource::builders::AssociateTagOptionWithResourceFluentBuilder::set_tag_option_id): <p>The TagOption identifier.</p>
    /// - On success, responds with [`AssociateTagOptionWithResourceOutput`](crate::operation::associate_tag_option_with_resource::AssociateTagOptionWithResourceOutput)
    /// - On failure, responds with [`SdkError<AssociateTagOptionWithResourceError>`](crate::operation::associate_tag_option_with_resource::AssociateTagOptionWithResourceError)
    pub fn associate_tag_option_with_resource(&self) -> crate::operation::associate_tag_option_with_resource::builders::AssociateTagOptionWithResourceFluentBuilder{
        crate::operation::associate_tag_option_with_resource::builders::AssociateTagOptionWithResourceFluentBuilder::new(self.handle.clone())
    }
}
