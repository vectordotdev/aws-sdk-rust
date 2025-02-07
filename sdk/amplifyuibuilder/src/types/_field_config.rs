// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the configuration information for a field in a table.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FieldConfig {
    /// <p>The label for the field.</p>
    #[doc(hidden)]
    pub label: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the field position.</p>
    #[doc(hidden)]
    pub position: ::std::option::Option<crate::types::FieldPosition>,
    /// <p>Specifies whether to hide a field.</p>
    #[doc(hidden)]
    pub excluded: ::std::option::Option<bool>,
    /// <p>Describes the configuration for the default input value to display for a field.</p>
    #[doc(hidden)]
    pub input_type: ::std::option::Option<crate::types::FieldInputConfig>,
    /// <p>The validations to perform on the value in the field.</p>
    #[doc(hidden)]
    pub validations:
        ::std::option::Option<::std::vec::Vec<crate::types::FieldValidationConfiguration>>,
}
impl FieldConfig {
    /// <p>The label for the field.</p>
    pub fn label(&self) -> ::std::option::Option<&str> {
        self.label.as_deref()
    }
    /// <p>Specifies the field position.</p>
    pub fn position(&self) -> ::std::option::Option<&crate::types::FieldPosition> {
        self.position.as_ref()
    }
    /// <p>Specifies whether to hide a field.</p>
    pub fn excluded(&self) -> ::std::option::Option<bool> {
        self.excluded
    }
    /// <p>Describes the configuration for the default input value to display for a field.</p>
    pub fn input_type(&self) -> ::std::option::Option<&crate::types::FieldInputConfig> {
        self.input_type.as_ref()
    }
    /// <p>The validations to perform on the value in the field.</p>
    pub fn validations(
        &self,
    ) -> ::std::option::Option<&[crate::types::FieldValidationConfiguration]> {
        self.validations.as_deref()
    }
}
impl FieldConfig {
    /// Creates a new builder-style object to manufacture [`FieldConfig`](crate::types::FieldConfig).
    pub fn builder() -> crate::types::builders::FieldConfigBuilder {
        crate::types::builders::FieldConfigBuilder::default()
    }
}

/// A builder for [`FieldConfig`](crate::types::FieldConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FieldConfigBuilder {
    pub(crate) label: ::std::option::Option<::std::string::String>,
    pub(crate) position: ::std::option::Option<crate::types::FieldPosition>,
    pub(crate) excluded: ::std::option::Option<bool>,
    pub(crate) input_type: ::std::option::Option<crate::types::FieldInputConfig>,
    pub(crate) validations:
        ::std::option::Option<::std::vec::Vec<crate::types::FieldValidationConfiguration>>,
}
impl FieldConfigBuilder {
    /// <p>The label for the field.</p>
    pub fn label(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.label = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The label for the field.</p>
    pub fn set_label(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.label = input;
        self
    }
    /// <p>Specifies the field position.</p>
    pub fn position(mut self, input: crate::types::FieldPosition) -> Self {
        self.position = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the field position.</p>
    pub fn set_position(
        mut self,
        input: ::std::option::Option<crate::types::FieldPosition>,
    ) -> Self {
        self.position = input;
        self
    }
    /// <p>Specifies whether to hide a field.</p>
    pub fn excluded(mut self, input: bool) -> Self {
        self.excluded = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether to hide a field.</p>
    pub fn set_excluded(mut self, input: ::std::option::Option<bool>) -> Self {
        self.excluded = input;
        self
    }
    /// <p>Describes the configuration for the default input value to display for a field.</p>
    pub fn input_type(mut self, input: crate::types::FieldInputConfig) -> Self {
        self.input_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the configuration for the default input value to display for a field.</p>
    pub fn set_input_type(
        mut self,
        input: ::std::option::Option<crate::types::FieldInputConfig>,
    ) -> Self {
        self.input_type = input;
        self
    }
    /// Appends an item to `validations`.
    ///
    /// To override the contents of this collection use [`set_validations`](Self::set_validations).
    ///
    /// <p>The validations to perform on the value in the field.</p>
    pub fn validations(mut self, input: crate::types::FieldValidationConfiguration) -> Self {
        let mut v = self.validations.unwrap_or_default();
        v.push(input);
        self.validations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The validations to perform on the value in the field.</p>
    pub fn set_validations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FieldValidationConfiguration>>,
    ) -> Self {
        self.validations = input;
        self
    }
    /// Consumes the builder and constructs a [`FieldConfig`](crate::types::FieldConfig).
    pub fn build(self) -> crate::types::FieldConfig {
        crate::types::FieldConfig {
            label: self.label,
            position: self.position,
            excluded: self.excluded,
            input_type: self.input_type,
            validations: self.validations,
        }
    }
}
