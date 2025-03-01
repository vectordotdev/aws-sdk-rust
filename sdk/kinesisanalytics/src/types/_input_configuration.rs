// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>When you start your application, you provide this configuration, which identifies the input source and the point in the input source at which you want the application to start processing records.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InputConfiguration {
    /// <p>Input source ID. You can get this ID by calling the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>Point at which you want the application to start processing records from the streaming source.</p>
    #[doc(hidden)]
    pub input_starting_position_configuration:
        ::std::option::Option<crate::types::InputStartingPositionConfiguration>,
}
impl InputConfiguration {
    /// <p>Input source ID. You can get this ID by calling the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>Point at which you want the application to start processing records from the streaming source.</p>
    pub fn input_starting_position_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::InputStartingPositionConfiguration> {
        self.input_starting_position_configuration.as_ref()
    }
}
impl InputConfiguration {
    /// Creates a new builder-style object to manufacture [`InputConfiguration`](crate::types::InputConfiguration).
    pub fn builder() -> crate::types::builders::InputConfigurationBuilder {
        crate::types::builders::InputConfigurationBuilder::default()
    }
}

/// A builder for [`InputConfiguration`](crate::types::InputConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InputConfigurationBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) input_starting_position_configuration:
        ::std::option::Option<crate::types::InputStartingPositionConfiguration>,
}
impl InputConfigurationBuilder {
    /// <p>Input source ID. You can get this ID by calling the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Input source ID. You can get this ID by calling the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>Point at which you want the application to start processing records from the streaming source.</p>
    pub fn input_starting_position_configuration(
        mut self,
        input: crate::types::InputStartingPositionConfiguration,
    ) -> Self {
        self.input_starting_position_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Point at which you want the application to start processing records from the streaming source.</p>
    pub fn set_input_starting_position_configuration(
        mut self,
        input: ::std::option::Option<crate::types::InputStartingPositionConfiguration>,
    ) -> Self {
        self.input_starting_position_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`InputConfiguration`](crate::types::InputConfiguration).
    pub fn build(self) -> crate::types::InputConfiguration {
        crate::types::InputConfiguration {
            id: self.id,
            input_starting_position_configuration: self.input_starting_position_configuration,
        }
    }
}
