// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetRolePolicyInput {
    /// <p>The name of the role associated with the policy.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub role_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the policy document to get.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub policy_name: ::std::option::Option<::std::string::String>,
}
impl GetRolePolicyInput {
    /// <p>The name of the role associated with the policy.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn role_name(&self) -> ::std::option::Option<&str> {
        self.role_name.as_deref()
    }
    /// <p>The name of the policy document to get.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn policy_name(&self) -> ::std::option::Option<&str> {
        self.policy_name.as_deref()
    }
}
impl GetRolePolicyInput {
    /// Creates a new builder-style object to manufacture [`GetRolePolicyInput`](crate::operation::get_role_policy::GetRolePolicyInput).
    pub fn builder() -> crate::operation::get_role_policy::builders::GetRolePolicyInputBuilder {
        crate::operation::get_role_policy::builders::GetRolePolicyInputBuilder::default()
    }
}

/// A builder for [`GetRolePolicyInput`](crate::operation::get_role_policy::GetRolePolicyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetRolePolicyInputBuilder {
    pub(crate) role_name: ::std::option::Option<::std::string::String>,
    pub(crate) policy_name: ::std::option::Option<::std::string::String>,
}
impl GetRolePolicyInputBuilder {
    /// <p>The name of the role associated with the policy.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn role_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the role associated with the policy.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_role_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_name = input;
        self
    }
    /// <p>The name of the policy document to get.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the policy document to get.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_name = input;
        self
    }
    /// Consumes the builder and constructs a [`GetRolePolicyInput`](crate::operation::get_role_policy::GetRolePolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_role_policy::GetRolePolicyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_role_policy::GetRolePolicyInput {
            role_name: self.role_name,
            policy_name: self.policy_name,
        })
    }
}
