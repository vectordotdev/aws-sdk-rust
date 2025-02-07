// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ConfirmProductInstance`](crate::operation::confirm_product_instance::builders::ConfirmProductInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::confirm_product_instance::builders::ConfirmProductInstanceFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::confirm_product_instance::builders::ConfirmProductInstanceFluentBuilder::set_instance_id): <p>The ID of the instance.</p>
    ///   - [`product_code(impl ::std::convert::Into<String>)`](crate::operation::confirm_product_instance::builders::ConfirmProductInstanceFluentBuilder::product_code) / [`set_product_code(Option<String>)`](crate::operation::confirm_product_instance::builders::ConfirmProductInstanceFluentBuilder::set_product_code): <p>The product code. This must be a product code that you own.</p>
    ///   - [`dry_run(bool)`](crate::operation::confirm_product_instance::builders::ConfirmProductInstanceFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::confirm_product_instance::builders::ConfirmProductInstanceFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`ConfirmProductInstanceOutput`](crate::operation::confirm_product_instance::ConfirmProductInstanceOutput) with field(s):
    ///   - [`owner_id(Option<String>)`](crate::operation::confirm_product_instance::ConfirmProductInstanceOutput::owner_id): <p>The Amazon Web Services account ID of the instance owner. This is only present if the product code is attached to the instance.</p>
    ///   - [`r#return(Option<bool>)`](crate::operation::confirm_product_instance::ConfirmProductInstanceOutput::return): <p>The return value of the request. Returns <code>true</code> if the specified product code is owned by the requester and associated with the specified instance.</p>
    /// - On failure, responds with [`SdkError<ConfirmProductInstanceError>`](crate::operation::confirm_product_instance::ConfirmProductInstanceError)
    pub fn confirm_product_instance(
        &self,
    ) -> crate::operation::confirm_product_instance::builders::ConfirmProductInstanceFluentBuilder
    {
        crate::operation::confirm_product_instance::builders::ConfirmProductInstanceFluentBuilder::new(self.handle.clone())
    }
}
