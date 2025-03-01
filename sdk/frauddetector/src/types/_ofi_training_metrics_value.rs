// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The Online Fraud Insights (OFI) model training metric details. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OfiTrainingMetricsValue {
    /// <p> The model's performance metrics data points. </p>
    #[doc(hidden)]
    pub metric_data_points:
        ::std::option::Option<::std::vec::Vec<crate::types::OfiMetricDataPoint>>,
    /// <p> The model's overall performance score. </p>
    #[doc(hidden)]
    pub model_performance: ::std::option::Option<crate::types::OfiModelPerformance>,
}
impl OfiTrainingMetricsValue {
    /// <p> The model's performance metrics data points. </p>
    pub fn metric_data_points(&self) -> ::std::option::Option<&[crate::types::OfiMetricDataPoint]> {
        self.metric_data_points.as_deref()
    }
    /// <p> The model's overall performance score. </p>
    pub fn model_performance(&self) -> ::std::option::Option<&crate::types::OfiModelPerformance> {
        self.model_performance.as_ref()
    }
}
impl OfiTrainingMetricsValue {
    /// Creates a new builder-style object to manufacture [`OfiTrainingMetricsValue`](crate::types::OfiTrainingMetricsValue).
    pub fn builder() -> crate::types::builders::OfiTrainingMetricsValueBuilder {
        crate::types::builders::OfiTrainingMetricsValueBuilder::default()
    }
}

/// A builder for [`OfiTrainingMetricsValue`](crate::types::OfiTrainingMetricsValue).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OfiTrainingMetricsValueBuilder {
    pub(crate) metric_data_points:
        ::std::option::Option<::std::vec::Vec<crate::types::OfiMetricDataPoint>>,
    pub(crate) model_performance: ::std::option::Option<crate::types::OfiModelPerformance>,
}
impl OfiTrainingMetricsValueBuilder {
    /// Appends an item to `metric_data_points`.
    ///
    /// To override the contents of this collection use [`set_metric_data_points`](Self::set_metric_data_points).
    ///
    /// <p> The model's performance metrics data points. </p>
    pub fn metric_data_points(mut self, input: crate::types::OfiMetricDataPoint) -> Self {
        let mut v = self.metric_data_points.unwrap_or_default();
        v.push(input);
        self.metric_data_points = ::std::option::Option::Some(v);
        self
    }
    /// <p> The model's performance metrics data points. </p>
    pub fn set_metric_data_points(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::OfiMetricDataPoint>>,
    ) -> Self {
        self.metric_data_points = input;
        self
    }
    /// <p> The model's overall performance score. </p>
    pub fn model_performance(mut self, input: crate::types::OfiModelPerformance) -> Self {
        self.model_performance = ::std::option::Option::Some(input);
        self
    }
    /// <p> The model's overall performance score. </p>
    pub fn set_model_performance(
        mut self,
        input: ::std::option::Option<crate::types::OfiModelPerformance>,
    ) -> Self {
        self.model_performance = input;
        self
    }
    /// Consumes the builder and constructs a [`OfiTrainingMetricsValue`](crate::types::OfiTrainingMetricsValue).
    pub fn build(self) -> crate::types::OfiTrainingMetricsValue {
        crate::types::OfiTrainingMetricsValue {
            metric_data_points: self.metric_data_points,
            model_performance: self.model_performance,
        }
    }
}
