// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteRolePolicyInput {
    /// <p>The name (friendly name, not ARN) identifying the role that the policy is embedded in.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub role_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the inline policy to delete from the specified IAM role.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub policy_name: ::std::option::Option<::std::string::String>,
}
impl DeleteRolePolicyInput {
    /// <p>The name (friendly name, not ARN) identifying the role that the policy is embedded in.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn role_name(&self) -> ::std::option::Option<&str> {
        self.role_name.as_deref()
    }
    /// <p>The name of the inline policy to delete from the specified IAM role.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn policy_name(&self) -> ::std::option::Option<&str> {
        self.policy_name.as_deref()
    }
}
impl DeleteRolePolicyInput {
    /// Creates a new builder-style object to manufacture [`DeleteRolePolicyInput`](crate::operation::delete_role_policy::DeleteRolePolicyInput).
    pub fn builder() -> crate::operation::delete_role_policy::builders::DeleteRolePolicyInputBuilder
    {
        crate::operation::delete_role_policy::builders::DeleteRolePolicyInputBuilder::default()
    }
}

/// A builder for [`DeleteRolePolicyInput`](crate::operation::delete_role_policy::DeleteRolePolicyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteRolePolicyInputBuilder {
    pub(crate) role_name: ::std::option::Option<::std::string::String>,
    pub(crate) policy_name: ::std::option::Option<::std::string::String>,
}
impl DeleteRolePolicyInputBuilder {
    /// <p>The name (friendly name, not ARN) identifying the role that the policy is embedded in.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn role_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name (friendly name, not ARN) identifying the role that the policy is embedded in.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_role_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_name = input;
        self
    }
    /// <p>The name of the inline policy to delete from the specified IAM role.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the inline policy to delete from the specified IAM role.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteRolePolicyInput`](crate::operation::delete_role_policy::DeleteRolePolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_role_policy::DeleteRolePolicyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_role_policy::DeleteRolePolicyInput {
                role_name: self.role_name,
                policy_name: self.policy_name,
            },
        )
    }
}
