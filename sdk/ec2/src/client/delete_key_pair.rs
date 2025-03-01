// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteKeyPair`](crate::operation::delete_key_pair::builders::DeleteKeyPairFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_name(impl ::std::convert::Into<String>)`](crate::operation::delete_key_pair::builders::DeleteKeyPairFluentBuilder::key_name) / [`set_key_name(Option<String>)`](crate::operation::delete_key_pair::builders::DeleteKeyPairFluentBuilder::set_key_name): <p>The name of the key pair.</p>
    ///   - [`key_pair_id(impl ::std::convert::Into<String>)`](crate::operation::delete_key_pair::builders::DeleteKeyPairFluentBuilder::key_pair_id) / [`set_key_pair_id(Option<String>)`](crate::operation::delete_key_pair::builders::DeleteKeyPairFluentBuilder::set_key_pair_id): <p>The ID of the key pair.</p>
    ///   - [`dry_run(bool)`](crate::operation::delete_key_pair::builders::DeleteKeyPairFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_key_pair::builders::DeleteKeyPairFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DeleteKeyPairOutput`](crate::operation::delete_key_pair::DeleteKeyPairOutput)
    /// - On failure, responds with [`SdkError<DeleteKeyPairError>`](crate::operation::delete_key_pair::DeleteKeyPairError)
    pub fn delete_key_pair(
        &self,
    ) -> crate::operation::delete_key_pair::builders::DeleteKeyPairFluentBuilder {
        crate::operation::delete_key_pair::builders::DeleteKeyPairFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
