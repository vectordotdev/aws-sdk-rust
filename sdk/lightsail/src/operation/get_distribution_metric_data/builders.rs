// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_distribution_metric_data::_get_distribution_metric_data_output::GetDistributionMetricDataOutputBuilder;

pub use crate::operation::get_distribution_metric_data::_get_distribution_metric_data_input::GetDistributionMetricDataInputBuilder;

/// Fluent builder constructing a request to `GetDistributionMetricData`.
///
/// <p>Returns the data points of a specific metric for an Amazon Lightsail content delivery network (CDN) distribution.</p>
/// <p>Metrics report the utilization of your resources, and the error counts generated by them. Monitor and collect metric data regularly to maintain the reliability, availability, and performance of your resources.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetDistributionMetricDataFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::get_distribution_metric_data::builders::GetDistributionMetricDataInputBuilder,
}
impl GetDistributionMetricDataFluentBuilder {
    /// Creates a new `GetDistributionMetricData`.
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
            crate::operation::get_distribution_metric_data::GetDistributionMetricData,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_distribution_metric_data::GetDistributionMetricDataError,
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
        crate::operation::get_distribution_metric_data::GetDistributionMetricDataOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_distribution_metric_data::GetDistributionMetricDataError,
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
        crate::operation::get_distribution_metric_data::GetDistributionMetricDataOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_distribution_metric_data::GetDistributionMetricDataError,
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
            crate::operation::get_distribution_metric_data::GetDistributionMetricData,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_distribution_metric_data::GetDistributionMetricDataError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the distribution for which to get metric data.</p>
    /// <p>Use the <code>GetDistributions</code> action to get a list of distribution names that you can specify.</p>
    pub fn distribution_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.distribution_name(input.into());
        self
    }
    /// <p>The name of the distribution for which to get metric data.</p>
    /// <p>Use the <code>GetDistributions</code> action to get a list of distribution names that you can specify.</p>
    pub fn set_distribution_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_distribution_name(input);
        self
    }
    /// <p>The metric for which you want to return information.</p>
    /// <p>Valid distribution metric names are listed below, along with the most useful <code>statistics</code> to include in your request, and the published <code>unit</code> value.</p>
    /// <ul>
    /// <li> <p> <b> <code>Requests</code> </b> - The total number of viewer requests received by your Lightsail distribution, for all HTTP methods, and for both HTTP and HTTPS requests.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> <p> <code>Unit</code>: The published unit is <code>None</code>.</p> </li>
    /// <li> <p> <b> <code>BytesDownloaded</code> </b> - The number of bytes downloaded by viewers for GET, HEAD, and OPTIONS requests.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> <p> <code>Unit</code>: The published unit is <code>None</code>.</p> </li>
    /// <li> <p> <b> <code>BytesUploaded </code> </b> - The number of bytes uploaded to your origin by your Lightsail distribution, using POST and PUT requests.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> <p> <code>Unit</code>: The published unit is <code>None</code>.</p> </li>
    /// <li> <p> <b> <code>TotalErrorRate</code> </b> - The percentage of all viewer requests for which the response's HTTP status code was 4xx or 5xx.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Average</code>.</p> <p> <code>Unit</code>: The published unit is <code>Percent</code>.</p> </li>
    /// <li> <p> <b> <code>4xxErrorRate</code> </b> - The percentage of all viewer requests for which the response's HTTP status cod was 4xx. In these cases, the client or client viewer may have made an error. For example, a status code of 404 (Not Found) means that the client requested an object that could not be found.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Average</code>.</p> <p> <code>Unit</code>: The published unit is <code>Percent</code>.</p> </li>
    /// <li> <p> <b> <code>5xxErrorRate</code> </b> - The percentage of all viewer requests for which the response's HTTP status code was 5xx. In these cases, the origin server did not satisfy the requests. For example, a status code of 503 (Service Unavailable) means that the origin server is currently unavailable.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Average</code>.</p> <p> <code>Unit</code>: The published unit is <code>Percent</code>.</p> </li>
    /// </ul>
    pub fn metric_name(mut self, input: crate::types::DistributionMetricName) -> Self {
        self.inner = self.inner.metric_name(input);
        self
    }
    /// <p>The metric for which you want to return information.</p>
    /// <p>Valid distribution metric names are listed below, along with the most useful <code>statistics</code> to include in your request, and the published <code>unit</code> value.</p>
    /// <ul>
    /// <li> <p> <b> <code>Requests</code> </b> - The total number of viewer requests received by your Lightsail distribution, for all HTTP methods, and for both HTTP and HTTPS requests.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> <p> <code>Unit</code>: The published unit is <code>None</code>.</p> </li>
    /// <li> <p> <b> <code>BytesDownloaded</code> </b> - The number of bytes downloaded by viewers for GET, HEAD, and OPTIONS requests.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> <p> <code>Unit</code>: The published unit is <code>None</code>.</p> </li>
    /// <li> <p> <b> <code>BytesUploaded </code> </b> - The number of bytes uploaded to your origin by your Lightsail distribution, using POST and PUT requests.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> <p> <code>Unit</code>: The published unit is <code>None</code>.</p> </li>
    /// <li> <p> <b> <code>TotalErrorRate</code> </b> - The percentage of all viewer requests for which the response's HTTP status code was 4xx or 5xx.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Average</code>.</p> <p> <code>Unit</code>: The published unit is <code>Percent</code>.</p> </li>
    /// <li> <p> <b> <code>4xxErrorRate</code> </b> - The percentage of all viewer requests for which the response's HTTP status cod was 4xx. In these cases, the client or client viewer may have made an error. For example, a status code of 404 (Not Found) means that the client requested an object that could not be found.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Average</code>.</p> <p> <code>Unit</code>: The published unit is <code>Percent</code>.</p> </li>
    /// <li> <p> <b> <code>5xxErrorRate</code> </b> - The percentage of all viewer requests for which the response's HTTP status code was 5xx. In these cases, the origin server did not satisfy the requests. For example, a status code of 503 (Service Unavailable) means that the origin server is currently unavailable.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Average</code>.</p> <p> <code>Unit</code>: The published unit is <code>Percent</code>.</p> </li>
    /// </ul>
    pub fn set_metric_name(
        mut self,
        input: ::std::option::Option<crate::types::DistributionMetricName>,
    ) -> Self {
        self.inner = self.inner.set_metric_name(input);
        self
    }
    /// <p>The start of the time interval for which to get metric data.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li>
    /// <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use a start time of October 1, 2018, at 8 PM UTC, specify <code>1538424000</code> as the start time.</p> </li>
    /// </ul>
    /// <p>You can convert a human-friendly time to Unix time format using a converter like <a href="https://www.epochconverter.com/">Epoch converter</a>.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The start of the time interval for which to get metric data.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li>
    /// <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use a start time of October 1, 2018, at 8 PM UTC, specify <code>1538424000</code> as the start time.</p> </li>
    /// </ul>
    /// <p>You can convert a human-friendly time to Unix time format using a converter like <a href="https://www.epochconverter.com/">Epoch converter</a>.</p>
    pub fn set_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The end of the time interval for which to get metric data.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li>
    /// <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use an end time of October 1, 2018, at 9 PM UTC, specify <code>1538427600</code> as the end time.</p> </li>
    /// </ul>
    /// <p>You can convert a human-friendly time to Unix time format using a converter like <a href="https://www.epochconverter.com/">Epoch converter</a>.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The end of the time interval for which to get metric data.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li>
    /// <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use an end time of October 1, 2018, at 9 PM UTC, specify <code>1538427600</code> as the end time.</p> </li>
    /// </ul>
    /// <p>You can convert a human-friendly time to Unix time format using a converter like <a href="https://www.epochconverter.com/">Epoch converter</a>.</p>
    pub fn set_end_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The granularity, in seconds, for the metric data points that will be returned.</p>
    pub fn period(mut self, input: i32) -> Self {
        self.inner = self.inner.period(input);
        self
    }
    /// <p>The granularity, in seconds, for the metric data points that will be returned.</p>
    pub fn set_period(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_period(input);
        self
    }
    /// <p>The unit for the metric data request.</p>
    /// <p>Valid units depend on the metric data being requested. For the valid units with each available metric, see the <code>metricName</code> parameter.</p>
    pub fn unit(mut self, input: crate::types::MetricUnit) -> Self {
        self.inner = self.inner.unit(input);
        self
    }
    /// <p>The unit for the metric data request.</p>
    /// <p>Valid units depend on the metric data being requested. For the valid units with each available metric, see the <code>metricName</code> parameter.</p>
    pub fn set_unit(mut self, input: ::std::option::Option<crate::types::MetricUnit>) -> Self {
        self.inner = self.inner.set_unit(input);
        self
    }
    /// Appends an item to `statistics`.
    ///
    /// To override the contents of this collection use [`set_statistics`](Self::set_statistics).
    ///
    /// <p>The statistic for the metric.</p>
    /// <p>The following statistics are available:</p>
    /// <ul>
    /// <li> <p> <code>Minimum</code> - The lowest value observed during the specified period. Use this value to determine low volumes of activity for your application.</p> </li>
    /// <li> <p> <code>Maximum</code> - The highest value observed during the specified period. Use this value to determine high volumes of activity for your application.</p> </li>
    /// <li> <p> <code>Sum</code> - All values submitted for the matching metric added together. You can use this statistic to determine the total volume of a metric.</p> </li>
    /// <li> <p> <code>Average</code> - The value of Sum / SampleCount during the specified period. By comparing this statistic with the Minimum and Maximum values, you can determine the full scope of a metric and how close the average use is to the Minimum and Maximum values. This comparison helps you to know when to increase or decrease your resources.</p> </li>
    /// <li> <p> <code>SampleCount</code> - The count, or number, of data points used for the statistical calculation.</p> </li>
    /// </ul>
    pub fn statistics(mut self, input: crate::types::MetricStatistic) -> Self {
        self.inner = self.inner.statistics(input);
        self
    }
    /// <p>The statistic for the metric.</p>
    /// <p>The following statistics are available:</p>
    /// <ul>
    /// <li> <p> <code>Minimum</code> - The lowest value observed during the specified period. Use this value to determine low volumes of activity for your application.</p> </li>
    /// <li> <p> <code>Maximum</code> - The highest value observed during the specified period. Use this value to determine high volumes of activity for your application.</p> </li>
    /// <li> <p> <code>Sum</code> - All values submitted for the matching metric added together. You can use this statistic to determine the total volume of a metric.</p> </li>
    /// <li> <p> <code>Average</code> - The value of Sum / SampleCount during the specified period. By comparing this statistic with the Minimum and Maximum values, you can determine the full scope of a metric and how close the average use is to the Minimum and Maximum values. This comparison helps you to know when to increase or decrease your resources.</p> </li>
    /// <li> <p> <code>SampleCount</code> - The count, or number, of data points used for the statistical calculation.</p> </li>
    /// </ul>
    pub fn set_statistics(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MetricStatistic>>,
    ) -> Self {
        self.inner = self.inner.set_statistics(input);
        self
    }
}
