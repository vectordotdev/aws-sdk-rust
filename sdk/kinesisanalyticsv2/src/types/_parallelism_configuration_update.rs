// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes updates to parameters for how an application executes multiple tasks simultaneously.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ParallelismConfigurationUpdate {
    /// <p>Describes updates to whether the application uses the default parallelism for the Kinesis Data Analytics service, or if a custom parallelism is used. You must set this property to <code>CUSTOM</code> in order to change your application's <code>AutoScalingEnabled</code>, <code>Parallelism</code>, or <code>ParallelismPerKPU</code> properties.</p>
    #[doc(hidden)]
    pub configuration_type_update: ::std::option::Option<crate::types::ConfigurationType>,
    /// <p>Describes updates to the initial number of parallel tasks an application can perform. If <code>AutoScalingEnabled</code> is set to True, then Kinesis Data Analytics can increase the <code>CurrentParallelism</code> value in response to application load. The service can increase <code>CurrentParallelism</code> up to the maximum parallelism, which is <code>ParalellismPerKPU</code> times the maximum KPUs for the application. The maximum KPUs for an application is 32 by default, and can be increased by requesting a limit increase. If application load is reduced, the service will reduce <code>CurrentParallelism</code> down to the <code>Parallelism</code> setting.</p>
    #[doc(hidden)]
    pub parallelism_update: ::std::option::Option<i32>,
    /// <p>Describes updates to the number of parallel tasks an application can perform per Kinesis Processing Unit (KPU) used by the application.</p>
    #[doc(hidden)]
    pub parallelism_per_kpu_update: ::std::option::Option<i32>,
    /// <p>Describes updates to whether the Kinesis Data Analytics service can increase the parallelism of a Flink-based Kinesis Data Analytics application in response to increased throughput.</p>
    #[doc(hidden)]
    pub auto_scaling_enabled_update: ::std::option::Option<bool>,
}
impl ParallelismConfigurationUpdate {
    /// <p>Describes updates to whether the application uses the default parallelism for the Kinesis Data Analytics service, or if a custom parallelism is used. You must set this property to <code>CUSTOM</code> in order to change your application's <code>AutoScalingEnabled</code>, <code>Parallelism</code>, or <code>ParallelismPerKPU</code> properties.</p>
    pub fn configuration_type_update(
        &self,
    ) -> ::std::option::Option<&crate::types::ConfigurationType> {
        self.configuration_type_update.as_ref()
    }
    /// <p>Describes updates to the initial number of parallel tasks an application can perform. If <code>AutoScalingEnabled</code> is set to True, then Kinesis Data Analytics can increase the <code>CurrentParallelism</code> value in response to application load. The service can increase <code>CurrentParallelism</code> up to the maximum parallelism, which is <code>ParalellismPerKPU</code> times the maximum KPUs for the application. The maximum KPUs for an application is 32 by default, and can be increased by requesting a limit increase. If application load is reduced, the service will reduce <code>CurrentParallelism</code> down to the <code>Parallelism</code> setting.</p>
    pub fn parallelism_update(&self) -> ::std::option::Option<i32> {
        self.parallelism_update
    }
    /// <p>Describes updates to the number of parallel tasks an application can perform per Kinesis Processing Unit (KPU) used by the application.</p>
    pub fn parallelism_per_kpu_update(&self) -> ::std::option::Option<i32> {
        self.parallelism_per_kpu_update
    }
    /// <p>Describes updates to whether the Kinesis Data Analytics service can increase the parallelism of a Flink-based Kinesis Data Analytics application in response to increased throughput.</p>
    pub fn auto_scaling_enabled_update(&self) -> ::std::option::Option<bool> {
        self.auto_scaling_enabled_update
    }
}
impl ParallelismConfigurationUpdate {
    /// Creates a new builder-style object to manufacture [`ParallelismConfigurationUpdate`](crate::types::ParallelismConfigurationUpdate).
    pub fn builder() -> crate::types::builders::ParallelismConfigurationUpdateBuilder {
        crate::types::builders::ParallelismConfigurationUpdateBuilder::default()
    }
}

/// A builder for [`ParallelismConfigurationUpdate`](crate::types::ParallelismConfigurationUpdate).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ParallelismConfigurationUpdateBuilder {
    pub(crate) configuration_type_update: ::std::option::Option<crate::types::ConfigurationType>,
    pub(crate) parallelism_update: ::std::option::Option<i32>,
    pub(crate) parallelism_per_kpu_update: ::std::option::Option<i32>,
    pub(crate) auto_scaling_enabled_update: ::std::option::Option<bool>,
}
impl ParallelismConfigurationUpdateBuilder {
    /// <p>Describes updates to whether the application uses the default parallelism for the Kinesis Data Analytics service, or if a custom parallelism is used. You must set this property to <code>CUSTOM</code> in order to change your application's <code>AutoScalingEnabled</code>, <code>Parallelism</code>, or <code>ParallelismPerKPU</code> properties.</p>
    pub fn configuration_type_update(mut self, input: crate::types::ConfigurationType) -> Self {
        self.configuration_type_update = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes updates to whether the application uses the default parallelism for the Kinesis Data Analytics service, or if a custom parallelism is used. You must set this property to <code>CUSTOM</code> in order to change your application's <code>AutoScalingEnabled</code>, <code>Parallelism</code>, or <code>ParallelismPerKPU</code> properties.</p>
    pub fn set_configuration_type_update(
        mut self,
        input: ::std::option::Option<crate::types::ConfigurationType>,
    ) -> Self {
        self.configuration_type_update = input;
        self
    }
    /// <p>Describes updates to the initial number of parallel tasks an application can perform. If <code>AutoScalingEnabled</code> is set to True, then Kinesis Data Analytics can increase the <code>CurrentParallelism</code> value in response to application load. The service can increase <code>CurrentParallelism</code> up to the maximum parallelism, which is <code>ParalellismPerKPU</code> times the maximum KPUs for the application. The maximum KPUs for an application is 32 by default, and can be increased by requesting a limit increase. If application load is reduced, the service will reduce <code>CurrentParallelism</code> down to the <code>Parallelism</code> setting.</p>
    pub fn parallelism_update(mut self, input: i32) -> Self {
        self.parallelism_update = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes updates to the initial number of parallel tasks an application can perform. If <code>AutoScalingEnabled</code> is set to True, then Kinesis Data Analytics can increase the <code>CurrentParallelism</code> value in response to application load. The service can increase <code>CurrentParallelism</code> up to the maximum parallelism, which is <code>ParalellismPerKPU</code> times the maximum KPUs for the application. The maximum KPUs for an application is 32 by default, and can be increased by requesting a limit increase. If application load is reduced, the service will reduce <code>CurrentParallelism</code> down to the <code>Parallelism</code> setting.</p>
    pub fn set_parallelism_update(mut self, input: ::std::option::Option<i32>) -> Self {
        self.parallelism_update = input;
        self
    }
    /// <p>Describes updates to the number of parallel tasks an application can perform per Kinesis Processing Unit (KPU) used by the application.</p>
    pub fn parallelism_per_kpu_update(mut self, input: i32) -> Self {
        self.parallelism_per_kpu_update = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes updates to the number of parallel tasks an application can perform per Kinesis Processing Unit (KPU) used by the application.</p>
    pub fn set_parallelism_per_kpu_update(mut self, input: ::std::option::Option<i32>) -> Self {
        self.parallelism_per_kpu_update = input;
        self
    }
    /// <p>Describes updates to whether the Kinesis Data Analytics service can increase the parallelism of a Flink-based Kinesis Data Analytics application in response to increased throughput.</p>
    pub fn auto_scaling_enabled_update(mut self, input: bool) -> Self {
        self.auto_scaling_enabled_update = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes updates to whether the Kinesis Data Analytics service can increase the parallelism of a Flink-based Kinesis Data Analytics application in response to increased throughput.</p>
    pub fn set_auto_scaling_enabled_update(mut self, input: ::std::option::Option<bool>) -> Self {
        self.auto_scaling_enabled_update = input;
        self
    }
    /// Consumes the builder and constructs a [`ParallelismConfigurationUpdate`](crate::types::ParallelismConfigurationUpdate).
    pub fn build(self) -> crate::types::ParallelismConfigurationUpdate {
        crate::types::ParallelismConfigurationUpdate {
            configuration_type_update: self.configuration_type_update,
            parallelism_update: self.parallelism_update,
            parallelism_per_kpu_update: self.parallelism_per_kpu_update,
            auto_scaling_enabled_update: self.auto_scaling_enabled_update,
        }
    }
}
