// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A workflow in list of workflows.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListWorkflowsItem {
    /// <p>The type of workflow. The only supported value is APPFLOW_INTEGRATION.</p>
    #[doc(hidden)]
    pub workflow_type: ::std::option::Option<crate::types::WorkflowType>,
    /// <p>Unique identifier for the workflow.</p>
    #[doc(hidden)]
    pub workflow_id: ::std::option::Option<::std::string::String>,
    /// <p>Status of workflow execution.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::Status>,
    /// <p>Description for workflow execution status.</p>
    #[doc(hidden)]
    pub status_description: ::std::option::Option<::std::string::String>,
    /// <p>Creation timestamp for workflow.</p>
    #[doc(hidden)]
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Last updated timestamp for workflow.</p>
    #[doc(hidden)]
    pub last_updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ListWorkflowsItem {
    /// <p>The type of workflow. The only supported value is APPFLOW_INTEGRATION.</p>
    pub fn workflow_type(&self) -> ::std::option::Option<&crate::types::WorkflowType> {
        self.workflow_type.as_ref()
    }
    /// <p>Unique identifier for the workflow.</p>
    pub fn workflow_id(&self) -> ::std::option::Option<&str> {
        self.workflow_id.as_deref()
    }
    /// <p>Status of workflow execution.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::Status> {
        self.status.as_ref()
    }
    /// <p>Description for workflow execution status.</p>
    pub fn status_description(&self) -> ::std::option::Option<&str> {
        self.status_description.as_deref()
    }
    /// <p>Creation timestamp for workflow.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>Last updated timestamp for workflow.</p>
    pub fn last_updated_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated_at.as_ref()
    }
}
impl ListWorkflowsItem {
    /// Creates a new builder-style object to manufacture [`ListWorkflowsItem`](crate::types::ListWorkflowsItem).
    pub fn builder() -> crate::types::builders::ListWorkflowsItemBuilder {
        crate::types::builders::ListWorkflowsItemBuilder::default()
    }
}

/// A builder for [`ListWorkflowsItem`](crate::types::ListWorkflowsItem).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListWorkflowsItemBuilder {
    pub(crate) workflow_type: ::std::option::Option<crate::types::WorkflowType>,
    pub(crate) workflow_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::Status>,
    pub(crate) status_description: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ListWorkflowsItemBuilder {
    /// <p>The type of workflow. The only supported value is APPFLOW_INTEGRATION.</p>
    pub fn workflow_type(mut self, input: crate::types::WorkflowType) -> Self {
        self.workflow_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of workflow. The only supported value is APPFLOW_INTEGRATION.</p>
    pub fn set_workflow_type(
        mut self,
        input: ::std::option::Option<crate::types::WorkflowType>,
    ) -> Self {
        self.workflow_type = input;
        self
    }
    /// <p>Unique identifier for the workflow.</p>
    pub fn workflow_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workflow_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique identifier for the workflow.</p>
    pub fn set_workflow_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workflow_id = input;
        self
    }
    /// <p>Status of workflow execution.</p>
    pub fn status(mut self, input: crate::types::Status) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Status of workflow execution.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::Status>) -> Self {
        self.status = input;
        self
    }
    /// <p>Description for workflow execution status.</p>
    pub fn status_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.status_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Description for workflow execution status.</p>
    pub fn set_status_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.status_description = input;
        self
    }
    /// <p>Creation timestamp for workflow.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>Creation timestamp for workflow.</p>
    pub fn set_created_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_at = input;
        self
    }
    /// <p>Last updated timestamp for workflow.</p>
    pub fn last_updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>Last updated timestamp for workflow.</p>
    pub fn set_last_updated_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_updated_at = input;
        self
    }
    /// Consumes the builder and constructs a [`ListWorkflowsItem`](crate::types::ListWorkflowsItem).
    pub fn build(self) -> crate::types::ListWorkflowsItem {
        crate::types::ListWorkflowsItem {
            workflow_type: self.workflow_type,
            workflow_id: self.workflow_id,
            status: self.status,
            status_description: self.status_description,
            created_at: self.created_at,
            last_updated_at: self.last_updated_at,
        }
    }
}
