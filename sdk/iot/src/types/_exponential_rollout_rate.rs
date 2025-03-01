// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Allows you to create an exponential rate of rollout for a job.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExponentialRolloutRate {
    /// <p>The minimum number of things that will be notified of a pending job, per minute at the start of job rollout. This parameter allows you to define the initial rate of rollout.</p>
    #[doc(hidden)]
    pub base_rate_per_minute: ::std::option::Option<i32>,
    /// <p>The exponential factor to increase the rate of rollout for a job.</p>
    /// <p>Amazon Web Services IoT Core supports up to one digit after the decimal (for example, 1.5, but not 1.55).</p>
    #[doc(hidden)]
    pub increment_factor: f64,
    /// <p>The criteria to initiate the increase in rate of rollout for a job.</p>
    #[doc(hidden)]
    pub rate_increase_criteria: ::std::option::Option<crate::types::RateIncreaseCriteria>,
}
impl ExponentialRolloutRate {
    /// <p>The minimum number of things that will be notified of a pending job, per minute at the start of job rollout. This parameter allows you to define the initial rate of rollout.</p>
    pub fn base_rate_per_minute(&self) -> ::std::option::Option<i32> {
        self.base_rate_per_minute
    }
    /// <p>The exponential factor to increase the rate of rollout for a job.</p>
    /// <p>Amazon Web Services IoT Core supports up to one digit after the decimal (for example, 1.5, but not 1.55).</p>
    pub fn increment_factor(&self) -> f64 {
        self.increment_factor
    }
    /// <p>The criteria to initiate the increase in rate of rollout for a job.</p>
    pub fn rate_increase_criteria(
        &self,
    ) -> ::std::option::Option<&crate::types::RateIncreaseCriteria> {
        self.rate_increase_criteria.as_ref()
    }
}
impl ExponentialRolloutRate {
    /// Creates a new builder-style object to manufacture [`ExponentialRolloutRate`](crate::types::ExponentialRolloutRate).
    pub fn builder() -> crate::types::builders::ExponentialRolloutRateBuilder {
        crate::types::builders::ExponentialRolloutRateBuilder::default()
    }
}

/// A builder for [`ExponentialRolloutRate`](crate::types::ExponentialRolloutRate).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExponentialRolloutRateBuilder {
    pub(crate) base_rate_per_minute: ::std::option::Option<i32>,
    pub(crate) increment_factor: ::std::option::Option<f64>,
    pub(crate) rate_increase_criteria: ::std::option::Option<crate::types::RateIncreaseCriteria>,
}
impl ExponentialRolloutRateBuilder {
    /// <p>The minimum number of things that will be notified of a pending job, per minute at the start of job rollout. This parameter allows you to define the initial rate of rollout.</p>
    pub fn base_rate_per_minute(mut self, input: i32) -> Self {
        self.base_rate_per_minute = ::std::option::Option::Some(input);
        self
    }
    /// <p>The minimum number of things that will be notified of a pending job, per minute at the start of job rollout. This parameter allows you to define the initial rate of rollout.</p>
    pub fn set_base_rate_per_minute(mut self, input: ::std::option::Option<i32>) -> Self {
        self.base_rate_per_minute = input;
        self
    }
    /// <p>The exponential factor to increase the rate of rollout for a job.</p>
    /// <p>Amazon Web Services IoT Core supports up to one digit after the decimal (for example, 1.5, but not 1.55).</p>
    pub fn increment_factor(mut self, input: f64) -> Self {
        self.increment_factor = ::std::option::Option::Some(input);
        self
    }
    /// <p>The exponential factor to increase the rate of rollout for a job.</p>
    /// <p>Amazon Web Services IoT Core supports up to one digit after the decimal (for example, 1.5, but not 1.55).</p>
    pub fn set_increment_factor(mut self, input: ::std::option::Option<f64>) -> Self {
        self.increment_factor = input;
        self
    }
    /// <p>The criteria to initiate the increase in rate of rollout for a job.</p>
    pub fn rate_increase_criteria(mut self, input: crate::types::RateIncreaseCriteria) -> Self {
        self.rate_increase_criteria = ::std::option::Option::Some(input);
        self
    }
    /// <p>The criteria to initiate the increase in rate of rollout for a job.</p>
    pub fn set_rate_increase_criteria(
        mut self,
        input: ::std::option::Option<crate::types::RateIncreaseCriteria>,
    ) -> Self {
        self.rate_increase_criteria = input;
        self
    }
    /// Consumes the builder and constructs a [`ExponentialRolloutRate`](crate::types::ExponentialRolloutRate).
    pub fn build(self) -> crate::types::ExponentialRolloutRate {
        crate::types::ExponentialRolloutRate {
            base_rate_per_minute: self.base_rate_per_minute,
            increment_factor: self.increment_factor.unwrap_or_default(),
            rate_increase_criteria: self.rate_increase_criteria,
        }
    }
}
