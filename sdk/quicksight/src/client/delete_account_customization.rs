// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAccountCustomization`](crate::operation::delete_account_customization::builders::DeleteAccountCustomizationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::delete_account_customization::builders::DeleteAccountCustomizationFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::delete_account_customization::builders::DeleteAccountCustomizationFluentBuilder::set_aws_account_id): <p>The ID for the Amazon Web Services account that you want to delete Amazon QuickSight customizations from in this Amazon Web Services Region.</p>
    ///   - [`namespace(impl ::std::convert::Into<String>)`](crate::operation::delete_account_customization::builders::DeleteAccountCustomizationFluentBuilder::namespace) / [`set_namespace(Option<String>)`](crate::operation::delete_account_customization::builders::DeleteAccountCustomizationFluentBuilder::set_namespace): <p>The Amazon QuickSight namespace that you're deleting the customizations from.</p>
    /// - On success, responds with [`DeleteAccountCustomizationOutput`](crate::operation::delete_account_customization::DeleteAccountCustomizationOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::delete_account_customization::DeleteAccountCustomizationOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::delete_account_customization::DeleteAccountCustomizationOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<DeleteAccountCustomizationError>`](crate::operation::delete_account_customization::DeleteAccountCustomizationError)
    pub fn delete_account_customization(&self) -> crate::operation::delete_account_customization::builders::DeleteAccountCustomizationFluentBuilder{
        crate::operation::delete_account_customization::builders::DeleteAccountCustomizationFluentBuilder::new(self.handle.clone())
    }
}
