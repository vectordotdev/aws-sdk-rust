// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListReportsOutput {
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p> The list of returned ARNs for the reports in the current Amazon Web Services account. </p>
    #[doc(hidden)]
    pub reports: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    _request_id: Option<String>,
}
impl ListReportsOutput {
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p> The list of returned ARNs for the reports in the current Amazon Web Services account. </p>
    pub fn reports(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.reports.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListReportsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListReportsOutput {
    /// Creates a new builder-style object to manufacture [`ListReportsOutput`](crate::operation::list_reports::ListReportsOutput).
    pub fn builder() -> crate::operation::list_reports::builders::ListReportsOutputBuilder {
        crate::operation::list_reports::builders::ListReportsOutputBuilder::default()
    }
}

/// A builder for [`ListReportsOutput`](crate::operation::list_reports::ListReportsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListReportsOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) reports: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    _request_id: Option<String>,
}
impl ListReportsOutputBuilder {
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> During a previous call, the maximum number of items that can be returned is the value specified in <code>maxResults</code>. If there more items in the list, then a unique string called a <i>nextToken</i> is returned. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `reports`.
    ///
    /// To override the contents of this collection use [`set_reports`](Self::set_reports).
    ///
    /// <p> The list of returned ARNs for the reports in the current Amazon Web Services account. </p>
    pub fn reports(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.reports.unwrap_or_default();
        v.push(input.into());
        self.reports = ::std::option::Option::Some(v);
        self
    }
    /// <p> The list of returned ARNs for the reports in the current Amazon Web Services account. </p>
    pub fn set_reports(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.reports = input;
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
    /// Consumes the builder and constructs a [`ListReportsOutput`](crate::operation::list_reports::ListReportsOutput).
    pub fn build(self) -> crate::operation::list_reports::ListReportsOutput {
        crate::operation::list_reports::ListReportsOutput {
            next_token: self.next_token,
            reports: self.reports,
            _request_id: self._request_id,
        }
    }
}
