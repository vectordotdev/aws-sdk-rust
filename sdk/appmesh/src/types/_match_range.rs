// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents the range of values to match on. The first character of the range is included in the range, though the last character is not. For example, if the range specified were 1-100, only values 1-99 would be matched.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MatchRange {
    /// <p>The start of the range.</p>
    #[doc(hidden)]
    pub start: ::std::option::Option<i64>,
    /// <p>The end of the range.</p>
    #[doc(hidden)]
    pub end: ::std::option::Option<i64>,
}
impl MatchRange {
    /// <p>The start of the range.</p>
    pub fn start(&self) -> ::std::option::Option<i64> {
        self.start
    }
    /// <p>The end of the range.</p>
    pub fn end(&self) -> ::std::option::Option<i64> {
        self.end
    }
}
impl MatchRange {
    /// Creates a new builder-style object to manufacture [`MatchRange`](crate::types::MatchRange).
    pub fn builder() -> crate::types::builders::MatchRangeBuilder {
        crate::types::builders::MatchRangeBuilder::default()
    }
}

/// A builder for [`MatchRange`](crate::types::MatchRange).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MatchRangeBuilder {
    pub(crate) start: ::std::option::Option<i64>,
    pub(crate) end: ::std::option::Option<i64>,
}
impl MatchRangeBuilder {
    /// <p>The start of the range.</p>
    pub fn start(mut self, input: i64) -> Self {
        self.start = ::std::option::Option::Some(input);
        self
    }
    /// <p>The start of the range.</p>
    pub fn set_start(mut self, input: ::std::option::Option<i64>) -> Self {
        self.start = input;
        self
    }
    /// <p>The end of the range.</p>
    pub fn end(mut self, input: i64) -> Self {
        self.end = ::std::option::Option::Some(input);
        self
    }
    /// <p>The end of the range.</p>
    pub fn set_end(mut self, input: ::std::option::Option<i64>) -> Self {
        self.end = input;
        self
    }
    /// Consumes the builder and constructs a [`MatchRange`](crate::types::MatchRange).
    pub fn build(self) -> crate::types::MatchRange {
        crate::types::MatchRange {
            start: self.start,
            end: self.end,
        }
    }
}
