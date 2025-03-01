// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The communications returned by the <code>DescribeCommunications</code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeCommunicationsOutput {
    /// <p>The communications for the case.</p>
    #[doc(hidden)]
    pub communications: ::std::option::Option<::std::vec::Vec<crate::types::Communication>>,
    /// <p>A resumption point for pagination.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeCommunicationsOutput {
    /// <p>The communications for the case.</p>
    pub fn communications(&self) -> ::std::option::Option<&[crate::types::Communication]> {
        self.communications.as_deref()
    }
    /// <p>A resumption point for pagination.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeCommunicationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeCommunicationsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeCommunicationsOutput`](crate::operation::describe_communications::DescribeCommunicationsOutput).
    pub fn builder(
    ) -> crate::operation::describe_communications::builders::DescribeCommunicationsOutputBuilder
    {
        crate::operation::describe_communications::builders::DescribeCommunicationsOutputBuilder::default()
    }
}

/// A builder for [`DescribeCommunicationsOutput`](crate::operation::describe_communications::DescribeCommunicationsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeCommunicationsOutputBuilder {
    pub(crate) communications: ::std::option::Option<::std::vec::Vec<crate::types::Communication>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeCommunicationsOutputBuilder {
    /// Appends an item to `communications`.
    ///
    /// To override the contents of this collection use [`set_communications`](Self::set_communications).
    ///
    /// <p>The communications for the case.</p>
    pub fn communications(mut self, input: crate::types::Communication) -> Self {
        let mut v = self.communications.unwrap_or_default();
        v.push(input);
        self.communications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The communications for the case.</p>
    pub fn set_communications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Communication>>,
    ) -> Self {
        self.communications = input;
        self
    }
    /// <p>A resumption point for pagination.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A resumption point for pagination.</p>
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
    /// Consumes the builder and constructs a [`DescribeCommunicationsOutput`](crate::operation::describe_communications::DescribeCommunicationsOutput).
    pub fn build(self) -> crate::operation::describe_communications::DescribeCommunicationsOutput {
        crate::operation::describe_communications::DescribeCommunicationsOutput {
            communications: self.communications,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
