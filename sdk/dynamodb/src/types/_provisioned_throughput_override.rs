// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Replica-specific provisioned throughput settings. If not specified, uses the source table's provisioned throughput settings.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProvisionedThroughputOverride {
    /// <p>Replica-specific read capacity units. If not specified, uses the source table's read capacity settings.</p>
    #[doc(hidden)]
    pub read_capacity_units: ::std::option::Option<i64>,
}
impl ProvisionedThroughputOverride {
    /// <p>Replica-specific read capacity units. If not specified, uses the source table's read capacity settings.</p>
    pub fn read_capacity_units(&self) -> ::std::option::Option<i64> {
        self.read_capacity_units
    }
}
impl ProvisionedThroughputOverride {
    /// Creates a new builder-style object to manufacture [`ProvisionedThroughputOverride`](crate::types::ProvisionedThroughputOverride).
    pub fn builder() -> crate::types::builders::ProvisionedThroughputOverrideBuilder {
        crate::types::builders::ProvisionedThroughputOverrideBuilder::default()
    }
}

/// A builder for [`ProvisionedThroughputOverride`](crate::types::ProvisionedThroughputOverride).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ProvisionedThroughputOverrideBuilder {
    pub(crate) read_capacity_units: ::std::option::Option<i64>,
}
impl ProvisionedThroughputOverrideBuilder {
    /// <p>Replica-specific read capacity units. If not specified, uses the source table's read capacity settings.</p>
    pub fn read_capacity_units(mut self, input: i64) -> Self {
        self.read_capacity_units = ::std::option::Option::Some(input);
        self
    }
    /// <p>Replica-specific read capacity units. If not specified, uses the source table's read capacity settings.</p>
    pub fn set_read_capacity_units(mut self, input: ::std::option::Option<i64>) -> Self {
        self.read_capacity_units = input;
        self
    }
    /// Consumes the builder and constructs a [`ProvisionedThroughputOverride`](crate::types::ProvisionedThroughputOverride).
    pub fn build(self) -> crate::types::ProvisionedThroughputOverride {
        crate::types::ProvisionedThroughputOverride {
            read_capacity_units: self.read_capacity_units,
        }
    }
}
