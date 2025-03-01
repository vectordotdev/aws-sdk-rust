// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetTableVersionOutput {
    /// <p>The requested table version.</p>
    #[doc(hidden)]
    pub table_version: ::std::option::Option<crate::types::TableVersion>,
    _request_id: Option<String>,
}
impl GetTableVersionOutput {
    /// <p>The requested table version.</p>
    pub fn table_version(&self) -> ::std::option::Option<&crate::types::TableVersion> {
        self.table_version.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetTableVersionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetTableVersionOutput {
    /// Creates a new builder-style object to manufacture [`GetTableVersionOutput`](crate::operation::get_table_version::GetTableVersionOutput).
    pub fn builder() -> crate::operation::get_table_version::builders::GetTableVersionOutputBuilder
    {
        crate::operation::get_table_version::builders::GetTableVersionOutputBuilder::default()
    }
}

/// A builder for [`GetTableVersionOutput`](crate::operation::get_table_version::GetTableVersionOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetTableVersionOutputBuilder {
    pub(crate) table_version: ::std::option::Option<crate::types::TableVersion>,
    _request_id: Option<String>,
}
impl GetTableVersionOutputBuilder {
    /// <p>The requested table version.</p>
    pub fn table_version(mut self, input: crate::types::TableVersion) -> Self {
        self.table_version = ::std::option::Option::Some(input);
        self
    }
    /// <p>The requested table version.</p>
    pub fn set_table_version(
        mut self,
        input: ::std::option::Option<crate::types::TableVersion>,
    ) -> Self {
        self.table_version = input;
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
    /// Consumes the builder and constructs a [`GetTableVersionOutput`](crate::operation::get_table_version::GetTableVersionOutput).
    pub fn build(self) -> crate::operation::get_table_version::GetTableVersionOutput {
        crate::operation::get_table_version::GetTableVersionOutput {
            table_version: self.table_version,
            _request_id: self._request_id,
        }
    }
}
