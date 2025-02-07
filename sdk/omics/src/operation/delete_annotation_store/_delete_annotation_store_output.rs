// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteAnnotationStoreOutput {
    /// <p>The store's status.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::StoreStatus>,
    _request_id: Option<String>,
}
impl DeleteAnnotationStoreOutput {
    /// <p>The store's status.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::StoreStatus> {
        self.status.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DeleteAnnotationStoreOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteAnnotationStoreOutput {
    /// Creates a new builder-style object to manufacture [`DeleteAnnotationStoreOutput`](crate::operation::delete_annotation_store::DeleteAnnotationStoreOutput).
    pub fn builder(
    ) -> crate::operation::delete_annotation_store::builders::DeleteAnnotationStoreOutputBuilder
    {
        crate::operation::delete_annotation_store::builders::DeleteAnnotationStoreOutputBuilder::default()
    }
}

/// A builder for [`DeleteAnnotationStoreOutput`](crate::operation::delete_annotation_store::DeleteAnnotationStoreOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteAnnotationStoreOutputBuilder {
    pub(crate) status: ::std::option::Option<crate::types::StoreStatus>,
    _request_id: Option<String>,
}
impl DeleteAnnotationStoreOutputBuilder {
    /// <p>The store's status.</p>
    pub fn status(mut self, input: crate::types::StoreStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The store's status.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::StoreStatus>) -> Self {
        self.status = input;
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
    /// Consumes the builder and constructs a [`DeleteAnnotationStoreOutput`](crate::operation::delete_annotation_store::DeleteAnnotationStoreOutput).
    pub fn build(self) -> crate::operation::delete_annotation_store::DeleteAnnotationStoreOutput {
        crate::operation::delete_annotation_store::DeleteAnnotationStoreOutput {
            status: self.status,
            _request_id: self._request_id,
        }
    }
}
