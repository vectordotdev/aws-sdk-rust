// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The scale-out policy for the connector.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ScaleOutPolicy {
    /// <p>The CPU utilization percentage threshold at which you want connector scale out to be triggered.</p>
    #[doc(hidden)]
    pub cpu_utilization_percentage: i32,
}
impl ScaleOutPolicy {
    /// <p>The CPU utilization percentage threshold at which you want connector scale out to be triggered.</p>
    pub fn cpu_utilization_percentage(&self) -> i32 {
        self.cpu_utilization_percentage
    }
}
impl ScaleOutPolicy {
    /// Creates a new builder-style object to manufacture [`ScaleOutPolicy`](crate::types::ScaleOutPolicy).
    pub fn builder() -> crate::types::builders::ScaleOutPolicyBuilder {
        crate::types::builders::ScaleOutPolicyBuilder::default()
    }
}

/// A builder for [`ScaleOutPolicy`](crate::types::ScaleOutPolicy).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ScaleOutPolicyBuilder {
    pub(crate) cpu_utilization_percentage: ::std::option::Option<i32>,
}
impl ScaleOutPolicyBuilder {
    /// <p>The CPU utilization percentage threshold at which you want connector scale out to be triggered.</p>
    pub fn cpu_utilization_percentage(mut self, input: i32) -> Self {
        self.cpu_utilization_percentage = ::std::option::Option::Some(input);
        self
    }
    /// <p>The CPU utilization percentage threshold at which you want connector scale out to be triggered.</p>
    pub fn set_cpu_utilization_percentage(mut self, input: ::std::option::Option<i32>) -> Self {
        self.cpu_utilization_percentage = input;
        self
    }
    /// Consumes the builder and constructs a [`ScaleOutPolicy`](crate::types::ScaleOutPolicy).
    pub fn build(self) -> crate::types::ScaleOutPolicy {
        crate::types::ScaleOutPolicy {
            cpu_utilization_percentage: self.cpu_utilization_percentage.unwrap_or_default(),
        }
    }
}
