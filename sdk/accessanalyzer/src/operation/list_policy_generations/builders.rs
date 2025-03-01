// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_policy_generations::_list_policy_generations_output::ListPolicyGenerationsOutputBuilder;

pub use crate::operation::list_policy_generations::_list_policy_generations_input::ListPolicyGenerationsInputBuilder;

/// Fluent builder constructing a request to `ListPolicyGenerations`.
///
/// <p>Lists all of the policy generations requested in the last seven days.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListPolicyGenerationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_policy_generations::builders::ListPolicyGenerationsInputBuilder,
}
impl ListPolicyGenerationsFluentBuilder {
    /// Creates a new `ListPolicyGenerations`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_policy_generations::ListPolicyGenerations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_policy_generations::ListPolicyGenerationsError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_policy_generations::ListPolicyGenerationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_policy_generations::ListPolicyGenerationsError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_policy_generations::ListPolicyGenerationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_policy_generations::ListPolicyGenerationsError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_policy_generations::ListPolicyGenerations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_policy_generations::ListPolicyGenerationsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_policy_generations::paginator::ListPolicyGenerationsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_policy_generations::paginator::ListPolicyGenerationsPaginator {
        crate::operation::list_policy_generations::paginator::ListPolicyGenerationsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The ARN of the IAM entity (user or role) for which you are generating a policy. Use this with <code>ListGeneratedPolicies</code> to filter the results to only include results for a specific principal.</p>
    pub fn principal_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.principal_arn(input.into());
        self
    }
    /// <p>The ARN of the IAM entity (user or role) for which you are generating a policy. Use this with <code>ListGeneratedPolicies</code> to filter the results to only include results for a specific principal.</p>
    pub fn set_principal_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_principal_arn(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>A token used for pagination of results returned.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token used for pagination of results returned.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
