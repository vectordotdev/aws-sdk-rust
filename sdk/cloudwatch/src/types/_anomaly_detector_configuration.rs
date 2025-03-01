// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration specifies details about how the anomaly detection model is to be trained, including time ranges to exclude from use for training the model and the time zone to use for the metric.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AnomalyDetectorConfiguration {
    /// <p>An array of time ranges to exclude from use when the anomaly detection model is trained. Use this to make sure that events that could cause unusual values for the metric, such as deployments, aren't used when CloudWatch creates the model.</p>
    #[doc(hidden)]
    pub excluded_time_ranges: ::std::option::Option<::std::vec::Vec<crate::types::Range>>,
    /// <p>The time zone to use for the metric. This is useful to enable the model to automatically account for daylight savings time changes if the metric is sensitive to such time changes.</p>
    /// <p>To specify a time zone, use the name of the time zone as specified in the standard tz database. For more information, see <a href="https://en.wikipedia.org/wiki/Tz_database">tz database</a>.</p>
    #[doc(hidden)]
    pub metric_timezone: ::std::option::Option<::std::string::String>,
}
impl AnomalyDetectorConfiguration {
    /// <p>An array of time ranges to exclude from use when the anomaly detection model is trained. Use this to make sure that events that could cause unusual values for the metric, such as deployments, aren't used when CloudWatch creates the model.</p>
    pub fn excluded_time_ranges(&self) -> ::std::option::Option<&[crate::types::Range]> {
        self.excluded_time_ranges.as_deref()
    }
    /// <p>The time zone to use for the metric. This is useful to enable the model to automatically account for daylight savings time changes if the metric is sensitive to such time changes.</p>
    /// <p>To specify a time zone, use the name of the time zone as specified in the standard tz database. For more information, see <a href="https://en.wikipedia.org/wiki/Tz_database">tz database</a>.</p>
    pub fn metric_timezone(&self) -> ::std::option::Option<&str> {
        self.metric_timezone.as_deref()
    }
}
impl AnomalyDetectorConfiguration {
    /// Creates a new builder-style object to manufacture [`AnomalyDetectorConfiguration`](crate::types::AnomalyDetectorConfiguration).
    pub fn builder() -> crate::types::builders::AnomalyDetectorConfigurationBuilder {
        crate::types::builders::AnomalyDetectorConfigurationBuilder::default()
    }
}

/// A builder for [`AnomalyDetectorConfiguration`](crate::types::AnomalyDetectorConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AnomalyDetectorConfigurationBuilder {
    pub(crate) excluded_time_ranges: ::std::option::Option<::std::vec::Vec<crate::types::Range>>,
    pub(crate) metric_timezone: ::std::option::Option<::std::string::String>,
}
impl AnomalyDetectorConfigurationBuilder {
    /// Appends an item to `excluded_time_ranges`.
    ///
    /// To override the contents of this collection use [`set_excluded_time_ranges`](Self::set_excluded_time_ranges).
    ///
    /// <p>An array of time ranges to exclude from use when the anomaly detection model is trained. Use this to make sure that events that could cause unusual values for the metric, such as deployments, aren't used when CloudWatch creates the model.</p>
    pub fn excluded_time_ranges(mut self, input: crate::types::Range) -> Self {
        let mut v = self.excluded_time_ranges.unwrap_or_default();
        v.push(input);
        self.excluded_time_ranges = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of time ranges to exclude from use when the anomaly detection model is trained. Use this to make sure that events that could cause unusual values for the metric, such as deployments, aren't used when CloudWatch creates the model.</p>
    pub fn set_excluded_time_ranges(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Range>>,
    ) -> Self {
        self.excluded_time_ranges = input;
        self
    }
    /// <p>The time zone to use for the metric. This is useful to enable the model to automatically account for daylight savings time changes if the metric is sensitive to such time changes.</p>
    /// <p>To specify a time zone, use the name of the time zone as specified in the standard tz database. For more information, see <a href="https://en.wikipedia.org/wiki/Tz_database">tz database</a>.</p>
    pub fn metric_timezone(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.metric_timezone = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The time zone to use for the metric. This is useful to enable the model to automatically account for daylight savings time changes if the metric is sensitive to such time changes.</p>
    /// <p>To specify a time zone, use the name of the time zone as specified in the standard tz database. For more information, see <a href="https://en.wikipedia.org/wiki/Tz_database">tz database</a>.</p>
    pub fn set_metric_timezone(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.metric_timezone = input;
        self
    }
    /// Consumes the builder and constructs a [`AnomalyDetectorConfiguration`](crate::types::AnomalyDetectorConfiguration).
    pub fn build(self) -> crate::types::AnomalyDetectorConfiguration {
        crate::types::AnomalyDetectorConfiguration {
            excluded_time_ranges: self.excluded_time_ranges,
            metric_timezone: self.metric_timezone,
        }
    }
}
