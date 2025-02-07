// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The input for the GetPolicy operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPolicyInput {
    /// <p>The name of the policy.</p>
    #[doc(hidden)]
    pub policy_name: ::std::option::Option<::std::string::String>,
}
impl GetPolicyInput {
    /// <p>The name of the policy.</p>
    pub fn policy_name(&self) -> ::std::option::Option<&str> {
        self.policy_name.as_deref()
    }
}
impl GetPolicyInput {
    /// Creates a new builder-style object to manufacture [`GetPolicyInput`](crate::operation::get_policy::GetPolicyInput).
    pub fn builder() -> crate::operation::get_policy::builders::GetPolicyInputBuilder {
        crate::operation::get_policy::builders::GetPolicyInputBuilder::default()
    }
}

/// A builder for [`GetPolicyInput`](crate::operation::get_policy::GetPolicyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetPolicyInputBuilder {
    pub(crate) policy_name: ::std::option::Option<::std::string::String>,
}
impl GetPolicyInputBuilder {
    /// <p>The name of the policy.</p>
    pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the policy.</p>
    pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_name = input;
        self
    }
    /// Consumes the builder and constructs a [`GetPolicyInput`](crate::operation::get_policy::GetPolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_policy::GetPolicyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_policy::GetPolicyInput {
            policy_name: self.policy_name,
        })
    }
}
