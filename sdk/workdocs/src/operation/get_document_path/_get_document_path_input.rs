// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct GetDocumentPathInput {
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    #[doc(hidden)]
    pub authentication_token: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the document.</p>
    #[doc(hidden)]
    pub document_id: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of levels in the hierarchy to return.</p>
    #[doc(hidden)]
    pub limit: ::std::option::Option<i32>,
    /// <p>A comma-separated list of values. Specify <code>NAME</code> to include the names of the parent folders.</p>
    #[doc(hidden)]
    pub fields: ::std::option::Option<::std::string::String>,
    /// <p>This value is not supported.</p>
    #[doc(hidden)]
    pub marker: ::std::option::Option<::std::string::String>,
}
impl GetDocumentPathInput {
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn authentication_token(&self) -> ::std::option::Option<&str> {
        self.authentication_token.as_deref()
    }
    /// <p>The ID of the document.</p>
    pub fn document_id(&self) -> ::std::option::Option<&str> {
        self.document_id.as_deref()
    }
    /// <p>The maximum number of levels in the hierarchy to return.</p>
    pub fn limit(&self) -> ::std::option::Option<i32> {
        self.limit
    }
    /// <p>A comma-separated list of values. Specify <code>NAME</code> to include the names of the parent folders.</p>
    pub fn fields(&self) -> ::std::option::Option<&str> {
        self.fields.as_deref()
    }
    /// <p>This value is not supported.</p>
    pub fn marker(&self) -> ::std::option::Option<&str> {
        self.marker.as_deref()
    }
}
impl ::std::fmt::Debug for GetDocumentPathInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GetDocumentPathInput");
        formatter.field("authentication_token", &"*** Sensitive Data Redacted ***");
        formatter.field("document_id", &self.document_id);
        formatter.field("limit", &self.limit);
        formatter.field("fields", &self.fields);
        formatter.field("marker", &self.marker);
        formatter.finish()
    }
}
impl GetDocumentPathInput {
    /// Creates a new builder-style object to manufacture [`GetDocumentPathInput`](crate::operation::get_document_path::GetDocumentPathInput).
    pub fn builder() -> crate::operation::get_document_path::builders::GetDocumentPathInputBuilder {
        crate::operation::get_document_path::builders::GetDocumentPathInputBuilder::default()
    }
}

/// A builder for [`GetDocumentPathInput`](crate::operation::get_document_path::GetDocumentPathInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct GetDocumentPathInputBuilder {
    pub(crate) authentication_token: ::std::option::Option<::std::string::String>,
    pub(crate) document_id: ::std::option::Option<::std::string::String>,
    pub(crate) limit: ::std::option::Option<i32>,
    pub(crate) fields: ::std::option::Option<::std::string::String>,
    pub(crate) marker: ::std::option::Option<::std::string::String>,
}
impl GetDocumentPathInputBuilder {
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn authentication_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.authentication_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn set_authentication_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.authentication_token = input;
        self
    }
    /// <p>The ID of the document.</p>
    pub fn document_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.document_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the document.</p>
    pub fn set_document_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.document_id = input;
        self
    }
    /// <p>The maximum number of levels in the hierarchy to return.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of levels in the hierarchy to return.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// <p>A comma-separated list of values. Specify <code>NAME</code> to include the names of the parent folders.</p>
    pub fn fields(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fields = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A comma-separated list of values. Specify <code>NAME</code> to include the names of the parent folders.</p>
    pub fn set_fields(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fields = input;
        self
    }
    /// <p>This value is not supported.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>This value is not supported.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// Consumes the builder and constructs a [`GetDocumentPathInput`](crate::operation::get_document_path::GetDocumentPathInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_document_path::GetDocumentPathInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_document_path::GetDocumentPathInput {
            authentication_token: self.authentication_token,
            document_id: self.document_id,
            limit: self.limit,
            fields: self.fields,
            marker: self.marker,
        })
    }
}
impl ::std::fmt::Debug for GetDocumentPathInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GetDocumentPathInputBuilder");
        formatter.field("authentication_token", &"*** Sensitive Data Redacted ***");
        formatter.field("document_id", &self.document_id);
        formatter.field("limit", &self.limit);
        formatter.field("fields", &self.fields);
        formatter.field("marker", &self.marker);
        formatter.finish()
    }
}
