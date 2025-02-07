// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The section of the contact audio where that category rule was detected.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PointOfInterest {
    /// <p>The beginning offset in milliseconds where the category rule was detected.</p>
    #[doc(hidden)]
    pub begin_offset_millis: i32,
    /// <p>The ending offset in milliseconds where the category rule was detected.</p>
    #[doc(hidden)]
    pub end_offset_millis: i32,
}
impl PointOfInterest {
    /// <p>The beginning offset in milliseconds where the category rule was detected.</p>
    pub fn begin_offset_millis(&self) -> i32 {
        self.begin_offset_millis
    }
    /// <p>The ending offset in milliseconds where the category rule was detected.</p>
    pub fn end_offset_millis(&self) -> i32 {
        self.end_offset_millis
    }
}
impl PointOfInterest {
    /// Creates a new builder-style object to manufacture [`PointOfInterest`](crate::types::PointOfInterest).
    pub fn builder() -> crate::types::builders::PointOfInterestBuilder {
        crate::types::builders::PointOfInterestBuilder::default()
    }
}

/// A builder for [`PointOfInterest`](crate::types::PointOfInterest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PointOfInterestBuilder {
    pub(crate) begin_offset_millis: ::std::option::Option<i32>,
    pub(crate) end_offset_millis: ::std::option::Option<i32>,
}
impl PointOfInterestBuilder {
    /// <p>The beginning offset in milliseconds where the category rule was detected.</p>
    pub fn begin_offset_millis(mut self, input: i32) -> Self {
        self.begin_offset_millis = ::std::option::Option::Some(input);
        self
    }
    /// <p>The beginning offset in milliseconds where the category rule was detected.</p>
    pub fn set_begin_offset_millis(mut self, input: ::std::option::Option<i32>) -> Self {
        self.begin_offset_millis = input;
        self
    }
    /// <p>The ending offset in milliseconds where the category rule was detected.</p>
    pub fn end_offset_millis(mut self, input: i32) -> Self {
        self.end_offset_millis = ::std::option::Option::Some(input);
        self
    }
    /// <p>The ending offset in milliseconds where the category rule was detected.</p>
    pub fn set_end_offset_millis(mut self, input: ::std::option::Option<i32>) -> Self {
        self.end_offset_millis = input;
        self
    }
    /// Consumes the builder and constructs a [`PointOfInterest`](crate::types::PointOfInterest).
    pub fn build(self) -> crate::types::PointOfInterest {
        crate::types::PointOfInterest {
            begin_offset_millis: self.begin_offset_millis.unwrap_or_default(),
            end_offset_millis: self.end_offset_millis.unwrap_or_default(),
        }
    }
}
