// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The value to use in an Amazon CloudWatch custom metric dimension. This is used in the <code>PublishMetrics</code> <code>CustomAction</code>. A CloudWatch custom metric dimension is a name/value pair that's part of the identity of a metric. </p>
/// <p>Network Firewall sets the dimension name to <code>CustomAction</code> and you provide the dimension value. </p>
/// <p>For more information about CloudWatch custom metric dimensions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html#usingDimensions">Publishing Custom Metrics</a> in the <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/WhatIsCloudWatch.html">Amazon CloudWatch User Guide</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Dimension {
    /// <p>The value to use in the custom metric dimension.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl Dimension {
    /// <p>The value to use in the custom metric dimension.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl Dimension {
    /// Creates a new builder-style object to manufacture [`Dimension`](crate::types::Dimension).
    pub fn builder() -> crate::types::builders::DimensionBuilder {
        crate::types::builders::DimensionBuilder::default()
    }
}

/// A builder for [`Dimension`](crate::types::Dimension).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DimensionBuilder {
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl DimensionBuilder {
    /// <p>The value to use in the custom metric dimension.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value to use in the custom metric dimension.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`Dimension`](crate::types::Dimension).
    pub fn build(self) -> crate::types::Dimension {
        crate::types::Dimension { value: self.value }
    }
}
