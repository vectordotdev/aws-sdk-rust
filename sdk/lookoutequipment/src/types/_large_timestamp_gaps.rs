// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Entity that comprises information on large gaps between consecutive timestamps in data. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LargeTimestampGaps {
    /// <p> Indicates whether there is a potential data issue related to large gaps in timestamps. </p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::StatisticalIssueStatus>,
    /// <p> Indicates the number of large timestamp gaps, if there are any. </p>
    #[doc(hidden)]
    pub number_of_large_timestamp_gaps: ::std::option::Option<i32>,
    /// <p> Indicates the size of the largest timestamp gap, in days. </p>
    #[doc(hidden)]
    pub max_timestamp_gap_in_days: ::std::option::Option<i32>,
}
impl LargeTimestampGaps {
    /// <p> Indicates whether there is a potential data issue related to large gaps in timestamps. </p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::StatisticalIssueStatus> {
        self.status.as_ref()
    }
    /// <p> Indicates the number of large timestamp gaps, if there are any. </p>
    pub fn number_of_large_timestamp_gaps(&self) -> ::std::option::Option<i32> {
        self.number_of_large_timestamp_gaps
    }
    /// <p> Indicates the size of the largest timestamp gap, in days. </p>
    pub fn max_timestamp_gap_in_days(&self) -> ::std::option::Option<i32> {
        self.max_timestamp_gap_in_days
    }
}
impl LargeTimestampGaps {
    /// Creates a new builder-style object to manufacture [`LargeTimestampGaps`](crate::types::LargeTimestampGaps).
    pub fn builder() -> crate::types::builders::LargeTimestampGapsBuilder {
        crate::types::builders::LargeTimestampGapsBuilder::default()
    }
}

/// A builder for [`LargeTimestampGaps`](crate::types::LargeTimestampGaps).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LargeTimestampGapsBuilder {
    pub(crate) status: ::std::option::Option<crate::types::StatisticalIssueStatus>,
    pub(crate) number_of_large_timestamp_gaps: ::std::option::Option<i32>,
    pub(crate) max_timestamp_gap_in_days: ::std::option::Option<i32>,
}
impl LargeTimestampGapsBuilder {
    /// <p> Indicates whether there is a potential data issue related to large gaps in timestamps. </p>
    pub fn status(mut self, input: crate::types::StatisticalIssueStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p> Indicates whether there is a potential data issue related to large gaps in timestamps. </p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::StatisticalIssueStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p> Indicates the number of large timestamp gaps, if there are any. </p>
    pub fn number_of_large_timestamp_gaps(mut self, input: i32) -> Self {
        self.number_of_large_timestamp_gaps = ::std::option::Option::Some(input);
        self
    }
    /// <p> Indicates the number of large timestamp gaps, if there are any. </p>
    pub fn set_number_of_large_timestamp_gaps(mut self, input: ::std::option::Option<i32>) -> Self {
        self.number_of_large_timestamp_gaps = input;
        self
    }
    /// <p> Indicates the size of the largest timestamp gap, in days. </p>
    pub fn max_timestamp_gap_in_days(mut self, input: i32) -> Self {
        self.max_timestamp_gap_in_days = ::std::option::Option::Some(input);
        self
    }
    /// <p> Indicates the size of the largest timestamp gap, in days. </p>
    pub fn set_max_timestamp_gap_in_days(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_timestamp_gap_in_days = input;
        self
    }
    /// Consumes the builder and constructs a [`LargeTimestampGaps`](crate::types::LargeTimestampGaps).
    pub fn build(self) -> crate::types::LargeTimestampGaps {
        crate::types::LargeTimestampGaps {
            status: self.status,
            number_of_large_timestamp_gaps: self.number_of_large_timestamp_gaps,
            max_timestamp_gap_in_days: self.max_timestamp_gap_in_days,
        }
    }
}
