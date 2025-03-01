// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTagsOutput {
    /// <p>A list of resource tags.</p>
    #[doc(hidden)]
    pub resource_tag_list: ::std::option::Option<::std::vec::Vec<crate::types::ResourceTag>>,
    /// <p>Reserved for future use.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTagsOutput {
    /// <p>A list of resource tags.</p>
    pub fn resource_tag_list(&self) -> ::std::option::Option<&[crate::types::ResourceTag]> {
        self.resource_tag_list.as_deref()
    }
    /// <p>Reserved for future use.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListTagsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTagsOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsOutput`](crate::operation::list_tags::ListTagsOutput).
    pub fn builder() -> crate::operation::list_tags::builders::ListTagsOutputBuilder {
        crate::operation::list_tags::builders::ListTagsOutputBuilder::default()
    }
}

/// A builder for [`ListTagsOutput`](crate::operation::list_tags::ListTagsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListTagsOutputBuilder {
    pub(crate) resource_tag_list: ::std::option::Option<::std::vec::Vec<crate::types::ResourceTag>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTagsOutputBuilder {
    /// Appends an item to `resource_tag_list`.
    ///
    /// To override the contents of this collection use [`set_resource_tag_list`](Self::set_resource_tag_list).
    ///
    /// <p>A list of resource tags.</p>
    pub fn resource_tag_list(mut self, input: crate::types::ResourceTag) -> Self {
        let mut v = self.resource_tag_list.unwrap_or_default();
        v.push(input);
        self.resource_tag_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of resource tags.</p>
    pub fn set_resource_tag_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ResourceTag>>,
    ) -> Self {
        self.resource_tag_list = input;
        self
    }
    /// <p>Reserved for future use.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Reserved for future use.</p>
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
    /// Consumes the builder and constructs a [`ListTagsOutput`](crate::operation::list_tags::ListTagsOutput).
    pub fn build(self) -> crate::operation::list_tags::ListTagsOutput {
        crate::operation::list_tags::ListTagsOutput {
            resource_tag_list: self.resource_tag_list,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
