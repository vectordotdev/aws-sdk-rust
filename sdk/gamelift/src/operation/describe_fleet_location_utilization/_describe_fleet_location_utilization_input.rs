// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeFleetLocationUtilizationInput {
    /// <p>A unique identifier for the fleet to request location utilization for. You can use either the fleet ID or ARN value.</p>
    #[doc(hidden)]
    pub fleet_id: ::std::option::Option<::std::string::String>,
    /// <p>The fleet location to retrieve utilization information for. Specify a location in the form of an Amazon Web Services Region code, such as <code>us-west-2</code>.</p>
    #[doc(hidden)]
    pub location: ::std::option::Option<::std::string::String>,
}
impl DescribeFleetLocationUtilizationInput {
    /// <p>A unique identifier for the fleet to request location utilization for. You can use either the fleet ID or ARN value.</p>
    pub fn fleet_id(&self) -> ::std::option::Option<&str> {
        self.fleet_id.as_deref()
    }
    /// <p>The fleet location to retrieve utilization information for. Specify a location in the form of an Amazon Web Services Region code, such as <code>us-west-2</code>.</p>
    pub fn location(&self) -> ::std::option::Option<&str> {
        self.location.as_deref()
    }
}
impl DescribeFleetLocationUtilizationInput {
    /// Creates a new builder-style object to manufacture [`DescribeFleetLocationUtilizationInput`](crate::operation::describe_fleet_location_utilization::DescribeFleetLocationUtilizationInput).
    pub fn builder() -> crate::operation::describe_fleet_location_utilization::builders::DescribeFleetLocationUtilizationInputBuilder{
        crate::operation::describe_fleet_location_utilization::builders::DescribeFleetLocationUtilizationInputBuilder::default()
    }
}

/// A builder for [`DescribeFleetLocationUtilizationInput`](crate::operation::describe_fleet_location_utilization::DescribeFleetLocationUtilizationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeFleetLocationUtilizationInputBuilder {
    pub(crate) fleet_id: ::std::option::Option<::std::string::String>,
    pub(crate) location: ::std::option::Option<::std::string::String>,
}
impl DescribeFleetLocationUtilizationInputBuilder {
    /// <p>A unique identifier for the fleet to request location utilization for. You can use either the fleet ID or ARN value.</p>
    pub fn fleet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fleet_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the fleet to request location utilization for. You can use either the fleet ID or ARN value.</p>
    pub fn set_fleet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fleet_id = input;
        self
    }
    /// <p>The fleet location to retrieve utilization information for. Specify a location in the form of an Amazon Web Services Region code, such as <code>us-west-2</code>.</p>
    pub fn location(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.location = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The fleet location to retrieve utilization information for. Specify a location in the form of an Amazon Web Services Region code, such as <code>us-west-2</code>.</p>
    pub fn set_location(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.location = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeFleetLocationUtilizationInput`](crate::operation::describe_fleet_location_utilization::DescribeFleetLocationUtilizationInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::describe_fleet_location_utilization::DescribeFleetLocationUtilizationInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::describe_fleet_location_utilization::DescribeFleetLocationUtilizationInput {
                fleet_id: self.fleet_id
                ,
                location: self.location
                ,
            }
        )
    }
}
