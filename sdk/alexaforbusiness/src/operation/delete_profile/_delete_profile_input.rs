// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteProfileInput {
    /// <p>The ARN of the room profile to delete. Required.</p>
    #[doc(hidden)]
    pub profile_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteProfileInput {
    /// <p>The ARN of the room profile to delete. Required.</p>
    pub fn profile_arn(&self) -> ::std::option::Option<&str> {
        self.profile_arn.as_deref()
    }
}
impl DeleteProfileInput {
    /// Creates a new builder-style object to manufacture [`DeleteProfileInput`](crate::operation::delete_profile::DeleteProfileInput).
    pub fn builder() -> crate::operation::delete_profile::builders::DeleteProfileInputBuilder {
        crate::operation::delete_profile::builders::DeleteProfileInputBuilder::default()
    }
}

/// A builder for [`DeleteProfileInput`](crate::operation::delete_profile::DeleteProfileInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteProfileInputBuilder {
    pub(crate) profile_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteProfileInputBuilder {
    /// <p>The ARN of the room profile to delete. Required.</p>
    pub fn profile_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.profile_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the room profile to delete. Required.</p>
    pub fn set_profile_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.profile_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteProfileInput`](crate::operation::delete_profile::DeleteProfileInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_profile::DeleteProfileInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_profile::DeleteProfileInput {
            profile_arn: self.profile_arn,
        })
    }
}
