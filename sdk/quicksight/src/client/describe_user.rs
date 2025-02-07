// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeUser`](crate::operation::describe_user::builders::DescribeUserFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl ::std::convert::Into<String>)`](crate::operation::describe_user::builders::DescribeUserFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::describe_user::builders::DescribeUserFluentBuilder::set_user_name): <p>The name of the user that you want to describe.</p>
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::describe_user::builders::DescribeUserFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::describe_user::builders::DescribeUserFluentBuilder::set_aws_account_id): <p>The ID for the Amazon Web Services account that the user is in. Currently, you use the ID for the Amazon Web Services account that contains your Amazon QuickSight account.</p>
    ///   - [`namespace(impl ::std::convert::Into<String>)`](crate::operation::describe_user::builders::DescribeUserFluentBuilder::namespace) / [`set_namespace(Option<String>)`](crate::operation::describe_user::builders::DescribeUserFluentBuilder::set_namespace): <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    /// - On success, responds with [`DescribeUserOutput`](crate::operation::describe_user::DescribeUserOutput) with field(s):
    ///   - [`user(Option<User>)`](crate::operation::describe_user::DescribeUserOutput::user): <p>The user name.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::describe_user::DescribeUserOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::describe_user::DescribeUserOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<DescribeUserError>`](crate::operation::describe_user::DescribeUserError)
    pub fn describe_user(
        &self,
    ) -> crate::operation::describe_user::builders::DescribeUserFluentBuilder {
        crate::operation::describe_user::builders::DescribeUserFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
