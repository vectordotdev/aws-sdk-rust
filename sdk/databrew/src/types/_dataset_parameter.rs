// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a dataset parameter that defines type and conditions for a parameter in the Amazon S3 path of the dataset.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DatasetParameter {
    /// <p>The name of the parameter that is used in the dataset's Amazon S3 path.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The type of the dataset parameter, can be one of a 'String', 'Number' or 'Datetime'.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::ParameterType>,
    /// <p>Additional parameter options such as a format and a timezone. Required for datetime parameters.</p>
    #[doc(hidden)]
    pub datetime_options: ::std::option::Option<crate::types::DatetimeOptions>,
    /// <p>Optional boolean value that defines whether the captured value of this parameter should be used to create a new column in a dataset.</p>
    #[doc(hidden)]
    pub create_column: bool,
    /// <p>The optional filter expression structure to apply additional matching criteria to the parameter.</p>
    #[doc(hidden)]
    pub filter: ::std::option::Option<crate::types::FilterExpression>,
}
impl DatasetParameter {
    /// <p>The name of the parameter that is used in the dataset's Amazon S3 path.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The type of the dataset parameter, can be one of a 'String', 'Number' or 'Datetime'.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::ParameterType> {
        self.r#type.as_ref()
    }
    /// <p>Additional parameter options such as a format and a timezone. Required for datetime parameters.</p>
    pub fn datetime_options(&self) -> ::std::option::Option<&crate::types::DatetimeOptions> {
        self.datetime_options.as_ref()
    }
    /// <p>Optional boolean value that defines whether the captured value of this parameter should be used to create a new column in a dataset.</p>
    pub fn create_column(&self) -> bool {
        self.create_column
    }
    /// <p>The optional filter expression structure to apply additional matching criteria to the parameter.</p>
    pub fn filter(&self) -> ::std::option::Option<&crate::types::FilterExpression> {
        self.filter.as_ref()
    }
}
impl DatasetParameter {
    /// Creates a new builder-style object to manufacture [`DatasetParameter`](crate::types::DatasetParameter).
    pub fn builder() -> crate::types::builders::DatasetParameterBuilder {
        crate::types::builders::DatasetParameterBuilder::default()
    }
}

/// A builder for [`DatasetParameter`](crate::types::DatasetParameter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DatasetParameterBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::ParameterType>,
    pub(crate) datetime_options: ::std::option::Option<crate::types::DatetimeOptions>,
    pub(crate) create_column: ::std::option::Option<bool>,
    pub(crate) filter: ::std::option::Option<crate::types::FilterExpression>,
}
impl DatasetParameterBuilder {
    /// <p>The name of the parameter that is used in the dataset's Amazon S3 path.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the parameter that is used in the dataset's Amazon S3 path.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The type of the dataset parameter, can be one of a 'String', 'Number' or 'Datetime'.</p>
    pub fn r#type(mut self, input: crate::types::ParameterType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the dataset parameter, can be one of a 'String', 'Number' or 'Datetime'.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::ParameterType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>Additional parameter options such as a format and a timezone. Required for datetime parameters.</p>
    pub fn datetime_options(mut self, input: crate::types::DatetimeOptions) -> Self {
        self.datetime_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Additional parameter options such as a format and a timezone. Required for datetime parameters.</p>
    pub fn set_datetime_options(
        mut self,
        input: ::std::option::Option<crate::types::DatetimeOptions>,
    ) -> Self {
        self.datetime_options = input;
        self
    }
    /// <p>Optional boolean value that defines whether the captured value of this parameter should be used to create a new column in a dataset.</p>
    pub fn create_column(mut self, input: bool) -> Self {
        self.create_column = ::std::option::Option::Some(input);
        self
    }
    /// <p>Optional boolean value that defines whether the captured value of this parameter should be used to create a new column in a dataset.</p>
    pub fn set_create_column(mut self, input: ::std::option::Option<bool>) -> Self {
        self.create_column = input;
        self
    }
    /// <p>The optional filter expression structure to apply additional matching criteria to the parameter.</p>
    pub fn filter(mut self, input: crate::types::FilterExpression) -> Self {
        self.filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>The optional filter expression structure to apply additional matching criteria to the parameter.</p>
    pub fn set_filter(
        mut self,
        input: ::std::option::Option<crate::types::FilterExpression>,
    ) -> Self {
        self.filter = input;
        self
    }
    /// Consumes the builder and constructs a [`DatasetParameter`](crate::types::DatasetParameter).
    pub fn build(self) -> crate::types::DatasetParameter {
        crate::types::DatasetParameter {
            name: self.name,
            r#type: self.r#type,
            datetime_options: self.datetime_options,
            create_column: self.create_column.unwrap_or_default(),
            filter: self.filter,
        }
    }
}
