// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Used to filter for insights that have the status <code>CLOSED</code>. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListInsightsClosedStatusFilter {
    /// <p> Use to filter for either <code>REACTIVE</code> or <code>PROACTIVE</code> insights. </p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::InsightType>,
    /// <p> A time range used to specify when the behavior of the filtered insights ended. </p>
    #[doc(hidden)]
    pub end_time_range: ::std::option::Option<crate::types::EndTimeRange>,
}
impl ListInsightsClosedStatusFilter {
    /// <p> Use to filter for either <code>REACTIVE</code> or <code>PROACTIVE</code> insights. </p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::InsightType> {
        self.r#type.as_ref()
    }
    /// <p> A time range used to specify when the behavior of the filtered insights ended. </p>
    pub fn end_time_range(&self) -> ::std::option::Option<&crate::types::EndTimeRange> {
        self.end_time_range.as_ref()
    }
}
impl ListInsightsClosedStatusFilter {
    /// Creates a new builder-style object to manufacture [`ListInsightsClosedStatusFilter`](crate::types::ListInsightsClosedStatusFilter).
    pub fn builder() -> crate::types::builders::ListInsightsClosedStatusFilterBuilder {
        crate::types::builders::ListInsightsClosedStatusFilterBuilder::default()
    }
}

/// A builder for [`ListInsightsClosedStatusFilter`](crate::types::ListInsightsClosedStatusFilter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListInsightsClosedStatusFilterBuilder {
    pub(crate) r#type: ::std::option::Option<crate::types::InsightType>,
    pub(crate) end_time_range: ::std::option::Option<crate::types::EndTimeRange>,
}
impl ListInsightsClosedStatusFilterBuilder {
    /// <p> Use to filter for either <code>REACTIVE</code> or <code>PROACTIVE</code> insights. </p>
    pub fn r#type(mut self, input: crate::types::InsightType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p> Use to filter for either <code>REACTIVE</code> or <code>PROACTIVE</code> insights. </p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::InsightType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p> A time range used to specify when the behavior of the filtered insights ended. </p>
    pub fn end_time_range(mut self, input: crate::types::EndTimeRange) -> Self {
        self.end_time_range = ::std::option::Option::Some(input);
        self
    }
    /// <p> A time range used to specify when the behavior of the filtered insights ended. </p>
    pub fn set_end_time_range(
        mut self,
        input: ::std::option::Option<crate::types::EndTimeRange>,
    ) -> Self {
        self.end_time_range = input;
        self
    }
    /// Consumes the builder and constructs a [`ListInsightsClosedStatusFilter`](crate::types::ListInsightsClosedStatusFilter).
    pub fn build(self) -> crate::types::ListInsightsClosedStatusFilter {
        crate::types::ListInsightsClosedStatusFilter {
            r#type: self.r#type,
            end_time_range: self.end_time_range,
        }
    }
}
