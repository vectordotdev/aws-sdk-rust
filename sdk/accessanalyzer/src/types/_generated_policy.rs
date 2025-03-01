// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the text for the generated policy.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GeneratedPolicy {
    /// <p>The text to use as the content for the new policy. The policy is created using the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreatePolicy.html">CreatePolicy</a> action.</p>
    #[doc(hidden)]
    pub policy: ::std::option::Option<::std::string::String>,
}
impl GeneratedPolicy {
    /// <p>The text to use as the content for the new policy. The policy is created using the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreatePolicy.html">CreatePolicy</a> action.</p>
    pub fn policy(&self) -> ::std::option::Option<&str> {
        self.policy.as_deref()
    }
}
impl GeneratedPolicy {
    /// Creates a new builder-style object to manufacture [`GeneratedPolicy`](crate::types::GeneratedPolicy).
    pub fn builder() -> crate::types::builders::GeneratedPolicyBuilder {
        crate::types::builders::GeneratedPolicyBuilder::default()
    }
}

/// A builder for [`GeneratedPolicy`](crate::types::GeneratedPolicy).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GeneratedPolicyBuilder {
    pub(crate) policy: ::std::option::Option<::std::string::String>,
}
impl GeneratedPolicyBuilder {
    /// <p>The text to use as the content for the new policy. The policy is created using the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreatePolicy.html">CreatePolicy</a> action.</p>
    pub fn policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The text to use as the content for the new policy. The policy is created using the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreatePolicy.html">CreatePolicy</a> action.</p>
    pub fn set_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy = input;
        self
    }
    /// Consumes the builder and constructs a [`GeneratedPolicy`](crate::types::GeneratedPolicy).
    pub fn build(self) -> crate::types::GeneratedPolicy {
        crate::types::GeneratedPolicy {
            policy: self.policy,
        }
    }
}
