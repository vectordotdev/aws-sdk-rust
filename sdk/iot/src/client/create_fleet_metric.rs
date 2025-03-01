// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateFleetMetric`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`metric_name(impl ::std::convert::Into<String>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::metric_name) / [`set_metric_name(Option<String>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::set_metric_name): <p>The name of the fleet metric to create.</p>
    ///   - [`query_string(impl ::std::convert::Into<String>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::query_string) / [`set_query_string(Option<String>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::set_query_string): <p>The search query string.</p>
    ///   - [`aggregation_type(AggregationType)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::aggregation_type) / [`set_aggregation_type(Option<AggregationType>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::set_aggregation_type): <p>The type of the aggregation query.</p>
    ///   - [`period(i32)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::period) / [`set_period(Option<i32>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::set_period): <p>The time in seconds between fleet metric emissions. Range [60(1 min), 86400(1 day)] and must be multiple of 60.</p>
    ///   - [`aggregation_field(impl ::std::convert::Into<String>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::aggregation_field) / [`set_aggregation_field(Option<String>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::set_aggregation_field): <p>The field to aggregate.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::set_description): <p>The fleet metric description.</p>
    ///   - [`query_version(impl ::std::convert::Into<String>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::query_version) / [`set_query_version(Option<String>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::set_query_version): <p>The query version.</p>
    ///   - [`index_name(impl ::std::convert::Into<String>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::index_name) / [`set_index_name(Option<String>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::set_index_name): <p>The name of the index to search.</p>
    ///   - [`unit(FleetMetricUnit)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::unit) / [`set_unit(Option<FleetMetricUnit>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::set_unit): <p>Used to support unit transformation such as milliseconds to seconds. The unit must be supported by <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_MetricDatum.html">CW metric</a>. Default to null.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::set_tags): <p>Metadata, which can be used to manage the fleet metric.</p>
    /// - On success, responds with [`CreateFleetMetricOutput`](crate::operation::create_fleet_metric::CreateFleetMetricOutput) with field(s):
    ///   - [`metric_name(Option<String>)`](crate::operation::create_fleet_metric::CreateFleetMetricOutput::metric_name): <p>The name of the fleet metric to create.</p>
    ///   - [`metric_arn(Option<String>)`](crate::operation::create_fleet_metric::CreateFleetMetricOutput::metric_arn): <p>The Amazon Resource Name (ARN) of the new fleet metric.</p>
    /// - On failure, responds with [`SdkError<CreateFleetMetricError>`](crate::operation::create_fleet_metric::CreateFleetMetricError)
    pub fn create_fleet_metric(
        &self,
    ) -> crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder {
        crate::operation::create_fleet_metric::builders::CreateFleetMetricFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
