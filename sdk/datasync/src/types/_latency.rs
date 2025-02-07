// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The latency peaks for an on-premises storage system resource. Each data point represents the 95th percentile peak value during a 1-hour interval.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Latency {
    /// <p>Peak latency for read operations.</p>
    #[doc(hidden)]
    pub read: ::std::option::Option<f64>,
    /// <p>Peak latency for write operations.</p>
    #[doc(hidden)]
    pub write: ::std::option::Option<f64>,
    /// <p>Peak latency for operations unrelated to read and write operations.</p>
    #[doc(hidden)]
    pub other: ::std::option::Option<f64>,
}
impl Latency {
    /// <p>Peak latency for read operations.</p>
    pub fn read(&self) -> ::std::option::Option<f64> {
        self.read
    }
    /// <p>Peak latency for write operations.</p>
    pub fn write(&self) -> ::std::option::Option<f64> {
        self.write
    }
    /// <p>Peak latency for operations unrelated to read and write operations.</p>
    pub fn other(&self) -> ::std::option::Option<f64> {
        self.other
    }
}
impl Latency {
    /// Creates a new builder-style object to manufacture [`Latency`](crate::types::Latency).
    pub fn builder() -> crate::types::builders::LatencyBuilder {
        crate::types::builders::LatencyBuilder::default()
    }
}

/// A builder for [`Latency`](crate::types::Latency).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LatencyBuilder {
    pub(crate) read: ::std::option::Option<f64>,
    pub(crate) write: ::std::option::Option<f64>,
    pub(crate) other: ::std::option::Option<f64>,
}
impl LatencyBuilder {
    /// <p>Peak latency for read operations.</p>
    pub fn read(mut self, input: f64) -> Self {
        self.read = ::std::option::Option::Some(input);
        self
    }
    /// <p>Peak latency for read operations.</p>
    pub fn set_read(mut self, input: ::std::option::Option<f64>) -> Self {
        self.read = input;
        self
    }
    /// <p>Peak latency for write operations.</p>
    pub fn write(mut self, input: f64) -> Self {
        self.write = ::std::option::Option::Some(input);
        self
    }
    /// <p>Peak latency for write operations.</p>
    pub fn set_write(mut self, input: ::std::option::Option<f64>) -> Self {
        self.write = input;
        self
    }
    /// <p>Peak latency for operations unrelated to read and write operations.</p>
    pub fn other(mut self, input: f64) -> Self {
        self.other = ::std::option::Option::Some(input);
        self
    }
    /// <p>Peak latency for operations unrelated to read and write operations.</p>
    pub fn set_other(mut self, input: ::std::option::Option<f64>) -> Self {
        self.other = input;
        self
    }
    /// Consumes the builder and constructs a [`Latency`](crate::types::Latency).
    pub fn build(self) -> crate::types::Latency {
        crate::types::Latency {
            read: self.read,
            write: self.write,
            other: self.other,
        }
    }
}
