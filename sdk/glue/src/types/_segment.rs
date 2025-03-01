// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Defines a non-overlapping region of a table's partitions, allowing multiple requests to be run in parallel.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Segment {
    /// <p>The zero-based index number of the segment. For example, if the total number of segments is 4, <code>SegmentNumber</code> values range from 0 through 3.</p>
    #[doc(hidden)]
    pub segment_number: i32,
    /// <p>The total number of segments.</p>
    #[doc(hidden)]
    pub total_segments: i32,
}
impl Segment {
    /// <p>The zero-based index number of the segment. For example, if the total number of segments is 4, <code>SegmentNumber</code> values range from 0 through 3.</p>
    pub fn segment_number(&self) -> i32 {
        self.segment_number
    }
    /// <p>The total number of segments.</p>
    pub fn total_segments(&self) -> i32 {
        self.total_segments
    }
}
impl Segment {
    /// Creates a new builder-style object to manufacture [`Segment`](crate::types::Segment).
    pub fn builder() -> crate::types::builders::SegmentBuilder {
        crate::types::builders::SegmentBuilder::default()
    }
}

/// A builder for [`Segment`](crate::types::Segment).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SegmentBuilder {
    pub(crate) segment_number: ::std::option::Option<i32>,
    pub(crate) total_segments: ::std::option::Option<i32>,
}
impl SegmentBuilder {
    /// <p>The zero-based index number of the segment. For example, if the total number of segments is 4, <code>SegmentNumber</code> values range from 0 through 3.</p>
    pub fn segment_number(mut self, input: i32) -> Self {
        self.segment_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>The zero-based index number of the segment. For example, if the total number of segments is 4, <code>SegmentNumber</code> values range from 0 through 3.</p>
    pub fn set_segment_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.segment_number = input;
        self
    }
    /// <p>The total number of segments.</p>
    pub fn total_segments(mut self, input: i32) -> Self {
        self.total_segments = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total number of segments.</p>
    pub fn set_total_segments(mut self, input: ::std::option::Option<i32>) -> Self {
        self.total_segments = input;
        self
    }
    /// Consumes the builder and constructs a [`Segment`](crate::types::Segment).
    pub fn build(self) -> crate::types::Segment {
        crate::types::Segment {
            segment_number: self.segment_number.unwrap_or_default(),
            total_segments: self.total_segments.unwrap_or_default(),
        }
    }
}
