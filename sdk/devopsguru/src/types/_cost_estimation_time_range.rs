// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The time range of a cost estimation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CostEstimationTimeRange {
    /// <p>The start time of the cost estimation.</p>
    #[doc(hidden)]
    pub start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The end time of the cost estimation.</p>
    #[doc(hidden)]
    pub end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl CostEstimationTimeRange {
    /// <p>The start time of the cost estimation.</p>
    pub fn start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p>The end time of the cost estimation.</p>
    pub fn end_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.end_time.as_ref()
    }
}
impl CostEstimationTimeRange {
    /// Creates a new builder-style object to manufacture [`CostEstimationTimeRange`](crate::types::CostEstimationTimeRange).
    pub fn builder() -> crate::types::builders::CostEstimationTimeRangeBuilder {
        crate::types::builders::CostEstimationTimeRangeBuilder::default()
    }
}

/// A builder for [`CostEstimationTimeRange`](crate::types::CostEstimationTimeRange).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CostEstimationTimeRangeBuilder {
    pub(crate) start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl CostEstimationTimeRangeBuilder {
    /// <p>The start time of the cost estimation.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The start time of the cost estimation.</p>
    pub fn set_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.start_time = input;
        self
    }
    /// <p>The end time of the cost estimation.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.end_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The end time of the cost estimation.</p>
    pub fn set_end_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.end_time = input;
        self
    }
    /// Consumes the builder and constructs a [`CostEstimationTimeRange`](crate::types::CostEstimationTimeRange).
    pub fn build(self) -> crate::types::CostEstimationTimeRange {
        crate::types::CostEstimationTimeRange {
            start_time: self.start_time,
            end_time: self.end_time,
        }
    }
}
