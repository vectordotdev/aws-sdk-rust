// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the request to delete a user.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct DeleteUserInput {
    /// <p>A valid access token that Amazon Cognito issued to the user whose user profile you want to delete.</p>
    #[doc(hidden)]
    pub access_token: ::std::option::Option<::std::string::String>,
}
impl DeleteUserInput {
    /// <p>A valid access token that Amazon Cognito issued to the user whose user profile you want to delete.</p>
    pub fn access_token(&self) -> ::std::option::Option<&str> {
        self.access_token.as_deref()
    }
}
impl ::std::fmt::Debug for DeleteUserInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteUserInput");
        formatter.field("access_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl DeleteUserInput {
    /// Creates a new builder-style object to manufacture [`DeleteUserInput`](crate::operation::delete_user::DeleteUserInput).
    pub fn builder() -> crate::operation::delete_user::builders::DeleteUserInputBuilder {
        crate::operation::delete_user::builders::DeleteUserInputBuilder::default()
    }
}

/// A builder for [`DeleteUserInput`](crate::operation::delete_user::DeleteUserInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct DeleteUserInputBuilder {
    pub(crate) access_token: ::std::option::Option<::std::string::String>,
}
impl DeleteUserInputBuilder {
    /// <p>A valid access token that Amazon Cognito issued to the user whose user profile you want to delete.</p>
    pub fn access_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.access_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A valid access token that Amazon Cognito issued to the user whose user profile you want to delete.</p>
    pub fn set_access_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.access_token = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteUserInput`](crate::operation::delete_user::DeleteUserInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_user::DeleteUserInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_user::DeleteUserInput {
            access_token: self.access_token,
        })
    }
}
impl ::std::fmt::Debug for DeleteUserInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteUserInputBuilder");
        formatter.field("access_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
