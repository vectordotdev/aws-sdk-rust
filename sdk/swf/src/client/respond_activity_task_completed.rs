// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RespondActivityTaskCompleted`](crate::operation::respond_activity_task_completed::builders::RespondActivityTaskCompletedFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`task_token(impl ::std::convert::Into<String>)`](crate::operation::respond_activity_task_completed::builders::RespondActivityTaskCompletedFluentBuilder::task_token) / [`set_task_token(Option<String>)`](crate::operation::respond_activity_task_completed::builders::RespondActivityTaskCompletedFluentBuilder::set_task_token): <p>The <code>taskToken</code> of the <code>ActivityTask</code>.</p> <important>   <p> <code>taskToken</code> is generated by the service and should be treated as an opaque value. If the task is passed to another process, its <code>taskToken</code> must also be passed. This enables it to provide its progress and respond with results.</p>  </important>
    ///   - [`result(impl ::std::convert::Into<String>)`](crate::operation::respond_activity_task_completed::builders::RespondActivityTaskCompletedFluentBuilder::result) / [`set_result(Option<String>)`](crate::operation::respond_activity_task_completed::builders::RespondActivityTaskCompletedFluentBuilder::set_result): <p>The result of the activity task. It is a free form string that is implementation specific.</p>
    /// - On success, responds with [`RespondActivityTaskCompletedOutput`](crate::operation::respond_activity_task_completed::RespondActivityTaskCompletedOutput)
    /// - On failure, responds with [`SdkError<RespondActivityTaskCompletedError>`](crate::operation::respond_activity_task_completed::RespondActivityTaskCompletedError)
    pub fn respond_activity_task_completed(&self) -> crate::operation::respond_activity_task_completed::builders::RespondActivityTaskCompletedFluentBuilder{
        crate::operation::respond_activity_task_completed::builders::RespondActivityTaskCompletedFluentBuilder::new(self.handle.clone())
    }
}
