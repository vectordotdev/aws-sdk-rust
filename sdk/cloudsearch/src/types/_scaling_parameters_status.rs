// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The status and configuration of a search domain's scaling parameters. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ScalingParametersStatus {
    /// <p>The desired instance type and desired number of replicas of each index partition.</p>
    #[doc(hidden)]
    pub options: ::std::option::Option<crate::types::ScalingParameters>,
    /// <p>The status of domain configuration option.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::OptionStatus>,
}
impl ScalingParametersStatus {
    /// <p>The desired instance type and desired number of replicas of each index partition.</p>
    pub fn options(&self) -> ::std::option::Option<&crate::types::ScalingParameters> {
        self.options.as_ref()
    }
    /// <p>The status of domain configuration option.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::OptionStatus> {
        self.status.as_ref()
    }
}
impl ScalingParametersStatus {
    /// Creates a new builder-style object to manufacture [`ScalingParametersStatus`](crate::types::ScalingParametersStatus).
    pub fn builder() -> crate::types::builders::ScalingParametersStatusBuilder {
        crate::types::builders::ScalingParametersStatusBuilder::default()
    }
}

/// A builder for [`ScalingParametersStatus`](crate::types::ScalingParametersStatus).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ScalingParametersStatusBuilder {
    pub(crate) options: ::std::option::Option<crate::types::ScalingParameters>,
    pub(crate) status: ::std::option::Option<crate::types::OptionStatus>,
}
impl ScalingParametersStatusBuilder {
    /// <p>The desired instance type and desired number of replicas of each index partition.</p>
    pub fn options(mut self, input: crate::types::ScalingParameters) -> Self {
        self.options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The desired instance type and desired number of replicas of each index partition.</p>
    pub fn set_options(
        mut self,
        input: ::std::option::Option<crate::types::ScalingParameters>,
    ) -> Self {
        self.options = input;
        self
    }
    /// <p>The status of domain configuration option.</p>
    pub fn status(mut self, input: crate::types::OptionStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of domain configuration option.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::OptionStatus>) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`ScalingParametersStatus`](crate::types::ScalingParametersStatus).
    pub fn build(self) -> crate::types::ScalingParametersStatus {
        crate::types::ScalingParametersStatus {
            options: self.options,
            status: self.status,
        }
    }
}
