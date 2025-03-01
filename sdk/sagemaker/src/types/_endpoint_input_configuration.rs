// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The endpoint configuration for the load test.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EndpointInputConfiguration {
    /// <p>The instance types to use for the load test.</p>
    #[doc(hidden)]
    pub instance_type: ::std::option::Option<crate::types::ProductionVariantInstanceType>,
    /// <p>The inference specification name in the model package version.</p>
    #[doc(hidden)]
    pub inference_specification_name: ::std::option::Option<::std::string::String>,
    /// <p> The parameter you want to benchmark against.</p>
    #[doc(hidden)]
    pub environment_parameter_ranges:
        ::std::option::Option<crate::types::EnvironmentParameterRanges>,
}
impl EndpointInputConfiguration {
    /// <p>The instance types to use for the load test.</p>
    pub fn instance_type(
        &self,
    ) -> ::std::option::Option<&crate::types::ProductionVariantInstanceType> {
        self.instance_type.as_ref()
    }
    /// <p>The inference specification name in the model package version.</p>
    pub fn inference_specification_name(&self) -> ::std::option::Option<&str> {
        self.inference_specification_name.as_deref()
    }
    /// <p> The parameter you want to benchmark against.</p>
    pub fn environment_parameter_ranges(
        &self,
    ) -> ::std::option::Option<&crate::types::EnvironmentParameterRanges> {
        self.environment_parameter_ranges.as_ref()
    }
}
impl EndpointInputConfiguration {
    /// Creates a new builder-style object to manufacture [`EndpointInputConfiguration`](crate::types::EndpointInputConfiguration).
    pub fn builder() -> crate::types::builders::EndpointInputConfigurationBuilder {
        crate::types::builders::EndpointInputConfigurationBuilder::default()
    }
}

/// A builder for [`EndpointInputConfiguration`](crate::types::EndpointInputConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EndpointInputConfigurationBuilder {
    pub(crate) instance_type: ::std::option::Option<crate::types::ProductionVariantInstanceType>,
    pub(crate) inference_specification_name: ::std::option::Option<::std::string::String>,
    pub(crate) environment_parameter_ranges:
        ::std::option::Option<crate::types::EnvironmentParameterRanges>,
}
impl EndpointInputConfigurationBuilder {
    /// <p>The instance types to use for the load test.</p>
    pub fn instance_type(mut self, input: crate::types::ProductionVariantInstanceType) -> Self {
        self.instance_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The instance types to use for the load test.</p>
    pub fn set_instance_type(
        mut self,
        input: ::std::option::Option<crate::types::ProductionVariantInstanceType>,
    ) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>The inference specification name in the model package version.</p>
    pub fn inference_specification_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inference_specification_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The inference specification name in the model package version.</p>
    pub fn set_inference_specification_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inference_specification_name = input;
        self
    }
    /// <p> The parameter you want to benchmark against.</p>
    pub fn environment_parameter_ranges(
        mut self,
        input: crate::types::EnvironmentParameterRanges,
    ) -> Self {
        self.environment_parameter_ranges = ::std::option::Option::Some(input);
        self
    }
    /// <p> The parameter you want to benchmark against.</p>
    pub fn set_environment_parameter_ranges(
        mut self,
        input: ::std::option::Option<crate::types::EnvironmentParameterRanges>,
    ) -> Self {
        self.environment_parameter_ranges = input;
        self
    }
    /// Consumes the builder and constructs a [`EndpointInputConfiguration`](crate::types::EndpointInputConfiguration).
    pub fn build(self) -> crate::types::EndpointInputConfiguration {
        crate::types::EndpointInputConfiguration {
            instance_type: self.instance_type,
            inference_specification_name: self.inference_specification_name,
            environment_parameter_ranges: self.environment_parameter_ranges,
        }
    }
}
