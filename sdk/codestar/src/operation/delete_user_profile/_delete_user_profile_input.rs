// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteUserProfileInput {
    /// <p>The Amazon Resource Name (ARN) of the user to delete from AWS CodeStar.</p>
    #[doc(hidden)]
    pub user_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteUserProfileInput {
    /// <p>The Amazon Resource Name (ARN) of the user to delete from AWS CodeStar.</p>
    pub fn user_arn(&self) -> ::std::option::Option<&str> {
        self.user_arn.as_deref()
    }
}
impl DeleteUserProfileInput {
    /// Creates a new builder-style object to manufacture [`DeleteUserProfileInput`](crate::operation::delete_user_profile::DeleteUserProfileInput).
    pub fn builder(
    ) -> crate::operation::delete_user_profile::builders::DeleteUserProfileInputBuilder {
        crate::operation::delete_user_profile::builders::DeleteUserProfileInputBuilder::default()
    }
}

/// A builder for [`DeleteUserProfileInput`](crate::operation::delete_user_profile::DeleteUserProfileInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteUserProfileInputBuilder {
    pub(crate) user_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteUserProfileInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the user to delete from AWS CodeStar.</p>
    pub fn user_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the user to delete from AWS CodeStar.</p>
    pub fn set_user_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteUserProfileInput`](crate::operation::delete_user_profile::DeleteUserProfileInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_user_profile::DeleteUserProfileInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_user_profile::DeleteUserProfileInput {
                user_arn: self.user_arn,
            },
        )
    }
}
