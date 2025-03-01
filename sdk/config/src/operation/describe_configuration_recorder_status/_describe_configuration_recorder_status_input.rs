// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The input for the <code>DescribeConfigurationRecorderStatus</code> action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeConfigurationRecorderStatusInput {
    /// <p>The name(s) of the configuration recorder. If the name is not specified, the action returns the current status of all the configuration recorders associated with the account.</p>
    #[doc(hidden)]
    pub configuration_recorder_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DescribeConfigurationRecorderStatusInput {
    /// <p>The name(s) of the configuration recorder. If the name is not specified, the action returns the current status of all the configuration recorders associated with the account.</p>
    pub fn configuration_recorder_names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.configuration_recorder_names.as_deref()
    }
}
impl DescribeConfigurationRecorderStatusInput {
    /// Creates a new builder-style object to manufacture [`DescribeConfigurationRecorderStatusInput`](crate::operation::describe_configuration_recorder_status::DescribeConfigurationRecorderStatusInput).
    pub fn builder() -> crate::operation::describe_configuration_recorder_status::builders::DescribeConfigurationRecorderStatusInputBuilder{
        crate::operation::describe_configuration_recorder_status::builders::DescribeConfigurationRecorderStatusInputBuilder::default()
    }
}

/// A builder for [`DescribeConfigurationRecorderStatusInput`](crate::operation::describe_configuration_recorder_status::DescribeConfigurationRecorderStatusInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeConfigurationRecorderStatusInputBuilder {
    pub(crate) configuration_recorder_names:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DescribeConfigurationRecorderStatusInputBuilder {
    /// Appends an item to `configuration_recorder_names`.
    ///
    /// To override the contents of this collection use [`set_configuration_recorder_names`](Self::set_configuration_recorder_names).
    ///
    /// <p>The name(s) of the configuration recorder. If the name is not specified, the action returns the current status of all the configuration recorders associated with the account.</p>
    pub fn configuration_recorder_names(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.configuration_recorder_names.unwrap_or_default();
        v.push(input.into());
        self.configuration_recorder_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>The name(s) of the configuration recorder. If the name is not specified, the action returns the current status of all the configuration recorders associated with the account.</p>
    pub fn set_configuration_recorder_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.configuration_recorder_names = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeConfigurationRecorderStatusInput`](crate::operation::describe_configuration_recorder_status::DescribeConfigurationRecorderStatusInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::describe_configuration_recorder_status::DescribeConfigurationRecorderStatusInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::describe_configuration_recorder_status::DescribeConfigurationRecorderStatusInput {
                configuration_recorder_names: self.configuration_recorder_names
                ,
            }
        )
    }
}
