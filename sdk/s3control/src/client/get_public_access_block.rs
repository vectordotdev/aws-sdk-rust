// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetPublicAccessBlock`](crate::operation::get_public_access_block::builders::GetPublicAccessBlockFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::get_public_access_block::builders::GetPublicAccessBlockFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::get_public_access_block::builders::GetPublicAccessBlockFluentBuilder::set_account_id): <p>The account ID for the Amazon Web Services account whose <code>PublicAccessBlock</code> configuration you want to retrieve.</p>
    /// - On success, responds with [`GetPublicAccessBlockOutput`](crate::operation::get_public_access_block::GetPublicAccessBlockOutput) with field(s):
    ///   - [`public_access_block_configuration(Option<PublicAccessBlockConfiguration>)`](crate::operation::get_public_access_block::GetPublicAccessBlockOutput::public_access_block_configuration): <p>The <code>PublicAccessBlock</code> configuration currently in effect for this Amazon Web Services account.</p>
    /// - On failure, responds with [`SdkError<GetPublicAccessBlockError>`](crate::operation::get_public_access_block::GetPublicAccessBlockError)
    pub fn get_public_access_block(
        &self,
    ) -> crate::operation::get_public_access_block::builders::GetPublicAccessBlockFluentBuilder
    {
        crate::operation::get_public_access_block::builders::GetPublicAccessBlockFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
