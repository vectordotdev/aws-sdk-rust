// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The total options for a table visual.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TotalOptions {
    /// <p>The visibility configuration for the total cells.</p>
    #[doc(hidden)]
    pub totals_visibility: ::std::option::Option<crate::types::Visibility>,
    /// <p>The placement (start, end) for the total cells.</p>
    #[doc(hidden)]
    pub placement: ::std::option::Option<crate::types::TableTotalsPlacement>,
    /// <p>The scroll status (pinned, scrolled) for the total cells.</p>
    #[doc(hidden)]
    pub scroll_status: ::std::option::Option<crate::types::TableTotalsScrollStatus>,
    /// <p>The custom label string for the total cells.</p>
    #[doc(hidden)]
    pub custom_label: ::std::option::Option<::std::string::String>,
    /// <p>Cell styling options for the total cells.</p>
    #[doc(hidden)]
    pub total_cell_style: ::std::option::Option<crate::types::TableCellStyle>,
}
impl TotalOptions {
    /// <p>The visibility configuration for the total cells.</p>
    pub fn totals_visibility(&self) -> ::std::option::Option<&crate::types::Visibility> {
        self.totals_visibility.as_ref()
    }
    /// <p>The placement (start, end) for the total cells.</p>
    pub fn placement(&self) -> ::std::option::Option<&crate::types::TableTotalsPlacement> {
        self.placement.as_ref()
    }
    /// <p>The scroll status (pinned, scrolled) for the total cells.</p>
    pub fn scroll_status(&self) -> ::std::option::Option<&crate::types::TableTotalsScrollStatus> {
        self.scroll_status.as_ref()
    }
    /// <p>The custom label string for the total cells.</p>
    pub fn custom_label(&self) -> ::std::option::Option<&str> {
        self.custom_label.as_deref()
    }
    /// <p>Cell styling options for the total cells.</p>
    pub fn total_cell_style(&self) -> ::std::option::Option<&crate::types::TableCellStyle> {
        self.total_cell_style.as_ref()
    }
}
impl TotalOptions {
    /// Creates a new builder-style object to manufacture [`TotalOptions`](crate::types::TotalOptions).
    pub fn builder() -> crate::types::builders::TotalOptionsBuilder {
        crate::types::builders::TotalOptionsBuilder::default()
    }
}

/// A builder for [`TotalOptions`](crate::types::TotalOptions).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TotalOptionsBuilder {
    pub(crate) totals_visibility: ::std::option::Option<crate::types::Visibility>,
    pub(crate) placement: ::std::option::Option<crate::types::TableTotalsPlacement>,
    pub(crate) scroll_status: ::std::option::Option<crate::types::TableTotalsScrollStatus>,
    pub(crate) custom_label: ::std::option::Option<::std::string::String>,
    pub(crate) total_cell_style: ::std::option::Option<crate::types::TableCellStyle>,
}
impl TotalOptionsBuilder {
    /// <p>The visibility configuration for the total cells.</p>
    pub fn totals_visibility(mut self, input: crate::types::Visibility) -> Self {
        self.totals_visibility = ::std::option::Option::Some(input);
        self
    }
    /// <p>The visibility configuration for the total cells.</p>
    pub fn set_totals_visibility(
        mut self,
        input: ::std::option::Option<crate::types::Visibility>,
    ) -> Self {
        self.totals_visibility = input;
        self
    }
    /// <p>The placement (start, end) for the total cells.</p>
    pub fn placement(mut self, input: crate::types::TableTotalsPlacement) -> Self {
        self.placement = ::std::option::Option::Some(input);
        self
    }
    /// <p>The placement (start, end) for the total cells.</p>
    pub fn set_placement(
        mut self,
        input: ::std::option::Option<crate::types::TableTotalsPlacement>,
    ) -> Self {
        self.placement = input;
        self
    }
    /// <p>The scroll status (pinned, scrolled) for the total cells.</p>
    pub fn scroll_status(mut self, input: crate::types::TableTotalsScrollStatus) -> Self {
        self.scroll_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The scroll status (pinned, scrolled) for the total cells.</p>
    pub fn set_scroll_status(
        mut self,
        input: ::std::option::Option<crate::types::TableTotalsScrollStatus>,
    ) -> Self {
        self.scroll_status = input;
        self
    }
    /// <p>The custom label string for the total cells.</p>
    pub fn custom_label(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.custom_label = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The custom label string for the total cells.</p>
    pub fn set_custom_label(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.custom_label = input;
        self
    }
    /// <p>Cell styling options for the total cells.</p>
    pub fn total_cell_style(mut self, input: crate::types::TableCellStyle) -> Self {
        self.total_cell_style = ::std::option::Option::Some(input);
        self
    }
    /// <p>Cell styling options for the total cells.</p>
    pub fn set_total_cell_style(
        mut self,
        input: ::std::option::Option<crate::types::TableCellStyle>,
    ) -> Self {
        self.total_cell_style = input;
        self
    }
    /// Consumes the builder and constructs a [`TotalOptions`](crate::types::TotalOptions).
    pub fn build(self) -> crate::types::TotalOptions {
        crate::types::TotalOptions {
            totals_visibility: self.totals_visibility,
            placement: self.placement,
            scroll_status: self.scroll_status,
            custom_label: self.custom_label,
            total_cell_style: self.total_cell_style,
        }
    }
}
