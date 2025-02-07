// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A parameter declaration for the <code>DateTime</code> data type.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DateTimeParameterDeclaration {
    /// <p>The name of the parameter that is being declared.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The default values of a parameter. If the parameter is a single-value parameter, a maximum of one default value can be provided.</p>
    #[doc(hidden)]
    pub default_values: ::std::option::Option<crate::types::DateTimeDefaultValues>,
    /// <p>The level of time precision that is used to aggregate <code>DateTime</code> values.</p>
    #[doc(hidden)]
    pub time_granularity: ::std::option::Option<crate::types::TimeGranularity>,
    /// <p>The configuration that defines the default value of a <code>DateTime</code> parameter when a value has not been set.</p>
    #[doc(hidden)]
    pub value_when_unset: ::std::option::Option<crate::types::DateTimeValueWhenUnsetConfiguration>,
    /// <p>A list of dataset parameters that are mapped to an analysis parameter.</p>
    #[doc(hidden)]
    pub mapped_data_set_parameters:
        ::std::option::Option<::std::vec::Vec<crate::types::MappedDataSetParameter>>,
}
impl DateTimeParameterDeclaration {
    /// <p>The name of the parameter that is being declared.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The default values of a parameter. If the parameter is a single-value parameter, a maximum of one default value can be provided.</p>
    pub fn default_values(&self) -> ::std::option::Option<&crate::types::DateTimeDefaultValues> {
        self.default_values.as_ref()
    }
    /// <p>The level of time precision that is used to aggregate <code>DateTime</code> values.</p>
    pub fn time_granularity(&self) -> ::std::option::Option<&crate::types::TimeGranularity> {
        self.time_granularity.as_ref()
    }
    /// <p>The configuration that defines the default value of a <code>DateTime</code> parameter when a value has not been set.</p>
    pub fn value_when_unset(
        &self,
    ) -> ::std::option::Option<&crate::types::DateTimeValueWhenUnsetConfiguration> {
        self.value_when_unset.as_ref()
    }
    /// <p>A list of dataset parameters that are mapped to an analysis parameter.</p>
    pub fn mapped_data_set_parameters(
        &self,
    ) -> ::std::option::Option<&[crate::types::MappedDataSetParameter]> {
        self.mapped_data_set_parameters.as_deref()
    }
}
impl DateTimeParameterDeclaration {
    /// Creates a new builder-style object to manufacture [`DateTimeParameterDeclaration`](crate::types::DateTimeParameterDeclaration).
    pub fn builder() -> crate::types::builders::DateTimeParameterDeclarationBuilder {
        crate::types::builders::DateTimeParameterDeclarationBuilder::default()
    }
}

/// A builder for [`DateTimeParameterDeclaration`](crate::types::DateTimeParameterDeclaration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DateTimeParameterDeclarationBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) default_values: ::std::option::Option<crate::types::DateTimeDefaultValues>,
    pub(crate) time_granularity: ::std::option::Option<crate::types::TimeGranularity>,
    pub(crate) value_when_unset:
        ::std::option::Option<crate::types::DateTimeValueWhenUnsetConfiguration>,
    pub(crate) mapped_data_set_parameters:
        ::std::option::Option<::std::vec::Vec<crate::types::MappedDataSetParameter>>,
}
impl DateTimeParameterDeclarationBuilder {
    /// <p>The name of the parameter that is being declared.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the parameter that is being declared.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The default values of a parameter. If the parameter is a single-value parameter, a maximum of one default value can be provided.</p>
    pub fn default_values(mut self, input: crate::types::DateTimeDefaultValues) -> Self {
        self.default_values = ::std::option::Option::Some(input);
        self
    }
    /// <p>The default values of a parameter. If the parameter is a single-value parameter, a maximum of one default value can be provided.</p>
    pub fn set_default_values(
        mut self,
        input: ::std::option::Option<crate::types::DateTimeDefaultValues>,
    ) -> Self {
        self.default_values = input;
        self
    }
    /// <p>The level of time precision that is used to aggregate <code>DateTime</code> values.</p>
    pub fn time_granularity(mut self, input: crate::types::TimeGranularity) -> Self {
        self.time_granularity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The level of time precision that is used to aggregate <code>DateTime</code> values.</p>
    pub fn set_time_granularity(
        mut self,
        input: ::std::option::Option<crate::types::TimeGranularity>,
    ) -> Self {
        self.time_granularity = input;
        self
    }
    /// <p>The configuration that defines the default value of a <code>DateTime</code> parameter when a value has not been set.</p>
    pub fn value_when_unset(
        mut self,
        input: crate::types::DateTimeValueWhenUnsetConfiguration,
    ) -> Self {
        self.value_when_unset = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration that defines the default value of a <code>DateTime</code> parameter when a value has not been set.</p>
    pub fn set_value_when_unset(
        mut self,
        input: ::std::option::Option<crate::types::DateTimeValueWhenUnsetConfiguration>,
    ) -> Self {
        self.value_when_unset = input;
        self
    }
    /// Appends an item to `mapped_data_set_parameters`.
    ///
    /// To override the contents of this collection use [`set_mapped_data_set_parameters`](Self::set_mapped_data_set_parameters).
    ///
    /// <p>A list of dataset parameters that are mapped to an analysis parameter.</p>
    pub fn mapped_data_set_parameters(
        mut self,
        input: crate::types::MappedDataSetParameter,
    ) -> Self {
        let mut v = self.mapped_data_set_parameters.unwrap_or_default();
        v.push(input);
        self.mapped_data_set_parameters = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of dataset parameters that are mapped to an analysis parameter.</p>
    pub fn set_mapped_data_set_parameters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MappedDataSetParameter>>,
    ) -> Self {
        self.mapped_data_set_parameters = input;
        self
    }
    /// Consumes the builder and constructs a [`DateTimeParameterDeclaration`](crate::types::DateTimeParameterDeclaration).
    pub fn build(self) -> crate::types::DateTimeParameterDeclaration {
        crate::types::DateTimeParameterDeclaration {
            name: self.name,
            default_values: self.default_values,
            time_granularity: self.time_granularity,
            value_when_unset: self.value_when_unset,
            mapped_data_set_parameters: self.mapped_data_set_parameters,
        }
    }
}
