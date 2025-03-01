// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetWorkflowStepGroupInput {
    /// <p>The ID of the step group.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the migration workflow.</p>
    #[doc(hidden)]
    pub workflow_id: ::std::option::Option<::std::string::String>,
}
impl GetWorkflowStepGroupInput {
    /// <p>The ID of the step group.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The ID of the migration workflow.</p>
    pub fn workflow_id(&self) -> ::std::option::Option<&str> {
        self.workflow_id.as_deref()
    }
}
impl GetWorkflowStepGroupInput {
    /// Creates a new builder-style object to manufacture [`GetWorkflowStepGroupInput`](crate::operation::get_workflow_step_group::GetWorkflowStepGroupInput).
    pub fn builder(
    ) -> crate::operation::get_workflow_step_group::builders::GetWorkflowStepGroupInputBuilder {
        crate::operation::get_workflow_step_group::builders::GetWorkflowStepGroupInputBuilder::default()
    }
}

/// A builder for [`GetWorkflowStepGroupInput`](crate::operation::get_workflow_step_group::GetWorkflowStepGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetWorkflowStepGroupInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) workflow_id: ::std::option::Option<::std::string::String>,
}
impl GetWorkflowStepGroupInputBuilder {
    /// <p>The ID of the step group.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the step group.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The ID of the migration workflow.</p>
    pub fn workflow_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workflow_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the migration workflow.</p>
    pub fn set_workflow_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workflow_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetWorkflowStepGroupInput`](crate::operation::get_workflow_step_group::GetWorkflowStepGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_workflow_step_group::GetWorkflowStepGroupInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_workflow_step_group::GetWorkflowStepGroupInput {
                id: self.id,
                workflow_id: self.workflow_id,
            },
        )
    }
}
