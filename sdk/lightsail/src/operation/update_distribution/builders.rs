// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_distribution::_update_distribution_output::UpdateDistributionOutputBuilder;

pub use crate::operation::update_distribution::_update_distribution_input::UpdateDistributionInputBuilder;

/// Fluent builder constructing a request to `UpdateDistribution`.
///
/// <p>Updates an existing Amazon Lightsail content delivery network (CDN) distribution.</p>
/// <p>Use this action to update the configuration of your existing distribution.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDistributionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_distribution::builders::UpdateDistributionInputBuilder,
}
impl UpdateDistributionFluentBuilder {
    /// Creates a new `UpdateDistribution`.
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
            crate::operation::update_distribution::UpdateDistribution,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_distribution::UpdateDistributionError,
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
        crate::operation::update_distribution::UpdateDistributionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_distribution::UpdateDistributionError,
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
        crate::operation::update_distribution::UpdateDistributionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_distribution::UpdateDistributionError,
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
            crate::operation::update_distribution::UpdateDistribution,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_distribution::UpdateDistributionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the distribution to update.</p>
    /// <p>Use the <code>GetDistributions</code> action to get a list of distribution names that you can specify.</p>
    pub fn distribution_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.distribution_name(input.into());
        self
    }
    /// <p>The name of the distribution to update.</p>
    /// <p>Use the <code>GetDistributions</code> action to get a list of distribution names that you can specify.</p>
    pub fn set_distribution_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_distribution_name(input);
        self
    }
    /// <p>An object that describes the origin resource for the distribution, such as a Lightsail instance, bucket, or load balancer.</p>
    /// <p>The distribution pulls, caches, and serves content from the origin.</p>
    pub fn origin(mut self, input: crate::types::InputOrigin) -> Self {
        self.inner = self.inner.origin(input);
        self
    }
    /// <p>An object that describes the origin resource for the distribution, such as a Lightsail instance, bucket, or load balancer.</p>
    /// <p>The distribution pulls, caches, and serves content from the origin.</p>
    pub fn set_origin(mut self, input: ::std::option::Option<crate::types::InputOrigin>) -> Self {
        self.inner = self.inner.set_origin(input);
        self
    }
    /// <p>An object that describes the default cache behavior for the distribution.</p>
    pub fn default_cache_behavior(mut self, input: crate::types::CacheBehavior) -> Self {
        self.inner = self.inner.default_cache_behavior(input);
        self
    }
    /// <p>An object that describes the default cache behavior for the distribution.</p>
    pub fn set_default_cache_behavior(
        mut self,
        input: ::std::option::Option<crate::types::CacheBehavior>,
    ) -> Self {
        self.inner = self.inner.set_default_cache_behavior(input);
        self
    }
    /// <p>An object that describes the cache behavior settings for the distribution.</p> <note>
    /// <p>The <code>cacheBehaviorSettings</code> specified in your <code>UpdateDistributionRequest</code> will replace your distribution's existing settings.</p>
    /// </note>
    pub fn cache_behavior_settings(mut self, input: crate::types::CacheSettings) -> Self {
        self.inner = self.inner.cache_behavior_settings(input);
        self
    }
    /// <p>An object that describes the cache behavior settings for the distribution.</p> <note>
    /// <p>The <code>cacheBehaviorSettings</code> specified in your <code>UpdateDistributionRequest</code> will replace your distribution's existing settings.</p>
    /// </note>
    pub fn set_cache_behavior_settings(
        mut self,
        input: ::std::option::Option<crate::types::CacheSettings>,
    ) -> Self {
        self.inner = self.inner.set_cache_behavior_settings(input);
        self
    }
    /// Appends an item to `cacheBehaviors`.
    ///
    /// To override the contents of this collection use [`set_cache_behaviors`](Self::set_cache_behaviors).
    ///
    /// <p>An array of objects that describe the per-path cache behavior for the distribution.</p>
    pub fn cache_behaviors(mut self, input: crate::types::CacheBehaviorPerPath) -> Self {
        self.inner = self.inner.cache_behaviors(input);
        self
    }
    /// <p>An array of objects that describe the per-path cache behavior for the distribution.</p>
    pub fn set_cache_behaviors(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CacheBehaviorPerPath>>,
    ) -> Self {
        self.inner = self.inner.set_cache_behaviors(input);
        self
    }
    /// <p>Indicates whether to enable the distribution.</p>
    pub fn is_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.is_enabled(input);
        self
    }
    /// <p>Indicates whether to enable the distribution.</p>
    pub fn set_is_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_is_enabled(input);
        self
    }
}
