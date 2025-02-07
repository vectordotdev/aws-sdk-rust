// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListOfferingTransactions`](crate::operation::list_offering_transactions::builders::ListOfferingTransactionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_offering_transactions::builders::ListOfferingTransactionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_offering_transactions::builders::ListOfferingTransactionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_offering_transactions::builders::ListOfferingTransactionsFluentBuilder::set_next_token): <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    /// - On success, responds with [`ListOfferingTransactionsOutput`](crate::operation::list_offering_transactions::ListOfferingTransactionsOutput) with field(s):
    ///   - [`offering_transactions(Option<Vec<OfferingTransaction>>)`](crate::operation::list_offering_transactions::ListOfferingTransactionsOutput::offering_transactions): <p>The audit log of subscriptions you have purchased and modified through AWS Device Farm.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_offering_transactions::ListOfferingTransactionsOutput::next_token): <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    /// - On failure, responds with [`SdkError<ListOfferingTransactionsError>`](crate::operation::list_offering_transactions::ListOfferingTransactionsError)
    pub fn list_offering_transactions(
        &self,
    ) -> crate::operation::list_offering_transactions::builders::ListOfferingTransactionsFluentBuilder
    {
        crate::operation::list_offering_transactions::builders::ListOfferingTransactionsFluentBuilder::new(self.handle.clone())
    }
}
