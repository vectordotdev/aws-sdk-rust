// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_aws_network_performance_metric_subscription::_enable_aws_network_performance_metric_subscription_output::EnableAwsNetworkPerformanceMetricSubscriptionOutputBuilder;

pub use crate::operation::enable_aws_network_performance_metric_subscription::_enable_aws_network_performance_metric_subscription_input::EnableAwsNetworkPerformanceMetricSubscriptionInputBuilder;

/// Fluent builder constructing a request to `EnableAwsNetworkPerformanceMetricSubscription`.
///
/// <p>Enables Infrastructure Performance subscriptions.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EnableAwsNetworkPerformanceMetricSubscriptionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::enable_aws_network_performance_metric_subscription::builders::EnableAwsNetworkPerformanceMetricSubscriptionInputBuilder,
}
impl EnableAwsNetworkPerformanceMetricSubscriptionFluentBuilder {
    /// Creates a new `EnableAwsNetworkPerformanceMetricSubscription`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscription, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionOutput, ::aws_smithy_http::result::SdkError<crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionOutput, ::aws_smithy_http::result::SdkError<crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscription, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionError>
    >{
        self.customize_middleware().await
    }
    /// <p>The source Region or Availability Zone that the metric subscription is enabled for. For example, <code>us-east-1</code>.</p>
    pub fn source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source(input.into());
        self
    }
    /// <p>The source Region or Availability Zone that the metric subscription is enabled for. For example, <code>us-east-1</code>.</p>
    pub fn set_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source(input);
        self
    }
    /// <p>The target Region or Availability Zone that the metric subscription is enabled for. For example, <code>eu-west-1</code>.</p>
    pub fn destination(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.destination(input.into());
        self
    }
    /// <p>The target Region or Availability Zone that the metric subscription is enabled for. For example, <code>eu-west-1</code>.</p>
    pub fn set_destination(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_destination(input);
        self
    }
    /// <p>The metric used for the enabled subscription.</p>
    pub fn metric(mut self, input: crate::types::MetricType) -> Self {
        self.inner = self.inner.metric(input);
        self
    }
    /// <p>The metric used for the enabled subscription.</p>
    pub fn set_metric(mut self, input: ::std::option::Option<crate::types::MetricType>) -> Self {
        self.inner = self.inner.set_metric(input);
        self
    }
    /// <p>The statistic used for the enabled subscription.</p>
    pub fn statistic(mut self, input: crate::types::StatisticType) -> Self {
        self.inner = self.inner.statistic(input);
        self
    }
    /// <p>The statistic used for the enabled subscription.</p>
    pub fn set_statistic(
        mut self,
        input: ::std::option::Option<crate::types::StatisticType>,
    ) -> Self {
        self.inner = self.inner.set_statistic(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
}
