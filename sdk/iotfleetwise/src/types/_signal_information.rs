// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a signal.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SignalInformation {
    /// <p>The name of the signal.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of samples to collect.</p>
    #[doc(hidden)]
    pub max_sample_count: ::std::option::Option<i64>,
    /// <p>The minimum duration of time (in milliseconds) between two triggering events to collect data.</p> <note>
    /// <p>If a signal changes often, you might want to collect data at a slower rate.</p>
    /// </note>
    #[doc(hidden)]
    pub minimum_sampling_interval_ms: ::std::option::Option<i64>,
}
impl SignalInformation {
    /// <p>The name of the signal.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The maximum number of samples to collect.</p>
    pub fn max_sample_count(&self) -> ::std::option::Option<i64> {
        self.max_sample_count
    }
    /// <p>The minimum duration of time (in milliseconds) between two triggering events to collect data.</p> <note>
    /// <p>If a signal changes often, you might want to collect data at a slower rate.</p>
    /// </note>
    pub fn minimum_sampling_interval_ms(&self) -> ::std::option::Option<i64> {
        self.minimum_sampling_interval_ms
    }
}
impl SignalInformation {
    /// Creates a new builder-style object to manufacture [`SignalInformation`](crate::types::SignalInformation).
    pub fn builder() -> crate::types::builders::SignalInformationBuilder {
        crate::types::builders::SignalInformationBuilder::default()
    }
}

/// A builder for [`SignalInformation`](crate::types::SignalInformation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SignalInformationBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) max_sample_count: ::std::option::Option<i64>,
    pub(crate) minimum_sampling_interval_ms: ::std::option::Option<i64>,
}
impl SignalInformationBuilder {
    /// <p>The name of the signal.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the signal.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The maximum number of samples to collect.</p>
    pub fn max_sample_count(mut self, input: i64) -> Self {
        self.max_sample_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of samples to collect.</p>
    pub fn set_max_sample_count(mut self, input: ::std::option::Option<i64>) -> Self {
        self.max_sample_count = input;
        self
    }
    /// <p>The minimum duration of time (in milliseconds) between two triggering events to collect data.</p> <note>
    /// <p>If a signal changes often, you might want to collect data at a slower rate.</p>
    /// </note>
    pub fn minimum_sampling_interval_ms(mut self, input: i64) -> Self {
        self.minimum_sampling_interval_ms = ::std::option::Option::Some(input);
        self
    }
    /// <p>The minimum duration of time (in milliseconds) between two triggering events to collect data.</p> <note>
    /// <p>If a signal changes often, you might want to collect data at a slower rate.</p>
    /// </note>
    pub fn set_minimum_sampling_interval_ms(mut self, input: ::std::option::Option<i64>) -> Self {
        self.minimum_sampling_interval_ms = input;
        self
    }
    /// Consumes the builder and constructs a [`SignalInformation`](crate::types::SignalInformation).
    pub fn build(self) -> crate::types::SignalInformation {
        crate::types::SignalInformation {
            name: self.name,
            max_sample_count: self.max_sample_count,
            minimum_sampling_interval_ms: self.minimum_sampling_interval_ms,
        }
    }
}
