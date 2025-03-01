// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelRetrieval`](crate::operation::cancel_retrieval::builders::CancelRetrievalFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl ::std::convert::Into<String>)`](crate::operation::cancel_retrieval::builders::CancelRetrievalFluentBuilder::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::operation::cancel_retrieval::builders::CancelRetrievalFluentBuilder::set_gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    ///   - [`tape_arn(impl ::std::convert::Into<String>)`](crate::operation::cancel_retrieval::builders::CancelRetrievalFluentBuilder::tape_arn) / [`set_tape_arn(Option<String>)`](crate::operation::cancel_retrieval::builders::CancelRetrievalFluentBuilder::set_tape_arn): <p>The Amazon Resource Name (ARN) of the virtual tape you want to cancel retrieval for.</p>
    /// - On success, responds with [`CancelRetrievalOutput`](crate::operation::cancel_retrieval::CancelRetrievalOutput) with field(s):
    ///   - [`tape_arn(Option<String>)`](crate::operation::cancel_retrieval::CancelRetrievalOutput::tape_arn): <p>The Amazon Resource Name (ARN) of the virtual tape for which retrieval was canceled.</p>
    /// - On failure, responds with [`SdkError<CancelRetrievalError>`](crate::operation::cancel_retrieval::CancelRetrievalError)
    pub fn cancel_retrieval(
        &self,
    ) -> crate::operation::cancel_retrieval::builders::CancelRetrievalFluentBuilder {
        crate::operation::cancel_retrieval::builders::CancelRetrievalFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
