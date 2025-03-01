// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateDefaultView`](crate::operation::associate_default_view::builders::AssociateDefaultViewFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`view_arn(impl ::std::convert::Into<String>)`](crate::operation::associate_default_view::builders::AssociateDefaultViewFluentBuilder::view_arn) / [`set_view_arn(Option<String>)`](crate::operation::associate_default_view::builders::AssociateDefaultViewFluentBuilder::set_view_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view to set as the default for the Amazon Web Services Region and Amazon Web Services account in which you call this operation. The specified view must already exist in the called Region.</p>
    /// - On success, responds with [`AssociateDefaultViewOutput`](crate::operation::associate_default_view::AssociateDefaultViewOutput) with field(s):
    ///   - [`view_arn(Option<String>)`](crate::operation::associate_default_view::AssociateDefaultViewOutput::view_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that the operation set as the default for queries made in the Amazon Web Services Region and Amazon Web Services account in which you called this operation.</p>
    /// - On failure, responds with [`SdkError<AssociateDefaultViewError>`](crate::operation::associate_default_view::AssociateDefaultViewError)
    pub fn associate_default_view(
        &self,
    ) -> crate::operation::associate_default_view::builders::AssociateDefaultViewFluentBuilder {
        crate::operation::associate_default_view::builders::AssociateDefaultViewFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
