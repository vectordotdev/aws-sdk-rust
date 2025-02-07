// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_pull_through_cache_rule::_delete_pull_through_cache_rule_output::DeletePullThroughCacheRuleOutputBuilder;

pub use crate::operation::delete_pull_through_cache_rule::_delete_pull_through_cache_rule_input::DeletePullThroughCacheRuleInputBuilder;

/// Fluent builder constructing a request to `DeletePullThroughCacheRule`.
///
/// <p>Deletes a pull through cache rule.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeletePullThroughCacheRuleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::delete_pull_through_cache_rule::builders::DeletePullThroughCacheRuleInputBuilder,
}
impl DeletePullThroughCacheRuleFluentBuilder {
    /// Creates a new `DeletePullThroughCacheRule`.
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
            crate::operation::delete_pull_through_cache_rule::DeletePullThroughCacheRule,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_pull_through_cache_rule::DeletePullThroughCacheRuleError,
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
        crate::operation::delete_pull_through_cache_rule::DeletePullThroughCacheRuleOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_pull_through_cache_rule::DeletePullThroughCacheRuleError,
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
        crate::operation::delete_pull_through_cache_rule::DeletePullThroughCacheRuleOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_pull_through_cache_rule::DeletePullThroughCacheRuleError,
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
            crate::operation::delete_pull_through_cache_rule::DeletePullThroughCacheRule,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_pull_through_cache_rule::DeletePullThroughCacheRuleError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon ECR repository prefix associated with the pull through cache rule to delete.</p>
    pub fn ecr_repository_prefix(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.ecr_repository_prefix(input.into());
        self
    }
    /// <p>The Amazon ECR repository prefix associated with the pull through cache rule to delete.</p>
    pub fn set_ecr_repository_prefix(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_ecr_repository_prefix(input);
        self
    }
    /// <p>The Amazon Web Services account ID associated with the registry that contains the pull through cache rule. If you do not specify a registry, the default registry is assumed.</p>
    pub fn registry_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.registry_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID associated with the registry that contains the pull through cache rule. If you do not specify a registry, the default registry is assumed.</p>
    pub fn set_registry_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_registry_id(input);
        self
    }
}
