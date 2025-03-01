// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The aggregated field well for a box plot.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BoxPlotAggregatedFieldWells {
    /// <p>The group by field well of a box plot chart. Values are grouped based on group by fields.</p>
    #[doc(hidden)]
    pub group_by: ::std::option::Option<::std::vec::Vec<crate::types::DimensionField>>,
    /// <p>The value field well of a box plot chart. Values are aggregated based on group by fields.</p>
    #[doc(hidden)]
    pub values: ::std::option::Option<::std::vec::Vec<crate::types::MeasureField>>,
}
impl BoxPlotAggregatedFieldWells {
    /// <p>The group by field well of a box plot chart. Values are grouped based on group by fields.</p>
    pub fn group_by(&self) -> ::std::option::Option<&[crate::types::DimensionField]> {
        self.group_by.as_deref()
    }
    /// <p>The value field well of a box plot chart. Values are aggregated based on group by fields.</p>
    pub fn values(&self) -> ::std::option::Option<&[crate::types::MeasureField]> {
        self.values.as_deref()
    }
}
impl BoxPlotAggregatedFieldWells {
    /// Creates a new builder-style object to manufacture [`BoxPlotAggregatedFieldWells`](crate::types::BoxPlotAggregatedFieldWells).
    pub fn builder() -> crate::types::builders::BoxPlotAggregatedFieldWellsBuilder {
        crate::types::builders::BoxPlotAggregatedFieldWellsBuilder::default()
    }
}

/// A builder for [`BoxPlotAggregatedFieldWells`](crate::types::BoxPlotAggregatedFieldWells).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BoxPlotAggregatedFieldWellsBuilder {
    pub(crate) group_by: ::std::option::Option<::std::vec::Vec<crate::types::DimensionField>>,
    pub(crate) values: ::std::option::Option<::std::vec::Vec<crate::types::MeasureField>>,
}
impl BoxPlotAggregatedFieldWellsBuilder {
    /// Appends an item to `group_by`.
    ///
    /// To override the contents of this collection use [`set_group_by`](Self::set_group_by).
    ///
    /// <p>The group by field well of a box plot chart. Values are grouped based on group by fields.</p>
    pub fn group_by(mut self, input: crate::types::DimensionField) -> Self {
        let mut v = self.group_by.unwrap_or_default();
        v.push(input);
        self.group_by = ::std::option::Option::Some(v);
        self
    }
    /// <p>The group by field well of a box plot chart. Values are grouped based on group by fields.</p>
    pub fn set_group_by(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DimensionField>>,
    ) -> Self {
        self.group_by = input;
        self
    }
    /// Appends an item to `values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p>The value field well of a box plot chart. Values are aggregated based on group by fields.</p>
    pub fn values(mut self, input: crate::types::MeasureField) -> Self {
        let mut v = self.values.unwrap_or_default();
        v.push(input);
        self.values = ::std::option::Option::Some(v);
        self
    }
    /// <p>The value field well of a box plot chart. Values are aggregated based on group by fields.</p>
    pub fn set_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MeasureField>>,
    ) -> Self {
        self.values = input;
        self
    }
    /// Consumes the builder and constructs a [`BoxPlotAggregatedFieldWells`](crate::types::BoxPlotAggregatedFieldWells).
    pub fn build(self) -> crate::types::BoxPlotAggregatedFieldWells {
        crate::types::BoxPlotAggregatedFieldWells {
            group_by: self.group_by,
            values: self.values,
        }
    }
}
