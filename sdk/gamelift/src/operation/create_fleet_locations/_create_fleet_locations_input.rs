// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateFleetLocationsInput {
    /// <p>A unique identifier for the fleet to add locations to. You can use either the fleet ID or ARN value.</p>
    #[doc(hidden)]
    pub fleet_id: ::std::option::Option<::std::string::String>,
    /// <p>A list of locations to deploy additional instances to and manage as part of the fleet. You can add any Amazon GameLift-supported Amazon Web Services Region as a remote location, in the form of an Amazon Web Services Region code such as <code>us-west-2</code>. </p>
    #[doc(hidden)]
    pub locations: ::std::option::Option<::std::vec::Vec<crate::types::LocationConfiguration>>,
}
impl CreateFleetLocationsInput {
    /// <p>A unique identifier for the fleet to add locations to. You can use either the fleet ID or ARN value.</p>
    pub fn fleet_id(&self) -> ::std::option::Option<&str> {
        self.fleet_id.as_deref()
    }
    /// <p>A list of locations to deploy additional instances to and manage as part of the fleet. You can add any Amazon GameLift-supported Amazon Web Services Region as a remote location, in the form of an Amazon Web Services Region code such as <code>us-west-2</code>. </p>
    pub fn locations(&self) -> ::std::option::Option<&[crate::types::LocationConfiguration]> {
        self.locations.as_deref()
    }
}
impl CreateFleetLocationsInput {
    /// Creates a new builder-style object to manufacture [`CreateFleetLocationsInput`](crate::operation::create_fleet_locations::CreateFleetLocationsInput).
    pub fn builder(
    ) -> crate::operation::create_fleet_locations::builders::CreateFleetLocationsInputBuilder {
        crate::operation::create_fleet_locations::builders::CreateFleetLocationsInputBuilder::default()
    }
}

/// A builder for [`CreateFleetLocationsInput`](crate::operation::create_fleet_locations::CreateFleetLocationsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateFleetLocationsInputBuilder {
    pub(crate) fleet_id: ::std::option::Option<::std::string::String>,
    pub(crate) locations:
        ::std::option::Option<::std::vec::Vec<crate::types::LocationConfiguration>>,
}
impl CreateFleetLocationsInputBuilder {
    /// <p>A unique identifier for the fleet to add locations to. You can use either the fleet ID or ARN value.</p>
    pub fn fleet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fleet_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the fleet to add locations to. You can use either the fleet ID or ARN value.</p>
    pub fn set_fleet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fleet_id = input;
        self
    }
    /// Appends an item to `locations`.
    ///
    /// To override the contents of this collection use [`set_locations`](Self::set_locations).
    ///
    /// <p>A list of locations to deploy additional instances to and manage as part of the fleet. You can add any Amazon GameLift-supported Amazon Web Services Region as a remote location, in the form of an Amazon Web Services Region code such as <code>us-west-2</code>. </p>
    pub fn locations(mut self, input: crate::types::LocationConfiguration) -> Self {
        let mut v = self.locations.unwrap_or_default();
        v.push(input);
        self.locations = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of locations to deploy additional instances to and manage as part of the fleet. You can add any Amazon GameLift-supported Amazon Web Services Region as a remote location, in the form of an Amazon Web Services Region code such as <code>us-west-2</code>. </p>
    pub fn set_locations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::LocationConfiguration>>,
    ) -> Self {
        self.locations = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateFleetLocationsInput`](crate::operation::create_fleet_locations::CreateFleetLocationsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_fleet_locations::CreateFleetLocationsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_fleet_locations::CreateFleetLocationsInput {
                fleet_id: self.fleet_id,
                locations: self.locations,
            },
        )
    }
}
