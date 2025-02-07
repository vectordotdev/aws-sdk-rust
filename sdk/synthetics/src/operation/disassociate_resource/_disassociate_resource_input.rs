// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateResourceInput {
    /// <p>Specifies the group. You can specify the group name, the ARN, or the group ID as the <code>GroupIdentifier</code>.</p>
    #[doc(hidden)]
    pub group_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the canary that you want to remove from the specified group.</p>
    #[doc(hidden)]
    pub resource_arn: ::std::option::Option<::std::string::String>,
}
impl DisassociateResourceInput {
    /// <p>Specifies the group. You can specify the group name, the ARN, or the group ID as the <code>GroupIdentifier</code>.</p>
    pub fn group_identifier(&self) -> ::std::option::Option<&str> {
        self.group_identifier.as_deref()
    }
    /// <p>The ARN of the canary that you want to remove from the specified group.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
}
impl DisassociateResourceInput {
    /// Creates a new builder-style object to manufacture [`DisassociateResourceInput`](crate::operation::disassociate_resource::DisassociateResourceInput).
    pub fn builder(
    ) -> crate::operation::disassociate_resource::builders::DisassociateResourceInputBuilder {
        crate::operation::disassociate_resource::builders::DisassociateResourceInputBuilder::default(
        )
    }
}

/// A builder for [`DisassociateResourceInput`](crate::operation::disassociate_resource::DisassociateResourceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DisassociateResourceInputBuilder {
    pub(crate) group_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
}
impl DisassociateResourceInputBuilder {
    /// <p>Specifies the group. You can specify the group name, the ARN, or the group ID as the <code>GroupIdentifier</code>.</p>
    pub fn group_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.group_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the group. You can specify the group name, the ARN, or the group ID as the <code>GroupIdentifier</code>.</p>
    pub fn set_group_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.group_identifier = input;
        self
    }
    /// <p>The ARN of the canary that you want to remove from the specified group.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the canary that you want to remove from the specified group.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DisassociateResourceInput`](crate::operation::disassociate_resource::DisassociateResourceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disassociate_resource::DisassociateResourceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::disassociate_resource::DisassociateResourceInput {
                group_identifier: self.group_identifier,
                resource_arn: self.resource_arn,
            },
        )
    }
}
