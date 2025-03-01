// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Represents the output of a DescribeAlertManagerDefinition operation.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeAlertManagerDefinitionOutput {
    /// The properties of the selected workspace's alert manager definition.
    #[doc(hidden)]
    pub alert_manager_definition:
        ::std::option::Option<crate::types::AlertManagerDefinitionDescription>,
    _request_id: Option<String>,
}
impl DescribeAlertManagerDefinitionOutput {
    /// The properties of the selected workspace's alert manager definition.
    pub fn alert_manager_definition(
        &self,
    ) -> ::std::option::Option<&crate::types::AlertManagerDefinitionDescription> {
        self.alert_manager_definition.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeAlertManagerDefinitionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeAlertManagerDefinitionOutput {
    /// Creates a new builder-style object to manufacture [`DescribeAlertManagerDefinitionOutput`](crate::operation::describe_alert_manager_definition::DescribeAlertManagerDefinitionOutput).
    pub fn builder() -> crate::operation::describe_alert_manager_definition::builders::DescribeAlertManagerDefinitionOutputBuilder{
        crate::operation::describe_alert_manager_definition::builders::DescribeAlertManagerDefinitionOutputBuilder::default()
    }
}

/// A builder for [`DescribeAlertManagerDefinitionOutput`](crate::operation::describe_alert_manager_definition::DescribeAlertManagerDefinitionOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeAlertManagerDefinitionOutputBuilder {
    pub(crate) alert_manager_definition:
        ::std::option::Option<crate::types::AlertManagerDefinitionDescription>,
    _request_id: Option<String>,
}
impl DescribeAlertManagerDefinitionOutputBuilder {
    /// The properties of the selected workspace's alert manager definition.
    pub fn alert_manager_definition(
        mut self,
        input: crate::types::AlertManagerDefinitionDescription,
    ) -> Self {
        self.alert_manager_definition = ::std::option::Option::Some(input);
        self
    }
    /// The properties of the selected workspace's alert manager definition.
    pub fn set_alert_manager_definition(
        mut self,
        input: ::std::option::Option<crate::types::AlertManagerDefinitionDescription>,
    ) -> Self {
        self.alert_manager_definition = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAlertManagerDefinitionOutput`](crate::operation::describe_alert_manager_definition::DescribeAlertManagerDefinitionOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_alert_manager_definition::DescribeAlertManagerDefinitionOutput
    {
        crate::operation::describe_alert_manager_definition::DescribeAlertManagerDefinitionOutput {
            alert_manager_definition: self.alert_manager_definition,
            _request_id: self._request_id,
        }
    }
}
