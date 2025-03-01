// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteNamespace`](crate::operation::delete_namespace::builders::DeleteNamespaceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::delete_namespace::builders::DeleteNamespaceFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::delete_namespace::builders::DeleteNamespaceFluentBuilder::set_aws_account_id): <p>The ID for the Amazon Web Services account that you want to delete the Amazon QuickSight namespace from.</p>
    ///   - [`namespace(impl ::std::convert::Into<String>)`](crate::operation::delete_namespace::builders::DeleteNamespaceFluentBuilder::namespace) / [`set_namespace(Option<String>)`](crate::operation::delete_namespace::builders::DeleteNamespaceFluentBuilder::set_namespace): <p>The namespace that you want to delete.</p>
    /// - On success, responds with [`DeleteNamespaceOutput`](crate::operation::delete_namespace::DeleteNamespaceOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::delete_namespace::DeleteNamespaceOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::delete_namespace::DeleteNamespaceOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<DeleteNamespaceError>`](crate::operation::delete_namespace::DeleteNamespaceError)
    pub fn delete_namespace(
        &self,
    ) -> crate::operation::delete_namespace::builders::DeleteNamespaceFluentBuilder {
        crate::operation::delete_namespace::builders::DeleteNamespaceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
