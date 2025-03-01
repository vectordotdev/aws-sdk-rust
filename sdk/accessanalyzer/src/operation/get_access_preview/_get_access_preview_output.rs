// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAccessPreviewOutput {
    /// <p>An object that contains information about the access preview.</p>
    #[doc(hidden)]
    pub access_preview: ::std::option::Option<crate::types::AccessPreview>,
    _request_id: Option<String>,
}
impl GetAccessPreviewOutput {
    /// <p>An object that contains information about the access preview.</p>
    pub fn access_preview(&self) -> ::std::option::Option<&crate::types::AccessPreview> {
        self.access_preview.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetAccessPreviewOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetAccessPreviewOutput {
    /// Creates a new builder-style object to manufacture [`GetAccessPreviewOutput`](crate::operation::get_access_preview::GetAccessPreviewOutput).
    pub fn builder() -> crate::operation::get_access_preview::builders::GetAccessPreviewOutputBuilder
    {
        crate::operation::get_access_preview::builders::GetAccessPreviewOutputBuilder::default()
    }
}

/// A builder for [`GetAccessPreviewOutput`](crate::operation::get_access_preview::GetAccessPreviewOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetAccessPreviewOutputBuilder {
    pub(crate) access_preview: ::std::option::Option<crate::types::AccessPreview>,
    _request_id: Option<String>,
}
impl GetAccessPreviewOutputBuilder {
    /// <p>An object that contains information about the access preview.</p>
    pub fn access_preview(mut self, input: crate::types::AccessPreview) -> Self {
        self.access_preview = ::std::option::Option::Some(input);
        self
    }
    /// <p>An object that contains information about the access preview.</p>
    pub fn set_access_preview(
        mut self,
        input: ::std::option::Option<crate::types::AccessPreview>,
    ) -> Self {
        self.access_preview = input;
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
    /// Consumes the builder and constructs a [`GetAccessPreviewOutput`](crate::operation::get_access_preview::GetAccessPreviewOutput).
    pub fn build(self) -> crate::operation::get_access_preview::GetAccessPreviewOutput {
        crate::operation::get_access_preview::GetAccessPreviewOutput {
            access_preview: self.access_preview,
            _request_id: self._request_id,
        }
    }
}
