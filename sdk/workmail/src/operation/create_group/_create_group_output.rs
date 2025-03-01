// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateGroupOutput {
    /// <p>The identifier of the group.</p>
    #[doc(hidden)]
    pub group_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateGroupOutput {
    /// <p>The identifier of the group.</p>
    pub fn group_id(&self) -> ::std::option::Option<&str> {
        self.group_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateGroupOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateGroupOutput {
    /// Creates a new builder-style object to manufacture [`CreateGroupOutput`](crate::operation::create_group::CreateGroupOutput).
    pub fn builder() -> crate::operation::create_group::builders::CreateGroupOutputBuilder {
        crate::operation::create_group::builders::CreateGroupOutputBuilder::default()
    }
}

/// A builder for [`CreateGroupOutput`](crate::operation::create_group::CreateGroupOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateGroupOutputBuilder {
    pub(crate) group_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateGroupOutputBuilder {
    /// <p>The identifier of the group.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the group.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_id = input;
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
    /// Consumes the builder and constructs a [`CreateGroupOutput`](crate::operation::create_group::CreateGroupOutput).
    pub fn build(self) -> crate::operation::create_group::CreateGroupOutput {
        crate::operation::create_group::CreateGroupOutput {
            group_id: self.group_id,
            _request_id: self._request_id,
        }
    }
}
