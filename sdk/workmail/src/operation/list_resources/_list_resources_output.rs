// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListResourcesOutput {
    /// <p>One page of the organization's resource representation.</p>
    #[doc(hidden)]
    pub resources: ::std::option::Option<::std::vec::Vec<crate::types::Resource>>,
    /// <p> The token used to paginate through all the organization's resources. While results are still available, it has an associated value. When the last page is reached, the token is empty.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListResourcesOutput {
    /// <p>One page of the organization's resource representation.</p>
    pub fn resources(&self) -> ::std::option::Option<&[crate::types::Resource]> {
        self.resources.as_deref()
    }
    /// <p> The token used to paginate through all the organization's resources. While results are still available, it has an associated value. When the last page is reached, the token is empty.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListResourcesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListResourcesOutput {
    /// Creates a new builder-style object to manufacture [`ListResourcesOutput`](crate::operation::list_resources::ListResourcesOutput).
    pub fn builder() -> crate::operation::list_resources::builders::ListResourcesOutputBuilder {
        crate::operation::list_resources::builders::ListResourcesOutputBuilder::default()
    }
}

/// A builder for [`ListResourcesOutput`](crate::operation::list_resources::ListResourcesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListResourcesOutputBuilder {
    pub(crate) resources: ::std::option::Option<::std::vec::Vec<crate::types::Resource>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListResourcesOutputBuilder {
    /// Appends an item to `resources`.
    ///
    /// To override the contents of this collection use [`set_resources`](Self::set_resources).
    ///
    /// <p>One page of the organization's resource representation.</p>
    pub fn resources(mut self, input: crate::types::Resource) -> Self {
        let mut v = self.resources.unwrap_or_default();
        v.push(input);
        self.resources = ::std::option::Option::Some(v);
        self
    }
    /// <p>One page of the organization's resource representation.</p>
    pub fn set_resources(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Resource>>,
    ) -> Self {
        self.resources = input;
        self
    }
    /// <p> The token used to paginate through all the organization's resources. While results are still available, it has an associated value. When the last page is reached, the token is empty.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The token used to paginate through all the organization's resources. While results are still available, it has an associated value. When the last page is reached, the token is empty.</p>
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
    /// Consumes the builder and constructs a [`ListResourcesOutput`](crate::operation::list_resources::ListResourcesOutput).
    pub fn build(self) -> crate::operation::list_resources::ListResourcesOutput {
        crate::operation::list_resources::ListResourcesOutput {
            resources: self.resources,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
