// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the details of the <code>ScheduleLambdaFunctionFailed</code> event. It isn't set for other event types.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ScheduleLambdaFunctionFailedEventAttributes {
    /// <p>The ID provided in the <code>ScheduleLambdaFunction</code> decision that failed. </p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the Lambda function.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The cause of the failure. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p> <note>
    /// <p>If <code>cause</code> is set to <code>OPERATION_NOT_PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    /// </note>
    #[doc(hidden)]
    pub cause: ::std::option::Option<crate::types::ScheduleLambdaFunctionFailedCause>,
    /// <p>The ID of the <code>LambdaFunctionCompleted</code> event corresponding to the decision that resulted in scheduling this Lambda task. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    #[doc(hidden)]
    pub decision_task_completed_event_id: i64,
}
impl ScheduleLambdaFunctionFailedEventAttributes {
    /// <p>The ID provided in the <code>ScheduleLambdaFunction</code> decision that failed. </p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The name of the Lambda function.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The cause of the failure. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p> <note>
    /// <p>If <code>cause</code> is set to <code>OPERATION_NOT_PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    /// </note>
    pub fn cause(&self) -> ::std::option::Option<&crate::types::ScheduleLambdaFunctionFailedCause> {
        self.cause.as_ref()
    }
    /// <p>The ID of the <code>LambdaFunctionCompleted</code> event corresponding to the decision that resulted in scheduling this Lambda task. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    pub fn decision_task_completed_event_id(&self) -> i64 {
        self.decision_task_completed_event_id
    }
}
impl ScheduleLambdaFunctionFailedEventAttributes {
    /// Creates a new builder-style object to manufacture [`ScheduleLambdaFunctionFailedEventAttributes`](crate::types::ScheduleLambdaFunctionFailedEventAttributes).
    pub fn builder() -> crate::types::builders::ScheduleLambdaFunctionFailedEventAttributesBuilder {
        crate::types::builders::ScheduleLambdaFunctionFailedEventAttributesBuilder::default()
    }
}

/// A builder for [`ScheduleLambdaFunctionFailedEventAttributes`](crate::types::ScheduleLambdaFunctionFailedEventAttributes).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ScheduleLambdaFunctionFailedEventAttributesBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) cause: ::std::option::Option<crate::types::ScheduleLambdaFunctionFailedCause>,
    pub(crate) decision_task_completed_event_id: ::std::option::Option<i64>,
}
impl ScheduleLambdaFunctionFailedEventAttributesBuilder {
    /// <p>The ID provided in the <code>ScheduleLambdaFunction</code> decision that failed. </p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID provided in the <code>ScheduleLambdaFunction</code> decision that failed. </p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The name of the Lambda function.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Lambda function.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The cause of the failure. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p> <note>
    /// <p>If <code>cause</code> is set to <code>OPERATION_NOT_PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    /// </note>
    pub fn cause(mut self, input: crate::types::ScheduleLambdaFunctionFailedCause) -> Self {
        self.cause = ::std::option::Option::Some(input);
        self
    }
    /// <p>The cause of the failure. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p> <note>
    /// <p>If <code>cause</code> is set to <code>OPERATION_NOT_PERMITTED</code>, the decision failed because it lacked sufficient permissions. For details and example IAM policies, see <a href="https://docs.aws.amazon.com/amazonswf/latest/developerguide/swf-dev-iam.html">Using IAM to Manage Access to Amazon SWF Workflows</a> in the <i>Amazon SWF Developer Guide</i>.</p>
    /// </note>
    pub fn set_cause(
        mut self,
        input: ::std::option::Option<crate::types::ScheduleLambdaFunctionFailedCause>,
    ) -> Self {
        self.cause = input;
        self
    }
    /// <p>The ID of the <code>LambdaFunctionCompleted</code> event corresponding to the decision that resulted in scheduling this Lambda task. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    pub fn decision_task_completed_event_id(mut self, input: i64) -> Self {
        self.decision_task_completed_event_id = ::std::option::Option::Some(input);
        self
    }
    /// <p>The ID of the <code>LambdaFunctionCompleted</code> event corresponding to the decision that resulted in scheduling this Lambda task. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    pub fn set_decision_task_completed_event_id(
        mut self,
        input: ::std::option::Option<i64>,
    ) -> Self {
        self.decision_task_completed_event_id = input;
        self
    }
    /// Consumes the builder and constructs a [`ScheduleLambdaFunctionFailedEventAttributes`](crate::types::ScheduleLambdaFunctionFailedEventAttributes).
    pub fn build(self) -> crate::types::ScheduleLambdaFunctionFailedEventAttributes {
        crate::types::ScheduleLambdaFunctionFailedEventAttributes {
            id: self.id,
            name: self.name,
            cause: self.cause,
            decision_task_completed_event_id: self
                .decision_task_completed_event_id
                .unwrap_or_default(),
        }
    }
}
