// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The field series item configuration of a line chart.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FieldSeriesItem {
    /// <p>The field ID of the field for which you are setting the axis binding.</p>
    #[doc(hidden)]
    pub field_id: ::std::option::Option<::std::string::String>,
    /// <p>The axis that you are binding the field to.</p>
    #[doc(hidden)]
    pub axis_binding: ::std::option::Option<crate::types::AxisBinding>,
    /// <p>The options that determine the presentation of line series associated to the field.</p>
    #[doc(hidden)]
    pub settings: ::std::option::Option<crate::types::LineChartSeriesSettings>,
}
impl FieldSeriesItem {
    /// <p>The field ID of the field for which you are setting the axis binding.</p>
    pub fn field_id(&self) -> ::std::option::Option<&str> {
        self.field_id.as_deref()
    }
    /// <p>The axis that you are binding the field to.</p>
    pub fn axis_binding(&self) -> ::std::option::Option<&crate::types::AxisBinding> {
        self.axis_binding.as_ref()
    }
    /// <p>The options that determine the presentation of line series associated to the field.</p>
    pub fn settings(&self) -> ::std::option::Option<&crate::types::LineChartSeriesSettings> {
        self.settings.as_ref()
    }
}
impl FieldSeriesItem {
    /// Creates a new builder-style object to manufacture [`FieldSeriesItem`](crate::types::FieldSeriesItem).
    pub fn builder() -> crate::types::builders::FieldSeriesItemBuilder {
        crate::types::builders::FieldSeriesItemBuilder::default()
    }
}

/// A builder for [`FieldSeriesItem`](crate::types::FieldSeriesItem).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FieldSeriesItemBuilder {
    pub(crate) field_id: ::std::option::Option<::std::string::String>,
    pub(crate) axis_binding: ::std::option::Option<crate::types::AxisBinding>,
    pub(crate) settings: ::std::option::Option<crate::types::LineChartSeriesSettings>,
}
impl FieldSeriesItemBuilder {
    /// <p>The field ID of the field for which you are setting the axis binding.</p>
    pub fn field_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.field_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The field ID of the field for which you are setting the axis binding.</p>
    pub fn set_field_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.field_id = input;
        self
    }
    /// <p>The axis that you are binding the field to.</p>
    pub fn axis_binding(mut self, input: crate::types::AxisBinding) -> Self {
        self.axis_binding = ::std::option::Option::Some(input);
        self
    }
    /// <p>The axis that you are binding the field to.</p>
    pub fn set_axis_binding(
        mut self,
        input: ::std::option::Option<crate::types::AxisBinding>,
    ) -> Self {
        self.axis_binding = input;
        self
    }
    /// <p>The options that determine the presentation of line series associated to the field.</p>
    pub fn settings(mut self, input: crate::types::LineChartSeriesSettings) -> Self {
        self.settings = ::std::option::Option::Some(input);
        self
    }
    /// <p>The options that determine the presentation of line series associated to the field.</p>
    pub fn set_settings(
        mut self,
        input: ::std::option::Option<crate::types::LineChartSeriesSettings>,
    ) -> Self {
        self.settings = input;
        self
    }
    /// Consumes the builder and constructs a [`FieldSeriesItem`](crate::types::FieldSeriesItem).
    pub fn build(self) -> crate::types::FieldSeriesItem {
        crate::types::FieldSeriesItem {
            field_id: self.field_id,
            axis_binding: self.axis_binding,
            settings: self.settings,
        }
    }
}
