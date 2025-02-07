// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SendWorkflowStepStateInput {
    /// <p>A unique identifier for the workflow.</p>
    #[doc(hidden)]
    pub workflow_id: ::std::option::Option<::std::string::String>,
    /// <p>A unique identifier for the execution of a workflow.</p>
    #[doc(hidden)]
    pub execution_id: ::std::option::Option<::std::string::String>,
    /// <p>Used to distinguish between multiple callbacks for multiple Lambda steps within the same execution.</p>
    #[doc(hidden)]
    pub token: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether the specified step succeeded or failed.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::CustomStepStatus>,
}
impl SendWorkflowStepStateInput {
    /// <p>A unique identifier for the workflow.</p>
    pub fn workflow_id(&self) -> ::std::option::Option<&str> {
        self.workflow_id.as_deref()
    }
    /// <p>A unique identifier for the execution of a workflow.</p>
    pub fn execution_id(&self) -> ::std::option::Option<&str> {
        self.execution_id.as_deref()
    }
    /// <p>Used to distinguish between multiple callbacks for multiple Lambda steps within the same execution.</p>
    pub fn token(&self) -> ::std::option::Option<&str> {
        self.token.as_deref()
    }
    /// <p>Indicates whether the specified step succeeded or failed.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::CustomStepStatus> {
        self.status.as_ref()
    }
}
impl SendWorkflowStepStateInput {
    /// Creates a new builder-style object to manufacture [`SendWorkflowStepStateInput`](crate::operation::send_workflow_step_state::SendWorkflowStepStateInput).
    pub fn builder(
    ) -> crate::operation::send_workflow_step_state::builders::SendWorkflowStepStateInputBuilder
    {
        crate::operation::send_workflow_step_state::builders::SendWorkflowStepStateInputBuilder::default()
    }
}

/// A builder for [`SendWorkflowStepStateInput`](crate::operation::send_workflow_step_state::SendWorkflowStepStateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SendWorkflowStepStateInputBuilder {
    pub(crate) workflow_id: ::std::option::Option<::std::string::String>,
    pub(crate) execution_id: ::std::option::Option<::std::string::String>,
    pub(crate) token: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::CustomStepStatus>,
}
impl SendWorkflowStepStateInputBuilder {
    /// <p>A unique identifier for the workflow.</p>
    pub fn workflow_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workflow_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the workflow.</p>
    pub fn set_workflow_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workflow_id = input;
        self
    }
    /// <p>A unique identifier for the execution of a workflow.</p>
    pub fn execution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.execution_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the execution of a workflow.</p>
    pub fn set_execution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.execution_id = input;
        self
    }
    /// <p>Used to distinguish between multiple callbacks for multiple Lambda steps within the same execution.</p>
    pub fn token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Used to distinguish between multiple callbacks for multiple Lambda steps within the same execution.</p>
    pub fn set_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.token = input;
        self
    }
    /// <p>Indicates whether the specified step succeeded or failed.</p>
    pub fn status(mut self, input: crate::types::CustomStepStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the specified step succeeded or failed.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::CustomStepStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`SendWorkflowStepStateInput`](crate::operation::send_workflow_step_state::SendWorkflowStepStateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::send_workflow_step_state::SendWorkflowStepStateInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::send_workflow_step_state::SendWorkflowStepStateInput {
                workflow_id: self.workflow_id,
                execution_id: self.execution_id,
                token: self.token,
                status: self.status,
            },
        )
    }
}
