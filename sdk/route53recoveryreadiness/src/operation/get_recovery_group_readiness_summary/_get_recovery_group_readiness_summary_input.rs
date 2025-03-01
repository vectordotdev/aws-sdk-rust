// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetRecoveryGroupReadinessSummaryInput {
    /// <p>The number of objects that you want to return with this call.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token that identifies which batch of results you want to see.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The name of a recovery group.</p>
    #[doc(hidden)]
    pub recovery_group_name: ::std::option::Option<::std::string::String>,
}
impl GetRecoveryGroupReadinessSummaryInput {
    /// <p>The number of objects that you want to return with this call.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token that identifies which batch of results you want to see.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The name of a recovery group.</p>
    pub fn recovery_group_name(&self) -> ::std::option::Option<&str> {
        self.recovery_group_name.as_deref()
    }
}
impl GetRecoveryGroupReadinessSummaryInput {
    /// Creates a new builder-style object to manufacture [`GetRecoveryGroupReadinessSummaryInput`](crate::operation::get_recovery_group_readiness_summary::GetRecoveryGroupReadinessSummaryInput).
    pub fn builder() -> crate::operation::get_recovery_group_readiness_summary::builders::GetRecoveryGroupReadinessSummaryInputBuilder{
        crate::operation::get_recovery_group_readiness_summary::builders::GetRecoveryGroupReadinessSummaryInputBuilder::default()
    }
}

/// A builder for [`GetRecoveryGroupReadinessSummaryInput`](crate::operation::get_recovery_group_readiness_summary::GetRecoveryGroupReadinessSummaryInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetRecoveryGroupReadinessSummaryInputBuilder {
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) recovery_group_name: ::std::option::Option<::std::string::String>,
}
impl GetRecoveryGroupReadinessSummaryInputBuilder {
    /// <p>The number of objects that you want to return with this call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of objects that you want to return with this call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token that identifies which batch of results you want to see.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token that identifies which batch of results you want to see.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The name of a recovery group.</p>
    pub fn recovery_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.recovery_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of a recovery group.</p>
    pub fn set_recovery_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.recovery_group_name = input;
        self
    }
    /// Consumes the builder and constructs a [`GetRecoveryGroupReadinessSummaryInput`](crate::operation::get_recovery_group_readiness_summary::GetRecoveryGroupReadinessSummaryInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_recovery_group_readiness_summary::GetRecoveryGroupReadinessSummaryInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::get_recovery_group_readiness_summary::GetRecoveryGroupReadinessSummaryInput {
                max_results: self.max_results
                ,
                next_token: self.next_token
                ,
                recovery_group_name: self.recovery_group_name
                ,
            }
        )
    }
}
