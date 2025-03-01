// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchReadOutput {
    /// <p>A list of all the responses for each batch read.</p>
    #[doc(hidden)]
    pub responses: ::std::option::Option<::std::vec::Vec<crate::types::BatchReadOperationResponse>>,
    _request_id: Option<String>,
}
impl BatchReadOutput {
    /// <p>A list of all the responses for each batch read.</p>
    pub fn responses(&self) -> ::std::option::Option<&[crate::types::BatchReadOperationResponse]> {
        self.responses.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for BatchReadOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchReadOutput {
    /// Creates a new builder-style object to manufacture [`BatchReadOutput`](crate::operation::batch_read::BatchReadOutput).
    pub fn builder() -> crate::operation::batch_read::builders::BatchReadOutputBuilder {
        crate::operation::batch_read::builders::BatchReadOutputBuilder::default()
    }
}

/// A builder for [`BatchReadOutput`](crate::operation::batch_read::BatchReadOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchReadOutputBuilder {
    pub(crate) responses:
        ::std::option::Option<::std::vec::Vec<crate::types::BatchReadOperationResponse>>,
    _request_id: Option<String>,
}
impl BatchReadOutputBuilder {
    /// Appends an item to `responses`.
    ///
    /// To override the contents of this collection use [`set_responses`](Self::set_responses).
    ///
    /// <p>A list of all the responses for each batch read.</p>
    pub fn responses(mut self, input: crate::types::BatchReadOperationResponse) -> Self {
        let mut v = self.responses.unwrap_or_default();
        v.push(input);
        self.responses = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of all the responses for each batch read.</p>
    pub fn set_responses(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::BatchReadOperationResponse>>,
    ) -> Self {
        self.responses = input;
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
    /// Consumes the builder and constructs a [`BatchReadOutput`](crate::operation::batch_read::BatchReadOutput).
    pub fn build(self) -> crate::operation::batch_read::BatchReadOutput {
        crate::operation::batch_read::BatchReadOutput {
            responses: self.responses,
            _request_id: self._request_id,
        }
    }
}
