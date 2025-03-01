// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTransactionsOutput {
    /// <p>A list of transactions. The record for each transaction is a <code>TransactionDescription</code> object.</p>
    #[doc(hidden)]
    pub transactions: ::std::option::Option<::std::vec::Vec<crate::types::TransactionDescription>>,
    /// <p>A continuation token indicating whether additional data is available.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTransactionsOutput {
    /// <p>A list of transactions. The record for each transaction is a <code>TransactionDescription</code> object.</p>
    pub fn transactions(&self) -> ::std::option::Option<&[crate::types::TransactionDescription]> {
        self.transactions.as_deref()
    }
    /// <p>A continuation token indicating whether additional data is available.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListTransactionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTransactionsOutput {
    /// Creates a new builder-style object to manufacture [`ListTransactionsOutput`](crate::operation::list_transactions::ListTransactionsOutput).
    pub fn builder() -> crate::operation::list_transactions::builders::ListTransactionsOutputBuilder
    {
        crate::operation::list_transactions::builders::ListTransactionsOutputBuilder::default()
    }
}

/// A builder for [`ListTransactionsOutput`](crate::operation::list_transactions::ListTransactionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListTransactionsOutputBuilder {
    pub(crate) transactions:
        ::std::option::Option<::std::vec::Vec<crate::types::TransactionDescription>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTransactionsOutputBuilder {
    /// Appends an item to `transactions`.
    ///
    /// To override the contents of this collection use [`set_transactions`](Self::set_transactions).
    ///
    /// <p>A list of transactions. The record for each transaction is a <code>TransactionDescription</code> object.</p>
    pub fn transactions(mut self, input: crate::types::TransactionDescription) -> Self {
        let mut v = self.transactions.unwrap_or_default();
        v.push(input);
        self.transactions = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of transactions. The record for each transaction is a <code>TransactionDescription</code> object.</p>
    pub fn set_transactions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TransactionDescription>>,
    ) -> Self {
        self.transactions = input;
        self
    }
    /// <p>A continuation token indicating whether additional data is available.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A continuation token indicating whether additional data is available.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListTransactionsOutput`](crate::operation::list_transactions::ListTransactionsOutput).
    pub fn build(self) -> crate::operation::list_transactions::ListTransactionsOutput {
        crate::operation::list_transactions::ListTransactionsOutput {
            transactions: self.transactions,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
