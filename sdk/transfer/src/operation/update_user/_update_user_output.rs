// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> <code>UpdateUserResponse</code> returns the user name and identifier for the request to update a user's properties.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateUserOutput {
    /// <p>A system-assigned unique identifier for a Transfer Family server instance that the account is assigned to.</p>
    #[doc(hidden)]
    pub server_id: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for a user that is assigned to a server instance that was specified in the request.</p>
    #[doc(hidden)]
    pub user_name: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateUserOutput {
    /// <p>A system-assigned unique identifier for a Transfer Family server instance that the account is assigned to.</p>
    pub fn server_id(&self) -> ::std::option::Option<&str> {
        self.server_id.as_deref()
    }
    /// <p>The unique identifier for a user that is assigned to a server instance that was specified in the request.</p>
    pub fn user_name(&self) -> ::std::option::Option<&str> {
        self.user_name.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for UpdateUserOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateUserOutput {
    /// Creates a new builder-style object to manufacture [`UpdateUserOutput`](crate::operation::update_user::UpdateUserOutput).
    pub fn builder() -> crate::operation::update_user::builders::UpdateUserOutputBuilder {
        crate::operation::update_user::builders::UpdateUserOutputBuilder::default()
    }
}

/// A builder for [`UpdateUserOutput`](crate::operation::update_user::UpdateUserOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateUserOutputBuilder {
    pub(crate) server_id: ::std::option::Option<::std::string::String>,
    pub(crate) user_name: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateUserOutputBuilder {
    /// <p>A system-assigned unique identifier for a Transfer Family server instance that the account is assigned to.</p>
    pub fn server_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.server_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A system-assigned unique identifier for a Transfer Family server instance that the account is assigned to.</p>
    pub fn set_server_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.server_id = input;
        self
    }
    /// <p>The unique identifier for a user that is assigned to a server instance that was specified in the request.</p>
    pub fn user_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for a user that is assigned to a server instance that was specified in the request.</p>
    pub fn set_user_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_name = input;
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
    /// Consumes the builder and constructs a [`UpdateUserOutput`](crate::operation::update_user::UpdateUserOutput).
    pub fn build(self) -> crate::operation::update_user::UpdateUserOutput {
        crate::operation::update_user::UpdateUserOutput {
            server_id: self.server_id,
            user_name: self.user_name,
            _request_id: self._request_id,
        }
    }
}
