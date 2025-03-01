// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RejectGrantInput {
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    #[doc(hidden)]
    pub grant_arn: ::std::option::Option<::std::string::String>,
}
impl RejectGrantInput {
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    pub fn grant_arn(&self) -> ::std::option::Option<&str> {
        self.grant_arn.as_deref()
    }
}
impl RejectGrantInput {
    /// Creates a new builder-style object to manufacture [`RejectGrantInput`](crate::operation::reject_grant::RejectGrantInput).
    pub fn builder() -> crate::operation::reject_grant::builders::RejectGrantInputBuilder {
        crate::operation::reject_grant::builders::RejectGrantInputBuilder::default()
    }
}

/// A builder for [`RejectGrantInput`](crate::operation::reject_grant::RejectGrantInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RejectGrantInputBuilder {
    pub(crate) grant_arn: ::std::option::Option<::std::string::String>,
}
impl RejectGrantInputBuilder {
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    pub fn grant_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.grant_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of the grant.</p>
    pub fn set_grant_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.grant_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`RejectGrantInput`](crate::operation::reject_grant::RejectGrantInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::reject_grant::RejectGrantInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::reject_grant::RejectGrantInput {
            grant_arn: self.grant_arn,
        })
    }
}
