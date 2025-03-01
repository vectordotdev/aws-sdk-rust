// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_coverage_statistics::_get_coverage_statistics_output::GetCoverageStatisticsOutputBuilder;

pub use crate::operation::get_coverage_statistics::_get_coverage_statistics_input::GetCoverageStatisticsInputBuilder;

/// Fluent builder constructing a request to `GetCoverageStatistics`.
///
/// <p>Retrieves aggregated statistics for your account. If you are a GuardDuty administrator, you can retrieve the statistics for all the resources associated with the active member accounts in your organization who have enabled EKS Runtime Monitoring and have the GuardDuty agent running on their EKS nodes.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetCoverageStatisticsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_coverage_statistics::builders::GetCoverageStatisticsInputBuilder,
}
impl GetCoverageStatisticsFluentBuilder {
    /// Creates a new `GetCoverageStatistics`.
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
            crate::operation::get_coverage_statistics::GetCoverageStatistics,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_coverage_statistics::GetCoverageStatisticsError,
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
        crate::operation::get_coverage_statistics::GetCoverageStatisticsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_coverage_statistics::GetCoverageStatisticsError,
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
        crate::operation::get_coverage_statistics::GetCoverageStatisticsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_coverage_statistics::GetCoverageStatisticsError,
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
            crate::operation::get_coverage_statistics::GetCoverageStatistics,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_coverage_statistics::GetCoverageStatisticsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique ID of the GuardDuty detector associated to the coverage statistics.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The unique ID of the GuardDuty detector associated to the coverage statistics.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// <p>Represents the criteria used to filter the coverage statistics</p>
    pub fn filter_criteria(mut self, input: crate::types::CoverageFilterCriteria) -> Self {
        self.inner = self.inner.filter_criteria(input);
        self
    }
    /// <p>Represents the criteria used to filter the coverage statistics</p>
    pub fn set_filter_criteria(
        mut self,
        input: ::std::option::Option<crate::types::CoverageFilterCriteria>,
    ) -> Self {
        self.inner = self.inner.set_filter_criteria(input);
        self
    }
    /// Appends an item to `StatisticsType`.
    ///
    /// To override the contents of this collection use [`set_statistics_type`](Self::set_statistics_type).
    ///
    /// <p>Represents the statistics type used to aggregate the coverage details.</p>
    pub fn statistics_type(mut self, input: crate::types::CoverageStatisticsType) -> Self {
        self.inner = self.inner.statistics_type(input);
        self
    }
    /// <p>Represents the statistics type used to aggregate the coverage details.</p>
    pub fn set_statistics_type(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CoverageStatisticsType>>,
    ) -> Self {
        self.inner = self.inner.set_statistics_type(input);
        self
    }
}
