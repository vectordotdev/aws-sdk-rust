// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Represents the input of a DescribeAlertManagerDefinition operation.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeAlertManagerDefinitionInput {
    /// The ID of the workspace to describe.
    #[doc(hidden)]
    pub workspace_id: ::std::option::Option<::std::string::String>,
}
impl DescribeAlertManagerDefinitionInput {
    /// The ID of the workspace to describe.
    pub fn workspace_id(&self) -> ::std::option::Option<&str> {
        self.workspace_id.as_deref()
    }
}
impl DescribeAlertManagerDefinitionInput {
    /// Creates a new builder-style object to manufacture [`DescribeAlertManagerDefinitionInput`](crate::operation::describe_alert_manager_definition::DescribeAlertManagerDefinitionInput).
    pub fn builder() -> crate::operation::describe_alert_manager_definition::builders::DescribeAlertManagerDefinitionInputBuilder{
        crate::operation::describe_alert_manager_definition::builders::DescribeAlertManagerDefinitionInputBuilder::default()
    }
}

/// A builder for [`DescribeAlertManagerDefinitionInput`](crate::operation::describe_alert_manager_definition::DescribeAlertManagerDefinitionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeAlertManagerDefinitionInputBuilder {
    pub(crate) workspace_id: ::std::option::Option<::std::string::String>,
}
impl DescribeAlertManagerDefinitionInputBuilder {
    /// The ID of the workspace to describe.
    pub fn workspace_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workspace_id = ::std::option::Option::Some(input.into());
        self
    }
    /// The ID of the workspace to describe.
    pub fn set_workspace_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workspace_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAlertManagerDefinitionInput`](crate::operation::describe_alert_manager_definition::DescribeAlertManagerDefinitionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_alert_manager_definition::DescribeAlertManagerDefinitionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_alert_manager_definition::DescribeAlertManagerDefinitionInput {
                workspace_id: self.workspace_id
                ,
            }
        )
    }
}
