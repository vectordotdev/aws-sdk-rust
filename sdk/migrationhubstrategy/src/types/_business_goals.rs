// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Business goals that you specify. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BusinessGoals {
    /// <p> Business goal to achieve migration at a fast pace. </p>
    #[doc(hidden)]
    pub speed_of_migration: ::std::option::Option<i32>,
    /// <p> Business goal to reduce the operational overhead on the team by moving into managed services. </p>
    #[doc(hidden)]
    pub reduce_operational_overhead_with_managed_services: ::std::option::Option<i32>,
    /// <p> Business goal to modernize infrastructure by moving to cloud native technologies. </p>
    #[doc(hidden)]
    pub modernize_infrastructure_with_cloud_native_technologies: ::std::option::Option<i32>,
    /// <p> Business goal to reduce license costs. </p>
    #[doc(hidden)]
    pub license_cost_reduction: ::std::option::Option<i32>,
}
impl BusinessGoals {
    /// <p> Business goal to achieve migration at a fast pace. </p>
    pub fn speed_of_migration(&self) -> ::std::option::Option<i32> {
        self.speed_of_migration
    }
    /// <p> Business goal to reduce the operational overhead on the team by moving into managed services. </p>
    pub fn reduce_operational_overhead_with_managed_services(&self) -> ::std::option::Option<i32> {
        self.reduce_operational_overhead_with_managed_services
    }
    /// <p> Business goal to modernize infrastructure by moving to cloud native technologies. </p>
    pub fn modernize_infrastructure_with_cloud_native_technologies(
        &self,
    ) -> ::std::option::Option<i32> {
        self.modernize_infrastructure_with_cloud_native_technologies
    }
    /// <p> Business goal to reduce license costs. </p>
    pub fn license_cost_reduction(&self) -> ::std::option::Option<i32> {
        self.license_cost_reduction
    }
}
impl BusinessGoals {
    /// Creates a new builder-style object to manufacture [`BusinessGoals`](crate::types::BusinessGoals).
    pub fn builder() -> crate::types::builders::BusinessGoalsBuilder {
        crate::types::builders::BusinessGoalsBuilder::default()
    }
}

/// A builder for [`BusinessGoals`](crate::types::BusinessGoals).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BusinessGoalsBuilder {
    pub(crate) speed_of_migration: ::std::option::Option<i32>,
    pub(crate) reduce_operational_overhead_with_managed_services: ::std::option::Option<i32>,
    pub(crate) modernize_infrastructure_with_cloud_native_technologies: ::std::option::Option<i32>,
    pub(crate) license_cost_reduction: ::std::option::Option<i32>,
}
impl BusinessGoalsBuilder {
    /// <p> Business goal to achieve migration at a fast pace. </p>
    pub fn speed_of_migration(mut self, input: i32) -> Self {
        self.speed_of_migration = ::std::option::Option::Some(input);
        self
    }
    /// <p> Business goal to achieve migration at a fast pace. </p>
    pub fn set_speed_of_migration(mut self, input: ::std::option::Option<i32>) -> Self {
        self.speed_of_migration = input;
        self
    }
    /// <p> Business goal to reduce the operational overhead on the team by moving into managed services. </p>
    pub fn reduce_operational_overhead_with_managed_services(mut self, input: i32) -> Self {
        self.reduce_operational_overhead_with_managed_services = ::std::option::Option::Some(input);
        self
    }
    /// <p> Business goal to reduce the operational overhead on the team by moving into managed services. </p>
    pub fn set_reduce_operational_overhead_with_managed_services(
        mut self,
        input: ::std::option::Option<i32>,
    ) -> Self {
        self.reduce_operational_overhead_with_managed_services = input;
        self
    }
    /// <p> Business goal to modernize infrastructure by moving to cloud native technologies. </p>
    pub fn modernize_infrastructure_with_cloud_native_technologies(mut self, input: i32) -> Self {
        self.modernize_infrastructure_with_cloud_native_technologies =
            ::std::option::Option::Some(input);
        self
    }
    /// <p> Business goal to modernize infrastructure by moving to cloud native technologies. </p>
    pub fn set_modernize_infrastructure_with_cloud_native_technologies(
        mut self,
        input: ::std::option::Option<i32>,
    ) -> Self {
        self.modernize_infrastructure_with_cloud_native_technologies = input;
        self
    }
    /// <p> Business goal to reduce license costs. </p>
    pub fn license_cost_reduction(mut self, input: i32) -> Self {
        self.license_cost_reduction = ::std::option::Option::Some(input);
        self
    }
    /// <p> Business goal to reduce license costs. </p>
    pub fn set_license_cost_reduction(mut self, input: ::std::option::Option<i32>) -> Self {
        self.license_cost_reduction = input;
        self
    }
    /// Consumes the builder and constructs a [`BusinessGoals`](crate::types::BusinessGoals).
    pub fn build(self) -> crate::types::BusinessGoals {
        crate::types::BusinessGoals {
            speed_of_migration: self.speed_of_migration,
            reduce_operational_overhead_with_managed_services: self
                .reduce_operational_overhead_with_managed_services,
            modernize_infrastructure_with_cloud_native_technologies: self
                .modernize_infrastructure_with_cloud_native_technologies,
            license_cost_reduction: self.license_cost_reduction,
        }
    }
}
