// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for <code>UpdateSchedulingPolicy</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateSchedulingPolicyInput {
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy to update.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The fair share policy.</p>
    #[doc(hidden)]
    pub fairshare_policy: ::std::option::Option<crate::types::FairsharePolicy>,
}
impl UpdateSchedulingPolicyInput {
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy to update.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The fair share policy.</p>
    pub fn fairshare_policy(&self) -> ::std::option::Option<&crate::types::FairsharePolicy> {
        self.fairshare_policy.as_ref()
    }
}
impl UpdateSchedulingPolicyInput {
    /// Creates a new builder-style object to manufacture [`UpdateSchedulingPolicyInput`](crate::operation::update_scheduling_policy::UpdateSchedulingPolicyInput).
    pub fn builder(
    ) -> crate::operation::update_scheduling_policy::builders::UpdateSchedulingPolicyInputBuilder
    {
        crate::operation::update_scheduling_policy::builders::UpdateSchedulingPolicyInputBuilder::default()
    }
}

/// A builder for [`UpdateSchedulingPolicyInput`](crate::operation::update_scheduling_policy::UpdateSchedulingPolicyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateSchedulingPolicyInputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) fairshare_policy: ::std::option::Option<crate::types::FairsharePolicy>,
}
impl UpdateSchedulingPolicyInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy to update.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy to update.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The fair share policy.</p>
    pub fn fairshare_policy(mut self, input: crate::types::FairsharePolicy) -> Self {
        self.fairshare_policy = ::std::option::Option::Some(input);
        self
    }
    /// <p>The fair share policy.</p>
    pub fn set_fairshare_policy(
        mut self,
        input: ::std::option::Option<crate::types::FairsharePolicy>,
    ) -> Self {
        self.fairshare_policy = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateSchedulingPolicyInput`](crate::operation::update_scheduling_policy::UpdateSchedulingPolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_scheduling_policy::UpdateSchedulingPolicyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_scheduling_policy::UpdateSchedulingPolicyInput {
                arn: self.arn,
                fairshare_policy: self.fairshare_policy,
            },
        )
    }
}
