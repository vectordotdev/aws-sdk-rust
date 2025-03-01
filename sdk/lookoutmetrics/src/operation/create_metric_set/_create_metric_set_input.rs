// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateMetricSetInput {
    /// <p>The ARN of the anomaly detector that will use the dataset.</p>
    #[doc(hidden)]
    pub anomaly_detector_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the dataset.</p>
    #[doc(hidden)]
    pub metric_set_name: ::std::option::Option<::std::string::String>,
    /// <p>A description of the dataset you are creating.</p>
    #[doc(hidden)]
    pub metric_set_description: ::std::option::Option<::std::string::String>,
    /// <p>A list of metrics that the dataset will contain.</p>
    #[doc(hidden)]
    pub metric_list: ::std::option::Option<::std::vec::Vec<crate::types::Metric>>,
    /// <p>After an interval ends, the amount of seconds that the detector waits before importing data. Offset is only supported for S3, Redshift, Athena and datasources.</p>
    #[doc(hidden)]
    pub offset: ::std::option::Option<i32>,
    /// <p>Contains information about the column used for tracking time in your source data.</p>
    #[doc(hidden)]
    pub timestamp_column: ::std::option::Option<crate::types::TimestampColumn>,
    /// <p>A list of the fields you want to treat as dimensions.</p>
    #[doc(hidden)]
    pub dimension_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The frequency with which the source data will be analyzed for anomalies.</p>
    #[doc(hidden)]
    pub metric_set_frequency: ::std::option::Option<crate::types::Frequency>,
    /// <p>Contains information about how the source data should be interpreted.</p>
    #[doc(hidden)]
    pub metric_source: ::std::option::Option<crate::types::MetricSource>,
    /// <p>The time zone in which your source data was recorded.</p>
    #[doc(hidden)]
    pub timezone: ::std::option::Option<::std::string::String>,
    /// <p>A list of <a href="https://docs.aws.amazon.com/lookoutmetrics/latest/dev/detectors-tags.html">tags</a> to apply to the dataset.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>A list of filters that specify which data is kept for anomaly detection.</p>
    #[doc(hidden)]
    pub dimension_filter_list:
        ::std::option::Option<::std::vec::Vec<crate::types::MetricSetDimensionFilter>>,
}
impl CreateMetricSetInput {
    /// <p>The ARN of the anomaly detector that will use the dataset.</p>
    pub fn anomaly_detector_arn(&self) -> ::std::option::Option<&str> {
        self.anomaly_detector_arn.as_deref()
    }
    /// <p>The name of the dataset.</p>
    pub fn metric_set_name(&self) -> ::std::option::Option<&str> {
        self.metric_set_name.as_deref()
    }
    /// <p>A description of the dataset you are creating.</p>
    pub fn metric_set_description(&self) -> ::std::option::Option<&str> {
        self.metric_set_description.as_deref()
    }
    /// <p>A list of metrics that the dataset will contain.</p>
    pub fn metric_list(&self) -> ::std::option::Option<&[crate::types::Metric]> {
        self.metric_list.as_deref()
    }
    /// <p>After an interval ends, the amount of seconds that the detector waits before importing data. Offset is only supported for S3, Redshift, Athena and datasources.</p>
    pub fn offset(&self) -> ::std::option::Option<i32> {
        self.offset
    }
    /// <p>Contains information about the column used for tracking time in your source data.</p>
    pub fn timestamp_column(&self) -> ::std::option::Option<&crate::types::TimestampColumn> {
        self.timestamp_column.as_ref()
    }
    /// <p>A list of the fields you want to treat as dimensions.</p>
    pub fn dimension_list(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.dimension_list.as_deref()
    }
    /// <p>The frequency with which the source data will be analyzed for anomalies.</p>
    pub fn metric_set_frequency(&self) -> ::std::option::Option<&crate::types::Frequency> {
        self.metric_set_frequency.as_ref()
    }
    /// <p>Contains information about how the source data should be interpreted.</p>
    pub fn metric_source(&self) -> ::std::option::Option<&crate::types::MetricSource> {
        self.metric_source.as_ref()
    }
    /// <p>The time zone in which your source data was recorded.</p>
    pub fn timezone(&self) -> ::std::option::Option<&str> {
        self.timezone.as_deref()
    }
    /// <p>A list of <a href="https://docs.aws.amazon.com/lookoutmetrics/latest/dev/detectors-tags.html">tags</a> to apply to the dataset.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
    /// <p>A list of filters that specify which data is kept for anomaly detection.</p>
    pub fn dimension_filter_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::MetricSetDimensionFilter]> {
        self.dimension_filter_list.as_deref()
    }
}
impl CreateMetricSetInput {
    /// Creates a new builder-style object to manufacture [`CreateMetricSetInput`](crate::operation::create_metric_set::CreateMetricSetInput).
    pub fn builder() -> crate::operation::create_metric_set::builders::CreateMetricSetInputBuilder {
        crate::operation::create_metric_set::builders::CreateMetricSetInputBuilder::default()
    }
}

/// A builder for [`CreateMetricSetInput`](crate::operation::create_metric_set::CreateMetricSetInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateMetricSetInputBuilder {
    pub(crate) anomaly_detector_arn: ::std::option::Option<::std::string::String>,
    pub(crate) metric_set_name: ::std::option::Option<::std::string::String>,
    pub(crate) metric_set_description: ::std::option::Option<::std::string::String>,
    pub(crate) metric_list: ::std::option::Option<::std::vec::Vec<crate::types::Metric>>,
    pub(crate) offset: ::std::option::Option<i32>,
    pub(crate) timestamp_column: ::std::option::Option<crate::types::TimestampColumn>,
    pub(crate) dimension_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) metric_set_frequency: ::std::option::Option<crate::types::Frequency>,
    pub(crate) metric_source: ::std::option::Option<crate::types::MetricSource>,
    pub(crate) timezone: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) dimension_filter_list:
        ::std::option::Option<::std::vec::Vec<crate::types::MetricSetDimensionFilter>>,
}
impl CreateMetricSetInputBuilder {
    /// <p>The ARN of the anomaly detector that will use the dataset.</p>
    pub fn anomaly_detector_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.anomaly_detector_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the anomaly detector that will use the dataset.</p>
    pub fn set_anomaly_detector_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.anomaly_detector_arn = input;
        self
    }
    /// <p>The name of the dataset.</p>
    pub fn metric_set_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.metric_set_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the dataset.</p>
    pub fn set_metric_set_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.metric_set_name = input;
        self
    }
    /// <p>A description of the dataset you are creating.</p>
    pub fn metric_set_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.metric_set_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the dataset you are creating.</p>
    pub fn set_metric_set_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.metric_set_description = input;
        self
    }
    /// Appends an item to `metric_list`.
    ///
    /// To override the contents of this collection use [`set_metric_list`](Self::set_metric_list).
    ///
    /// <p>A list of metrics that the dataset will contain.</p>
    pub fn metric_list(mut self, input: crate::types::Metric) -> Self {
        let mut v = self.metric_list.unwrap_or_default();
        v.push(input);
        self.metric_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of metrics that the dataset will contain.</p>
    pub fn set_metric_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Metric>>,
    ) -> Self {
        self.metric_list = input;
        self
    }
    /// <p>After an interval ends, the amount of seconds that the detector waits before importing data. Offset is only supported for S3, Redshift, Athena and datasources.</p>
    pub fn offset(mut self, input: i32) -> Self {
        self.offset = ::std::option::Option::Some(input);
        self
    }
    /// <p>After an interval ends, the amount of seconds that the detector waits before importing data. Offset is only supported for S3, Redshift, Athena and datasources.</p>
    pub fn set_offset(mut self, input: ::std::option::Option<i32>) -> Self {
        self.offset = input;
        self
    }
    /// <p>Contains information about the column used for tracking time in your source data.</p>
    pub fn timestamp_column(mut self, input: crate::types::TimestampColumn) -> Self {
        self.timestamp_column = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains information about the column used for tracking time in your source data.</p>
    pub fn set_timestamp_column(
        mut self,
        input: ::std::option::Option<crate::types::TimestampColumn>,
    ) -> Self {
        self.timestamp_column = input;
        self
    }
    /// Appends an item to `dimension_list`.
    ///
    /// To override the contents of this collection use [`set_dimension_list`](Self::set_dimension_list).
    ///
    /// <p>A list of the fields you want to treat as dimensions.</p>
    pub fn dimension_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.dimension_list.unwrap_or_default();
        v.push(input.into());
        self.dimension_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the fields you want to treat as dimensions.</p>
    pub fn set_dimension_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.dimension_list = input;
        self
    }
    /// <p>The frequency with which the source data will be analyzed for anomalies.</p>
    pub fn metric_set_frequency(mut self, input: crate::types::Frequency) -> Self {
        self.metric_set_frequency = ::std::option::Option::Some(input);
        self
    }
    /// <p>The frequency with which the source data will be analyzed for anomalies.</p>
    pub fn set_metric_set_frequency(
        mut self,
        input: ::std::option::Option<crate::types::Frequency>,
    ) -> Self {
        self.metric_set_frequency = input;
        self
    }
    /// <p>Contains information about how the source data should be interpreted.</p>
    pub fn metric_source(mut self, input: crate::types::MetricSource) -> Self {
        self.metric_source = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains information about how the source data should be interpreted.</p>
    pub fn set_metric_source(
        mut self,
        input: ::std::option::Option<crate::types::MetricSource>,
    ) -> Self {
        self.metric_source = input;
        self
    }
    /// <p>The time zone in which your source data was recorded.</p>
    pub fn timezone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.timezone = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The time zone in which your source data was recorded.</p>
    pub fn set_timezone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.timezone = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of <a href="https://docs.aws.amazon.com/lookoutmetrics/latest/dev/detectors-tags.html">tags</a> to apply to the dataset.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A list of <a href="https://docs.aws.amazon.com/lookoutmetrics/latest/dev/detectors-tags.html">tags</a> to apply to the dataset.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Appends an item to `dimension_filter_list`.
    ///
    /// To override the contents of this collection use [`set_dimension_filter_list`](Self::set_dimension_filter_list).
    ///
    /// <p>A list of filters that specify which data is kept for anomaly detection.</p>
    pub fn dimension_filter_list(mut self, input: crate::types::MetricSetDimensionFilter) -> Self {
        let mut v = self.dimension_filter_list.unwrap_or_default();
        v.push(input);
        self.dimension_filter_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of filters that specify which data is kept for anomaly detection.</p>
    pub fn set_dimension_filter_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MetricSetDimensionFilter>>,
    ) -> Self {
        self.dimension_filter_list = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateMetricSetInput`](crate::operation::create_metric_set::CreateMetricSetInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_metric_set::CreateMetricSetInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_metric_set::CreateMetricSetInput {
            anomaly_detector_arn: self.anomaly_detector_arn,
            metric_set_name: self.metric_set_name,
            metric_set_description: self.metric_set_description,
            metric_list: self.metric_list,
            offset: self.offset,
            timestamp_column: self.timestamp_column,
            dimension_list: self.dimension_list,
            metric_set_frequency: self.metric_set_frequency,
            metric_source: self.metric_source,
            timezone: self.timezone,
            tags: self.tags,
            dimension_filter_list: self.dimension_filter_list,
        })
    }
}
