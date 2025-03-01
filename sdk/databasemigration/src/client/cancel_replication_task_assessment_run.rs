// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelReplicationTaskAssessmentRun`](crate::operation::cancel_replication_task_assessment_run::builders::CancelReplicationTaskAssessmentRunFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`replication_task_assessment_run_arn(impl ::std::convert::Into<String>)`](crate::operation::cancel_replication_task_assessment_run::builders::CancelReplicationTaskAssessmentRunFluentBuilder::replication_task_assessment_run_arn) / [`set_replication_task_assessment_run_arn(Option<String>)`](crate::operation::cancel_replication_task_assessment_run::builders::CancelReplicationTaskAssessmentRunFluentBuilder::set_replication_task_assessment_run_arn): <p>Amazon Resource Name (ARN) of the premigration assessment run to be canceled.</p>
    /// - On success, responds with [`CancelReplicationTaskAssessmentRunOutput`](crate::operation::cancel_replication_task_assessment_run::CancelReplicationTaskAssessmentRunOutput) with field(s):
    ///   - [`replication_task_assessment_run(Option<ReplicationTaskAssessmentRun>)`](crate::operation::cancel_replication_task_assessment_run::CancelReplicationTaskAssessmentRunOutput::replication_task_assessment_run): <p>The <code>ReplicationTaskAssessmentRun</code> object for the canceled assessment run.</p>
    /// - On failure, responds with [`SdkError<CancelReplicationTaskAssessmentRunError>`](crate::operation::cancel_replication_task_assessment_run::CancelReplicationTaskAssessmentRunError)
    pub fn cancel_replication_task_assessment_run(&self) -> crate::operation::cancel_replication_task_assessment_run::builders::CancelReplicationTaskAssessmentRunFluentBuilder{
        crate::operation::cancel_replication_task_assessment_run::builders::CancelReplicationTaskAssessmentRunFluentBuilder::new(self.handle.clone())
    }
}
