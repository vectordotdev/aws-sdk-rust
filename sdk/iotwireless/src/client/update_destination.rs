// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateDestination`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::set_name): <p>The new name of the resource.</p>
    ///   - [`expression_type(ExpressionType)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::expression_type) / [`set_expression_type(Option<ExpressionType>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::set_expression_type): <p>The type of value in <code>Expression</code>.</p>
    ///   - [`expression(impl ::std::convert::Into<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::expression) / [`set_expression(Option<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::set_expression): <p>The new rule name or topic rule to send messages to.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::set_description): <p>A new description of the resource.</p>
    ///   - [`role_arn(impl ::std::convert::Into<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::set_role_arn): <p>The ARN of the IAM Role that authorizes the destination.</p>
    /// - On success, responds with [`UpdateDestinationOutput`](crate::operation::update_destination::UpdateDestinationOutput)
    /// - On failure, responds with [`SdkError<UpdateDestinationError>`](crate::operation::update_destination::UpdateDestinationError)
    pub fn update_destination(
        &self,
    ) -> crate::operation::update_destination::builders::UpdateDestinationFluentBuilder {
        crate::operation::update_destination::builders::UpdateDestinationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
