// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The table options for a pivot table visual.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PivotTableOptions {
    /// <p>The metric placement (row, column) options.</p>
    #[doc(hidden)]
    pub metric_placement: ::std::option::Option<crate::types::PivotTableMetricPlacement>,
    /// <p>The visibility of the single metric options.</p>
    #[doc(hidden)]
    pub single_metric_visibility: ::std::option::Option<crate::types::Visibility>,
    /// <p>The visibility of the column names.</p>
    #[doc(hidden)]
    pub column_names_visibility: ::std::option::Option<crate::types::Visibility>,
    /// <p>Determines the visibility of the pivot table.</p>
    #[doc(hidden)]
    pub toggle_buttons_visibility: ::std::option::Option<crate::types::Visibility>,
    /// <p>The table cell style of the column header.</p>
    #[doc(hidden)]
    pub column_header_style: ::std::option::Option<crate::types::TableCellStyle>,
    /// <p>The table cell style of the row headers.</p>
    #[doc(hidden)]
    pub row_header_style: ::std::option::Option<crate::types::TableCellStyle>,
    /// <p>The table cell style of cells.</p>
    #[doc(hidden)]
    pub cell_style: ::std::option::Option<crate::types::TableCellStyle>,
    /// <p>The table cell style of row field names.</p>
    #[doc(hidden)]
    pub row_field_names_style: ::std::option::Option<crate::types::TableCellStyle>,
    /// <p>The row alternate color options (widget status, row alternate colors).</p>
    #[doc(hidden)]
    pub row_alternate_color_options: ::std::option::Option<crate::types::RowAlternateColorOptions>,
    /// <p>The visibility setting of a pivot table's collapsed row dimension fields. If the value of this structure is <code>HIDDEN</code>, all collapsed columns in a pivot table are automatically hidden. The default value is <code>VISIBLE</code>.</p>
    #[doc(hidden)]
    pub collapsed_row_dimensions_visibility: ::std::option::Option<crate::types::Visibility>,
}
impl PivotTableOptions {
    /// <p>The metric placement (row, column) options.</p>
    pub fn metric_placement(
        &self,
    ) -> ::std::option::Option<&crate::types::PivotTableMetricPlacement> {
        self.metric_placement.as_ref()
    }
    /// <p>The visibility of the single metric options.</p>
    pub fn single_metric_visibility(&self) -> ::std::option::Option<&crate::types::Visibility> {
        self.single_metric_visibility.as_ref()
    }
    /// <p>The visibility of the column names.</p>
    pub fn column_names_visibility(&self) -> ::std::option::Option<&crate::types::Visibility> {
        self.column_names_visibility.as_ref()
    }
    /// <p>Determines the visibility of the pivot table.</p>
    pub fn toggle_buttons_visibility(&self) -> ::std::option::Option<&crate::types::Visibility> {
        self.toggle_buttons_visibility.as_ref()
    }
    /// <p>The table cell style of the column header.</p>
    pub fn column_header_style(&self) -> ::std::option::Option<&crate::types::TableCellStyle> {
        self.column_header_style.as_ref()
    }
    /// <p>The table cell style of the row headers.</p>
    pub fn row_header_style(&self) -> ::std::option::Option<&crate::types::TableCellStyle> {
        self.row_header_style.as_ref()
    }
    /// <p>The table cell style of cells.</p>
    pub fn cell_style(&self) -> ::std::option::Option<&crate::types::TableCellStyle> {
        self.cell_style.as_ref()
    }
    /// <p>The table cell style of row field names.</p>
    pub fn row_field_names_style(&self) -> ::std::option::Option<&crate::types::TableCellStyle> {
        self.row_field_names_style.as_ref()
    }
    /// <p>The row alternate color options (widget status, row alternate colors).</p>
    pub fn row_alternate_color_options(
        &self,
    ) -> ::std::option::Option<&crate::types::RowAlternateColorOptions> {
        self.row_alternate_color_options.as_ref()
    }
    /// <p>The visibility setting of a pivot table's collapsed row dimension fields. If the value of this structure is <code>HIDDEN</code>, all collapsed columns in a pivot table are automatically hidden. The default value is <code>VISIBLE</code>.</p>
    pub fn collapsed_row_dimensions_visibility(
        &self,
    ) -> ::std::option::Option<&crate::types::Visibility> {
        self.collapsed_row_dimensions_visibility.as_ref()
    }
}
impl PivotTableOptions {
    /// Creates a new builder-style object to manufacture [`PivotTableOptions`](crate::types::PivotTableOptions).
    pub fn builder() -> crate::types::builders::PivotTableOptionsBuilder {
        crate::types::builders::PivotTableOptionsBuilder::default()
    }
}

/// A builder for [`PivotTableOptions`](crate::types::PivotTableOptions).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PivotTableOptionsBuilder {
    pub(crate) metric_placement: ::std::option::Option<crate::types::PivotTableMetricPlacement>,
    pub(crate) single_metric_visibility: ::std::option::Option<crate::types::Visibility>,
    pub(crate) column_names_visibility: ::std::option::Option<crate::types::Visibility>,
    pub(crate) toggle_buttons_visibility: ::std::option::Option<crate::types::Visibility>,
    pub(crate) column_header_style: ::std::option::Option<crate::types::TableCellStyle>,
    pub(crate) row_header_style: ::std::option::Option<crate::types::TableCellStyle>,
    pub(crate) cell_style: ::std::option::Option<crate::types::TableCellStyle>,
    pub(crate) row_field_names_style: ::std::option::Option<crate::types::TableCellStyle>,
    pub(crate) row_alternate_color_options:
        ::std::option::Option<crate::types::RowAlternateColorOptions>,
    pub(crate) collapsed_row_dimensions_visibility: ::std::option::Option<crate::types::Visibility>,
}
impl PivotTableOptionsBuilder {
    /// <p>The metric placement (row, column) options.</p>
    pub fn metric_placement(mut self, input: crate::types::PivotTableMetricPlacement) -> Self {
        self.metric_placement = ::std::option::Option::Some(input);
        self
    }
    /// <p>The metric placement (row, column) options.</p>
    pub fn set_metric_placement(
        mut self,
        input: ::std::option::Option<crate::types::PivotTableMetricPlacement>,
    ) -> Self {
        self.metric_placement = input;
        self
    }
    /// <p>The visibility of the single metric options.</p>
    pub fn single_metric_visibility(mut self, input: crate::types::Visibility) -> Self {
        self.single_metric_visibility = ::std::option::Option::Some(input);
        self
    }
    /// <p>The visibility of the single metric options.</p>
    pub fn set_single_metric_visibility(
        mut self,
        input: ::std::option::Option<crate::types::Visibility>,
    ) -> Self {
        self.single_metric_visibility = input;
        self
    }
    /// <p>The visibility of the column names.</p>
    pub fn column_names_visibility(mut self, input: crate::types::Visibility) -> Self {
        self.column_names_visibility = ::std::option::Option::Some(input);
        self
    }
    /// <p>The visibility of the column names.</p>
    pub fn set_column_names_visibility(
        mut self,
        input: ::std::option::Option<crate::types::Visibility>,
    ) -> Self {
        self.column_names_visibility = input;
        self
    }
    /// <p>Determines the visibility of the pivot table.</p>
    pub fn toggle_buttons_visibility(mut self, input: crate::types::Visibility) -> Self {
        self.toggle_buttons_visibility = ::std::option::Option::Some(input);
        self
    }
    /// <p>Determines the visibility of the pivot table.</p>
    pub fn set_toggle_buttons_visibility(
        mut self,
        input: ::std::option::Option<crate::types::Visibility>,
    ) -> Self {
        self.toggle_buttons_visibility = input;
        self
    }
    /// <p>The table cell style of the column header.</p>
    pub fn column_header_style(mut self, input: crate::types::TableCellStyle) -> Self {
        self.column_header_style = ::std::option::Option::Some(input);
        self
    }
    /// <p>The table cell style of the column header.</p>
    pub fn set_column_header_style(
        mut self,
        input: ::std::option::Option<crate::types::TableCellStyle>,
    ) -> Self {
        self.column_header_style = input;
        self
    }
    /// <p>The table cell style of the row headers.</p>
    pub fn row_header_style(mut self, input: crate::types::TableCellStyle) -> Self {
        self.row_header_style = ::std::option::Option::Some(input);
        self
    }
    /// <p>The table cell style of the row headers.</p>
    pub fn set_row_header_style(
        mut self,
        input: ::std::option::Option<crate::types::TableCellStyle>,
    ) -> Self {
        self.row_header_style = input;
        self
    }
    /// <p>The table cell style of cells.</p>
    pub fn cell_style(mut self, input: crate::types::TableCellStyle) -> Self {
        self.cell_style = ::std::option::Option::Some(input);
        self
    }
    /// <p>The table cell style of cells.</p>
    pub fn set_cell_style(
        mut self,
        input: ::std::option::Option<crate::types::TableCellStyle>,
    ) -> Self {
        self.cell_style = input;
        self
    }
    /// <p>The table cell style of row field names.</p>
    pub fn row_field_names_style(mut self, input: crate::types::TableCellStyle) -> Self {
        self.row_field_names_style = ::std::option::Option::Some(input);
        self
    }
    /// <p>The table cell style of row field names.</p>
    pub fn set_row_field_names_style(
        mut self,
        input: ::std::option::Option<crate::types::TableCellStyle>,
    ) -> Self {
        self.row_field_names_style = input;
        self
    }
    /// <p>The row alternate color options (widget status, row alternate colors).</p>
    pub fn row_alternate_color_options(
        mut self,
        input: crate::types::RowAlternateColorOptions,
    ) -> Self {
        self.row_alternate_color_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The row alternate color options (widget status, row alternate colors).</p>
    pub fn set_row_alternate_color_options(
        mut self,
        input: ::std::option::Option<crate::types::RowAlternateColorOptions>,
    ) -> Self {
        self.row_alternate_color_options = input;
        self
    }
    /// <p>The visibility setting of a pivot table's collapsed row dimension fields. If the value of this structure is <code>HIDDEN</code>, all collapsed columns in a pivot table are automatically hidden. The default value is <code>VISIBLE</code>.</p>
    pub fn collapsed_row_dimensions_visibility(mut self, input: crate::types::Visibility) -> Self {
        self.collapsed_row_dimensions_visibility = ::std::option::Option::Some(input);
        self
    }
    /// <p>The visibility setting of a pivot table's collapsed row dimension fields. If the value of this structure is <code>HIDDEN</code>, all collapsed columns in a pivot table are automatically hidden. The default value is <code>VISIBLE</code>.</p>
    pub fn set_collapsed_row_dimensions_visibility(
        mut self,
        input: ::std::option::Option<crate::types::Visibility>,
    ) -> Self {
        self.collapsed_row_dimensions_visibility = input;
        self
    }
    /// Consumes the builder and constructs a [`PivotTableOptions`](crate::types::PivotTableOptions).
    pub fn build(self) -> crate::types::PivotTableOptions {
        crate::types::PivotTableOptions {
            metric_placement: self.metric_placement,
            single_metric_visibility: self.single_metric_visibility,
            column_names_visibility: self.column_names_visibility,
            toggle_buttons_visibility: self.toggle_buttons_visibility,
            column_header_style: self.column_header_style,
            row_header_style: self.row_header_style,
            cell_style: self.cell_style,
            row_field_names_style: self.row_field_names_style,
            row_alternate_color_options: self.row_alternate_color_options,
            collapsed_row_dimensions_visibility: self.collapsed_row_dimensions_visibility,
        }
    }
}
