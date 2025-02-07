// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Sets additional capacity units configured for your rescore execution plan. A rescore execution plan is an Amazon Kendra Intelligent Ranking resource used for provisioning the <code>Rescore</code> API. You can add and remove capacity units to fit your usage requirements.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CapacityUnitsConfiguration {
    /// <p>The amount of extra capacity for your rescore execution plan.</p>
    /// <p>A single extra capacity unit for a rescore execution plan provides 0.01 rescore requests per second. You can add up to 1000 extra capacity units.</p>
    #[doc(hidden)]
    pub rescore_capacity_units: ::std::option::Option<i32>,
}
impl CapacityUnitsConfiguration {
    /// <p>The amount of extra capacity for your rescore execution plan.</p>
    /// <p>A single extra capacity unit for a rescore execution plan provides 0.01 rescore requests per second. You can add up to 1000 extra capacity units.</p>
    pub fn rescore_capacity_units(&self) -> ::std::option::Option<i32> {
        self.rescore_capacity_units
    }
}
impl CapacityUnitsConfiguration {
    /// Creates a new builder-style object to manufacture [`CapacityUnitsConfiguration`](crate::types::CapacityUnitsConfiguration).
    pub fn builder() -> crate::types::builders::CapacityUnitsConfigurationBuilder {
        crate::types::builders::CapacityUnitsConfigurationBuilder::default()
    }
}

/// A builder for [`CapacityUnitsConfiguration`](crate::types::CapacityUnitsConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CapacityUnitsConfigurationBuilder {
    pub(crate) rescore_capacity_units: ::std::option::Option<i32>,
}
impl CapacityUnitsConfigurationBuilder {
    /// <p>The amount of extra capacity for your rescore execution plan.</p>
    /// <p>A single extra capacity unit for a rescore execution plan provides 0.01 rescore requests per second. You can add up to 1000 extra capacity units.</p>
    pub fn rescore_capacity_units(mut self, input: i32) -> Self {
        self.rescore_capacity_units = ::std::option::Option::Some(input);
        self
    }
    /// <p>The amount of extra capacity for your rescore execution plan.</p>
    /// <p>A single extra capacity unit for a rescore execution plan provides 0.01 rescore requests per second. You can add up to 1000 extra capacity units.</p>
    pub fn set_rescore_capacity_units(mut self, input: ::std::option::Option<i32>) -> Self {
        self.rescore_capacity_units = input;
        self
    }
    /// Consumes the builder and constructs a [`CapacityUnitsConfiguration`](crate::types::CapacityUnitsConfiguration).
    pub fn build(self) -> crate::types::CapacityUnitsConfiguration {
        crate::types::CapacityUnitsConfiguration {
            rescore_capacity_units: self.rescore_capacity_units,
        }
    }
}
