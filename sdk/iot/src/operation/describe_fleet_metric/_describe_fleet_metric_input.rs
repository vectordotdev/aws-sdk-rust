// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeFleetMetricInput {
    /// <p>The name of the fleet metric to describe.</p>
    #[doc(hidden)]
    pub metric_name: ::std::option::Option<::std::string::String>,
}
impl DescribeFleetMetricInput {
    /// <p>The name of the fleet metric to describe.</p>
    pub fn metric_name(&self) -> ::std::option::Option<&str> {
        self.metric_name.as_deref()
    }
}
impl DescribeFleetMetricInput {
    /// Creates a new builder-style object to manufacture [`DescribeFleetMetricInput`](crate::operation::describe_fleet_metric::DescribeFleetMetricInput).
    pub fn builder(
    ) -> crate::operation::describe_fleet_metric::builders::DescribeFleetMetricInputBuilder {
        crate::operation::describe_fleet_metric::builders::DescribeFleetMetricInputBuilder::default(
        )
    }
}

/// A builder for [`DescribeFleetMetricInput`](crate::operation::describe_fleet_metric::DescribeFleetMetricInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeFleetMetricInputBuilder {
    pub(crate) metric_name: ::std::option::Option<::std::string::String>,
}
impl DescribeFleetMetricInputBuilder {
    /// <p>The name of the fleet metric to describe.</p>
    pub fn metric_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.metric_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the fleet metric to describe.</p>
    pub fn set_metric_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.metric_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeFleetMetricInput`](crate::operation::describe_fleet_metric::DescribeFleetMetricInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_fleet_metric::DescribeFleetMetricInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_fleet_metric::DescribeFleetMetricInput {
                metric_name: self.metric_name,
            },
        )
    }
}
