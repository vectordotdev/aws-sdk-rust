// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartApplicationInput {
    /// <p>Name of the application.</p>
    #[doc(hidden)]
    pub application_name: ::std::option::Option<::std::string::String>,
    /// <p>Identifies the specific input, by ID, that the application starts consuming. Amazon Kinesis Analytics starts reading the streaming source associated with the input. You can also specify where in the streaming source you want Amazon Kinesis Analytics to start reading.</p>
    #[doc(hidden)]
    pub input_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::InputConfiguration>>,
}
impl StartApplicationInput {
    /// <p>Name of the application.</p>
    pub fn application_name(&self) -> ::std::option::Option<&str> {
        self.application_name.as_deref()
    }
    /// <p>Identifies the specific input, by ID, that the application starts consuming. Amazon Kinesis Analytics starts reading the streaming source associated with the input. You can also specify where in the streaming source you want Amazon Kinesis Analytics to start reading.</p>
    pub fn input_configurations(
        &self,
    ) -> ::std::option::Option<&[crate::types::InputConfiguration]> {
        self.input_configurations.as_deref()
    }
}
impl StartApplicationInput {
    /// Creates a new builder-style object to manufacture [`StartApplicationInput`](crate::operation::start_application::StartApplicationInput).
    pub fn builder() -> crate::operation::start_application::builders::StartApplicationInputBuilder
    {
        crate::operation::start_application::builders::StartApplicationInputBuilder::default()
    }
}

/// A builder for [`StartApplicationInput`](crate::operation::start_application::StartApplicationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartApplicationInputBuilder {
    pub(crate) application_name: ::std::option::Option<::std::string::String>,
    pub(crate) input_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::InputConfiguration>>,
}
impl StartApplicationInputBuilder {
    /// <p>Name of the application.</p>
    pub fn application_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the application.</p>
    pub fn set_application_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_name = input;
        self
    }
    /// Appends an item to `input_configurations`.
    ///
    /// To override the contents of this collection use [`set_input_configurations`](Self::set_input_configurations).
    ///
    /// <p>Identifies the specific input, by ID, that the application starts consuming. Amazon Kinesis Analytics starts reading the streaming source associated with the input. You can also specify where in the streaming source you want Amazon Kinesis Analytics to start reading.</p>
    pub fn input_configurations(mut self, input: crate::types::InputConfiguration) -> Self {
        let mut v = self.input_configurations.unwrap_or_default();
        v.push(input);
        self.input_configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>Identifies the specific input, by ID, that the application starts consuming. Amazon Kinesis Analytics starts reading the streaming source associated with the input. You can also specify where in the streaming source you want Amazon Kinesis Analytics to start reading.</p>
    pub fn set_input_configurations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::InputConfiguration>>,
    ) -> Self {
        self.input_configurations = input;
        self
    }
    /// Consumes the builder and constructs a [`StartApplicationInput`](crate::operation::start_application::StartApplicationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_application::StartApplicationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::start_application::StartApplicationInput {
            application_name: self.application_name,
            input_configurations: self.input_configurations,
        })
    }
}
