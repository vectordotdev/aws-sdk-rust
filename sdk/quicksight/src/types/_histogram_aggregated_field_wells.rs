// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The field well configuration of a histogram.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct HistogramAggregatedFieldWells {
    /// <p>The value field wells of a histogram. Values are aggregated by <code>COUNT</code> or <code>DISTINCT_COUNT</code>.</p>
    #[doc(hidden)]
    pub values: ::std::option::Option<::std::vec::Vec<crate::types::MeasureField>>,
}
impl HistogramAggregatedFieldWells {
    /// <p>The value field wells of a histogram. Values are aggregated by <code>COUNT</code> or <code>DISTINCT_COUNT</code>.</p>
    pub fn values(&self) -> ::std::option::Option<&[crate::types::MeasureField]> {
        self.values.as_deref()
    }
}
impl HistogramAggregatedFieldWells {
    /// Creates a new builder-style object to manufacture [`HistogramAggregatedFieldWells`](crate::types::HistogramAggregatedFieldWells).
    pub fn builder() -> crate::types::builders::HistogramAggregatedFieldWellsBuilder {
        crate::types::builders::HistogramAggregatedFieldWellsBuilder::default()
    }
}

/// A builder for [`HistogramAggregatedFieldWells`](crate::types::HistogramAggregatedFieldWells).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct HistogramAggregatedFieldWellsBuilder {
    pub(crate) values: ::std::option::Option<::std::vec::Vec<crate::types::MeasureField>>,
}
impl HistogramAggregatedFieldWellsBuilder {
    /// Appends an item to `values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p>The value field wells of a histogram. Values are aggregated by <code>COUNT</code> or <code>DISTINCT_COUNT</code>.</p>
    pub fn values(mut self, input: crate::types::MeasureField) -> Self {
        let mut v = self.values.unwrap_or_default();
        v.push(input);
        self.values = ::std::option::Option::Some(v);
        self
    }
    /// <p>The value field wells of a histogram. Values are aggregated by <code>COUNT</code> or <code>DISTINCT_COUNT</code>.</p>
    pub fn set_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MeasureField>>,
    ) -> Self {
        self.values = input;
        self
    }
    /// Consumes the builder and constructs a [`HistogramAggregatedFieldWells`](crate::types::HistogramAggregatedFieldWells).
    pub fn build(self) -> crate::types::HistogramAggregatedFieldWells {
        crate::types::HistogramAggregatedFieldWells {
            values: self.values,
        }
    }
}
