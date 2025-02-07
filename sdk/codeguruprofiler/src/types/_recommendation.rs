// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A potential improvement that was found from analyzing the profiling data.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Recommendation {
    /// <p>How many different places in the profile graph triggered a match.</p>
    #[doc(hidden)]
    pub all_matches_count: ::std::option::Option<i32>,
    /// <p>How much of the total sample count is potentially affected.</p>
    #[doc(hidden)]
    pub all_matches_sum: ::std::option::Option<f64>,
    /// <p>The pattern that analysis recognized in the profile to make this recommendation.</p>
    #[doc(hidden)]
    pub pattern: ::std::option::Option<crate::types::Pattern>,
    /// <p>List of the matches with most impact. </p>
    #[doc(hidden)]
    pub top_matches: ::std::option::Option<::std::vec::Vec<crate::types::Match>>,
    /// <p>The start time of the profile that was used by this analysis. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    #[doc(hidden)]
    pub start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>End time of the profile that was used by this analysis. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    #[doc(hidden)]
    pub end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl Recommendation {
    /// <p>How many different places in the profile graph triggered a match.</p>
    pub fn all_matches_count(&self) -> ::std::option::Option<i32> {
        self.all_matches_count
    }
    /// <p>How much of the total sample count is potentially affected.</p>
    pub fn all_matches_sum(&self) -> ::std::option::Option<f64> {
        self.all_matches_sum
    }
    /// <p>The pattern that analysis recognized in the profile to make this recommendation.</p>
    pub fn pattern(&self) -> ::std::option::Option<&crate::types::Pattern> {
        self.pattern.as_ref()
    }
    /// <p>List of the matches with most impact. </p>
    pub fn top_matches(&self) -> ::std::option::Option<&[crate::types::Match]> {
        self.top_matches.as_deref()
    }
    /// <p>The start time of the profile that was used by this analysis. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    pub fn start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p>End time of the profile that was used by this analysis. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    pub fn end_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.end_time.as_ref()
    }
}
impl Recommendation {
    /// Creates a new builder-style object to manufacture [`Recommendation`](crate::types::Recommendation).
    pub fn builder() -> crate::types::builders::RecommendationBuilder {
        crate::types::builders::RecommendationBuilder::default()
    }
}

/// A builder for [`Recommendation`](crate::types::Recommendation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RecommendationBuilder {
    pub(crate) all_matches_count: ::std::option::Option<i32>,
    pub(crate) all_matches_sum: ::std::option::Option<f64>,
    pub(crate) pattern: ::std::option::Option<crate::types::Pattern>,
    pub(crate) top_matches: ::std::option::Option<::std::vec::Vec<crate::types::Match>>,
    pub(crate) start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl RecommendationBuilder {
    /// <p>How many different places in the profile graph triggered a match.</p>
    pub fn all_matches_count(mut self, input: i32) -> Self {
        self.all_matches_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>How many different places in the profile graph triggered a match.</p>
    pub fn set_all_matches_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.all_matches_count = input;
        self
    }
    /// <p>How much of the total sample count is potentially affected.</p>
    pub fn all_matches_sum(mut self, input: f64) -> Self {
        self.all_matches_sum = ::std::option::Option::Some(input);
        self
    }
    /// <p>How much of the total sample count is potentially affected.</p>
    pub fn set_all_matches_sum(mut self, input: ::std::option::Option<f64>) -> Self {
        self.all_matches_sum = input;
        self
    }
    /// <p>The pattern that analysis recognized in the profile to make this recommendation.</p>
    pub fn pattern(mut self, input: crate::types::Pattern) -> Self {
        self.pattern = ::std::option::Option::Some(input);
        self
    }
    /// <p>The pattern that analysis recognized in the profile to make this recommendation.</p>
    pub fn set_pattern(mut self, input: ::std::option::Option<crate::types::Pattern>) -> Self {
        self.pattern = input;
        self
    }
    /// Appends an item to `top_matches`.
    ///
    /// To override the contents of this collection use [`set_top_matches`](Self::set_top_matches).
    ///
    /// <p>List of the matches with most impact. </p>
    pub fn top_matches(mut self, input: crate::types::Match) -> Self {
        let mut v = self.top_matches.unwrap_or_default();
        v.push(input);
        self.top_matches = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of the matches with most impact. </p>
    pub fn set_top_matches(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Match>>,
    ) -> Self {
        self.top_matches = input;
        self
    }
    /// <p>The start time of the profile that was used by this analysis. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The start time of the profile that was used by this analysis. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    pub fn set_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.start_time = input;
        self
    }
    /// <p>End time of the profile that was used by this analysis. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.end_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>End time of the profile that was used by this analysis. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC.</p>
    pub fn set_end_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.end_time = input;
        self
    }
    /// Consumes the builder and constructs a [`Recommendation`](crate::types::Recommendation).
    pub fn build(self) -> crate::types::Recommendation {
        crate::types::Recommendation {
            all_matches_count: self.all_matches_count,
            all_matches_sum: self.all_matches_sum,
            pattern: self.pattern,
            top_matches: self.top_matches,
            start_time: self.start_time,
            end_time: self.end_time,
        }
    }
}
