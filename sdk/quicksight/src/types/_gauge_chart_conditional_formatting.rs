// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The conditional formatting of a <code>GaugeChartVisual</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GaugeChartConditionalFormatting {
    /// <p>Conditional formatting options of a <code>GaugeChartVisual</code>.</p>
    #[doc(hidden)]
    pub conditional_formatting_options:
        ::std::option::Option<::std::vec::Vec<crate::types::GaugeChartConditionalFormattingOption>>,
}
impl GaugeChartConditionalFormatting {
    /// <p>Conditional formatting options of a <code>GaugeChartVisual</code>.</p>
    pub fn conditional_formatting_options(
        &self,
    ) -> ::std::option::Option<&[crate::types::GaugeChartConditionalFormattingOption]> {
        self.conditional_formatting_options.as_deref()
    }
}
impl GaugeChartConditionalFormatting {
    /// Creates a new builder-style object to manufacture [`GaugeChartConditionalFormatting`](crate::types::GaugeChartConditionalFormatting).
    pub fn builder() -> crate::types::builders::GaugeChartConditionalFormattingBuilder {
        crate::types::builders::GaugeChartConditionalFormattingBuilder::default()
    }
}

/// A builder for [`GaugeChartConditionalFormatting`](crate::types::GaugeChartConditionalFormatting).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GaugeChartConditionalFormattingBuilder {
    pub(crate) conditional_formatting_options:
        ::std::option::Option<::std::vec::Vec<crate::types::GaugeChartConditionalFormattingOption>>,
}
impl GaugeChartConditionalFormattingBuilder {
    /// Appends an item to `conditional_formatting_options`.
    ///
    /// To override the contents of this collection use [`set_conditional_formatting_options`](Self::set_conditional_formatting_options).
    ///
    /// <p>Conditional formatting options of a <code>GaugeChartVisual</code>.</p>
    pub fn conditional_formatting_options(
        mut self,
        input: crate::types::GaugeChartConditionalFormattingOption,
    ) -> Self {
        let mut v = self.conditional_formatting_options.unwrap_or_default();
        v.push(input);
        self.conditional_formatting_options = ::std::option::Option::Some(v);
        self
    }
    /// <p>Conditional formatting options of a <code>GaugeChartVisual</code>.</p>
    pub fn set_conditional_formatting_options(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::GaugeChartConditionalFormattingOption>,
        >,
    ) -> Self {
        self.conditional_formatting_options = input;
        self
    }
    /// Consumes the builder and constructs a [`GaugeChartConditionalFormatting`](crate::types::GaugeChartConditionalFormatting).
    pub fn build(self) -> crate::types::GaugeChartConditionalFormatting {
        crate::types::GaugeChartConditionalFormatting {
            conditional_formatting_options: self.conditional_formatting_options,
        }
    }
}
