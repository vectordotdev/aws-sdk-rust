// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetMLTaskRun`](crate::operation::get_ml_task_run::builders::GetMLTaskRunFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transform_id(impl ::std::convert::Into<String>)`](crate::operation::get_ml_task_run::builders::GetMLTaskRunFluentBuilder::transform_id) / [`set_transform_id(Option<String>)`](crate::operation::get_ml_task_run::builders::GetMLTaskRunFluentBuilder::set_transform_id): <p>The unique identifier of the machine learning transform.</p>
    ///   - [`task_run_id(impl ::std::convert::Into<String>)`](crate::operation::get_ml_task_run::builders::GetMLTaskRunFluentBuilder::task_run_id) / [`set_task_run_id(Option<String>)`](crate::operation::get_ml_task_run::builders::GetMLTaskRunFluentBuilder::set_task_run_id): <p>The unique identifier of the task run.</p>
    /// - On success, responds with [`GetMlTaskRunOutput`](crate::operation::get_ml_task_run::GetMlTaskRunOutput) with field(s):
    ///   - [`transform_id(Option<String>)`](crate::operation::get_ml_task_run::GetMlTaskRunOutput::transform_id): <p>The unique identifier of the task run.</p>
    ///   - [`task_run_id(Option<String>)`](crate::operation::get_ml_task_run::GetMlTaskRunOutput::task_run_id): <p>The unique run identifier associated with this run.</p>
    ///   - [`status(Option<TaskStatusType>)`](crate::operation::get_ml_task_run::GetMlTaskRunOutput::status): <p>The status for this task run.</p>
    ///   - [`log_group_name(Option<String>)`](crate::operation::get_ml_task_run::GetMlTaskRunOutput::log_group_name): <p>The names of the log groups that are associated with the task run.</p>
    ///   - [`properties(Option<TaskRunProperties>)`](crate::operation::get_ml_task_run::GetMlTaskRunOutput::properties): <p>The list of properties that are associated with the task run.</p>
    ///   - [`error_string(Option<String>)`](crate::operation::get_ml_task_run::GetMlTaskRunOutput::error_string): <p>The error strings that are associated with the task run.</p>
    ///   - [`started_on(Option<DateTime>)`](crate::operation::get_ml_task_run::GetMlTaskRunOutput::started_on): <p>The date and time when this task run started.</p>
    ///   - [`last_modified_on(Option<DateTime>)`](crate::operation::get_ml_task_run::GetMlTaskRunOutput::last_modified_on): <p>The date and time when this task run was last modified.</p>
    ///   - [`completed_on(Option<DateTime>)`](crate::operation::get_ml_task_run::GetMlTaskRunOutput::completed_on): <p>The date and time when this task run was completed.</p>
    ///   - [`execution_time(i32)`](crate::operation::get_ml_task_run::GetMlTaskRunOutput::execution_time): <p>The amount of time (in seconds) that the task run consumed resources.</p>
    /// - On failure, responds with [`SdkError<GetMLTaskRunError>`](crate::operation::get_ml_task_run::GetMLTaskRunError)
    pub fn get_ml_task_run(
        &self,
    ) -> crate::operation::get_ml_task_run::builders::GetMLTaskRunFluentBuilder {
        crate::operation::get_ml_task_run::builders::GetMLTaskRunFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
