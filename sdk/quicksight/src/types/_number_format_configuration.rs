// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Formatting configuration for number fields.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NumberFormatConfiguration {
    /// <p>The options that determine the numeric format configuration.</p>
    #[doc(hidden)]
    pub format_configuration: ::std::option::Option<crate::types::NumericFormatConfiguration>,
}
impl NumberFormatConfiguration {
    /// <p>The options that determine the numeric format configuration.</p>
    pub fn format_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::NumericFormatConfiguration> {
        self.format_configuration.as_ref()
    }
}
impl NumberFormatConfiguration {
    /// Creates a new builder-style object to manufacture [`NumberFormatConfiguration`](crate::types::NumberFormatConfiguration).
    pub fn builder() -> crate::types::builders::NumberFormatConfigurationBuilder {
        crate::types::builders::NumberFormatConfigurationBuilder::default()
    }
}

/// A builder for [`NumberFormatConfiguration`](crate::types::NumberFormatConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct NumberFormatConfigurationBuilder {
    pub(crate) format_configuration:
        ::std::option::Option<crate::types::NumericFormatConfiguration>,
}
impl NumberFormatConfigurationBuilder {
    /// <p>The options that determine the numeric format configuration.</p>
    pub fn format_configuration(mut self, input: crate::types::NumericFormatConfiguration) -> Self {
        self.format_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The options that determine the numeric format configuration.</p>
    pub fn set_format_configuration(
        mut self,
        input: ::std::option::Option<crate::types::NumericFormatConfiguration>,
    ) -> Self {
        self.format_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`NumberFormatConfiguration`](crate::types::NumberFormatConfiguration).
    pub fn build(self) -> crate::types::NumberFormatConfiguration {
        crate::types::NumberFormatConfiguration {
            format_configuration: self.format_configuration,
        }
    }
}
