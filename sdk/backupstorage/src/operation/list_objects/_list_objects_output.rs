// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListObjectsOutput {
    /// Object list
    #[doc(hidden)]
    pub object_list: ::std::option::Option<::std::vec::Vec<crate::types::BackupObject>>,
    /// Pagination token
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListObjectsOutput {
    /// Object list
    pub fn object_list(&self) -> ::std::option::Option<&[crate::types::BackupObject]> {
        self.object_list.as_deref()
    }
    /// Pagination token
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListObjectsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListObjectsOutput {
    /// Creates a new builder-style object to manufacture [`ListObjectsOutput`](crate::operation::list_objects::ListObjectsOutput).
    pub fn builder() -> crate::operation::list_objects::builders::ListObjectsOutputBuilder {
        crate::operation::list_objects::builders::ListObjectsOutputBuilder::default()
    }
}

/// A builder for [`ListObjectsOutput`](crate::operation::list_objects::ListObjectsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListObjectsOutputBuilder {
    pub(crate) object_list: ::std::option::Option<::std::vec::Vec<crate::types::BackupObject>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListObjectsOutputBuilder {
    /// Appends an item to `object_list`.
    ///
    /// To override the contents of this collection use [`set_object_list`](Self::set_object_list).
    ///
    /// Object list
    pub fn object_list(mut self, input: crate::types::BackupObject) -> Self {
        let mut v = self.object_list.unwrap_or_default();
        v.push(input);
        self.object_list = ::std::option::Option::Some(v);
        self
    }
    /// Object list
    pub fn set_object_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::BackupObject>>,
    ) -> Self {
        self.object_list = input;
        self
    }
    /// Pagination token
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// Pagination token
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
    /// Consumes the builder and constructs a [`ListObjectsOutput`](crate::operation::list_objects::ListObjectsOutput).
    pub fn build(self) -> crate::operation::list_objects::ListObjectsOutput {
        crate::operation::list_objects::ListObjectsOutput {
            object_list: self.object_list,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
