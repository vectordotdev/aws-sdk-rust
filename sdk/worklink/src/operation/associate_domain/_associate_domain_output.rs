// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateDomainOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for AssociateDomainOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AssociateDomainOutput {
    /// Creates a new builder-style object to manufacture [`AssociateDomainOutput`](crate::operation::associate_domain::AssociateDomainOutput).
    pub fn builder() -> crate::operation::associate_domain::builders::AssociateDomainOutputBuilder {
        crate::operation::associate_domain::builders::AssociateDomainOutputBuilder::default()
    }
}

/// A builder for [`AssociateDomainOutput`](crate::operation::associate_domain::AssociateDomainOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AssociateDomainOutputBuilder {
    _request_id: Option<String>,
}
impl AssociateDomainOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`AssociateDomainOutput`](crate::operation::associate_domain::AssociateDomainOutput).
    pub fn build(self) -> crate::operation::associate_domain::AssociateDomainOutput {
        crate::operation::associate_domain::AssociateDomainOutput {
            _request_id: self._request_id,
        }
    }
}
