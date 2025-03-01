// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the details of the <code>CompleteWorkflowExecutionFailed</code> event.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CompleteWorkflowExecutionFailedEventAttributes {
    /// <p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note>
    /// <p>If <code>cause</code> is set to <code>OPERATION_NOT_PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    /// </note>
    #[doc(hidden)]
    pub cause: ::std::option::Option<crate::types::CompleteWorkflowExecutionFailedCause>,
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>CompleteWorkflowExecution</code> decision to complete this execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    #[doc(hidden)]
    pub decision_task_completed_event_id: i64,
}
impl CompleteWorkflowExecutionFailedEventAttributes {
    /// <p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note>
    /// <p>If <code>cause</code> is set to <code>OPERATION_NOT_PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    /// </note>
    pub fn cause(
        &self,
    ) -> ::std::option::Option<&crate::types::CompleteWorkflowExecutionFailedCause> {
        self.cause.as_ref()
    }
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>CompleteWorkflowExecution</code> decision to complete this execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    pub fn decision_task_completed_event_id(&self) -> i64 {
        self.decision_task_completed_event_id
    }
}
impl CompleteWorkflowExecutionFailedEventAttributes {
    /// Creates a new builder-style object to manufacture [`CompleteWorkflowExecutionFailedEventAttributes`](crate::types::CompleteWorkflowExecutionFailedEventAttributes).
    pub fn builder() -> crate::types::builders::CompleteWorkflowExecutionFailedEventAttributesBuilder
    {
        crate::types::builders::CompleteWorkflowExecutionFailedEventAttributesBuilder::default()
    }
}

/// A builder for [`CompleteWorkflowExecutionFailedEventAttributes`](crate::types::CompleteWorkflowExecutionFailedEventAttributes).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CompleteWorkflowExecutionFailedEventAttributesBuilder {
    pub(crate) cause: ::std::option::Option<crate::types::CompleteWorkflowExecutionFailedCause>,
    pub(crate) decision_task_completed_event_id: ::std::option::Option<i64>,
}
impl CompleteWorkflowExecutionFailedEventAttributesBuilder {
    /// <p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note>
    /// <p>If <code>cause</code> is set to <code>OPERATION_NOT_PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    /// </note>
    pub fn cause(mut self, input: crate::types::CompleteWorkflowExecutionFailedCause) -> Self {
        self.cause = ::std::option::Option::Some(input);
        self
    }
    /// <p>The cause of the failure. This information is generated by the system and can be useful for diagnostic purposes.</p> <note>
    /// <p>If <code>cause</code> is set to <code>OPERATION_NOT_PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    /// </note>
    pub fn set_cause(
        mut self,
        input: ::std::option::Option<crate::types::CompleteWorkflowExecutionFailedCause>,
    ) -> Self {
        self.cause = input;
        self
    }
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>CompleteWorkflowExecution</code> decision to complete this execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    pub fn decision_task_completed_event_id(mut self, input: i64) -> Self {
        self.decision_task_completed_event_id = ::std::option::Option::Some(input);
        self
    }
    /// <p>The ID of the <code>DecisionTaskCompleted</code> event corresponding to the decision task that resulted in the <code>CompleteWorkflowExecution</code> decision to complete this execution. This information can be useful for diagnosing problems by tracing back the chain of events leading up to this event.</p>
    pub fn set_decision_task_completed_event_id(
        mut self,
        input: ::std::option::Option<i64>,
    ) -> Self {
        self.decision_task_completed_event_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CompleteWorkflowExecutionFailedEventAttributes`](crate::types::CompleteWorkflowExecutionFailedEventAttributes).
    pub fn build(self) -> crate::types::CompleteWorkflowExecutionFailedEventAttributes {
        crate::types::CompleteWorkflowExecutionFailedEventAttributes {
            cause: self.cause,
            decision_task_completed_event_id: self
                .decision_task_completed_event_id
                .unwrap_or_default(),
        }
    }
}
