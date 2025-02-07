// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::meter_usage::_meter_usage_output::MeterUsageOutputBuilder;

pub use crate::operation::meter_usage::_meter_usage_input::MeterUsageInputBuilder;

/// Fluent builder constructing a request to `MeterUsage`.
///
/// <p>API to emit metering records. For identical requests, the API is idempotent. It simply returns the metering record ID.</p>
/// <p> <code>MeterUsage</code> is authenticated on the buyer's AWS account using credentials from the EC2 instance, ECS task, or EKS pod.</p>
/// <p> <code>MeterUsage</code> can optionally include multiple usage allocations, to provide customers with usage data split into buckets by tags that you define (or allow the customer to define).</p>
/// <p>Usage records are expected to be submitted as quickly as possible after the event that is being recorded, and are not accepted more than 6 hours after the event.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct MeterUsageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::meter_usage::builders::MeterUsageInputBuilder,
}
impl MeterUsageFluentBuilder {
    /// Creates a new `MeterUsage`.
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
            crate::operation::meter_usage::MeterUsage,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::meter_usage::MeterUsageError>,
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
        crate::operation::meter_usage::MeterUsageOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::meter_usage::MeterUsageError>,
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
        crate::operation::meter_usage::MeterUsageOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::meter_usage::MeterUsageError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::meter_usage::MeterUsage,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::meter_usage::MeterUsageError>,
    > {
        self.customize_middleware().await
    }
    /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
    pub fn product_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.product_code(input.into());
        self
    }
    /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code should be the same as the one used during the publishing of a new product.</p>
    pub fn set_product_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_product_code(input);
        self
    }
    /// <p>Timestamp, in UTC, for which the usage is being reported. Your application can meter usage for up to one hour in the past. Make sure the <code>timestamp</code> value is not before the start of the software usage.</p>
    pub fn timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.timestamp(input);
        self
    }
    /// <p>Timestamp, in UTC, for which the usage is being reported. Your application can meter usage for up to one hour in the past. Make sure the <code>timestamp</code> value is not before the start of the software usage.</p>
    pub fn set_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_timestamp(input);
        self
    }
    /// <p>It will be one of the fcp dimension name provided during the publishing of the product.</p>
    pub fn usage_dimension(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.usage_dimension(input.into());
        self
    }
    /// <p>It will be one of the fcp dimension name provided during the publishing of the product.</p>
    pub fn set_usage_dimension(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_usage_dimension(input);
        self
    }
    /// <p>Consumption value for the hour. Defaults to <code>0</code> if not specified.</p>
    pub fn usage_quantity(mut self, input: i32) -> Self {
        self.inner = self.inner.usage_quantity(input);
        self
    }
    /// <p>Consumption value for the hour. Defaults to <code>0</code> if not specified.</p>
    pub fn set_usage_quantity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_usage_quantity(input);
        self
    }
    /// <p>Checks whether you have the permissions required for the action, but does not make the request. If you have the permissions, the request returns <code>DryRunOperation</code>; otherwise, it returns <code>UnauthorizedException</code>. Defaults to <code>false</code> if not specified.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the permissions required for the action, but does not make the request. If you have the permissions, the request returns <code>DryRunOperation</code>; otherwise, it returns <code>UnauthorizedException</code>. Defaults to <code>false</code> if not specified.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// Appends an item to `UsageAllocations`.
    ///
    /// To override the contents of this collection use [`set_usage_allocations`](Self::set_usage_allocations).
    ///
    /// <p>The set of <code>UsageAllocations</code> to submit.</p>
    /// <p>The sum of all <code>UsageAllocation</code> quantities must equal the <code>UsageQuantity</code> of the <code>MeterUsage</code> request, and each <code>UsageAllocation</code> must have a unique set of tags (include no tags).</p>
    pub fn usage_allocations(mut self, input: crate::types::UsageAllocation) -> Self {
        self.inner = self.inner.usage_allocations(input);
        self
    }
    /// <p>The set of <code>UsageAllocations</code> to submit.</p>
    /// <p>The sum of all <code>UsageAllocation</code> quantities must equal the <code>UsageQuantity</code> of the <code>MeterUsage</code> request, and each <code>UsageAllocation</code> must have a unique set of tags (include no tags).</p>
    pub fn set_usage_allocations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::UsageAllocation>>,
    ) -> Self {
        self.inner = self.inner.set_usage_allocations(input);
        self
    }
}
