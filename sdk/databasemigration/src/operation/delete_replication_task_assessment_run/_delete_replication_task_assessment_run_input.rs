// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteReplicationTaskAssessmentRunInput {
    /// <p>Amazon Resource Name (ARN) of the premigration assessment run to be deleted.</p>
    #[doc(hidden)]
    pub replication_task_assessment_run_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteReplicationTaskAssessmentRunInput {
    /// <p>Amazon Resource Name (ARN) of the premigration assessment run to be deleted.</p>
    pub fn replication_task_assessment_run_arn(&self) -> ::std::option::Option<&str> {
        self.replication_task_assessment_run_arn.as_deref()
    }
}
impl DeleteReplicationTaskAssessmentRunInput {
    /// Creates a new builder-style object to manufacture [`DeleteReplicationTaskAssessmentRunInput`](crate::operation::delete_replication_task_assessment_run::DeleteReplicationTaskAssessmentRunInput).
    pub fn builder() -> crate::operation::delete_replication_task_assessment_run::builders::DeleteReplicationTaskAssessmentRunInputBuilder{
        crate::operation::delete_replication_task_assessment_run::builders::DeleteReplicationTaskAssessmentRunInputBuilder::default()
    }
}

/// A builder for [`DeleteReplicationTaskAssessmentRunInput`](crate::operation::delete_replication_task_assessment_run::DeleteReplicationTaskAssessmentRunInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteReplicationTaskAssessmentRunInputBuilder {
    pub(crate) replication_task_assessment_run_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteReplicationTaskAssessmentRunInputBuilder {
    /// <p>Amazon Resource Name (ARN) of the premigration assessment run to be deleted.</p>
    pub fn replication_task_assessment_run_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.replication_task_assessment_run_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of the premigration assessment run to be deleted.</p>
    pub fn set_replication_task_assessment_run_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.replication_task_assessment_run_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteReplicationTaskAssessmentRunInput`](crate::operation::delete_replication_task_assessment_run::DeleteReplicationTaskAssessmentRunInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::delete_replication_task_assessment_run::DeleteReplicationTaskAssessmentRunInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::delete_replication_task_assessment_run::DeleteReplicationTaskAssessmentRunInput {
                replication_task_assessment_run_arn: self.replication_task_assessment_run_arn
                ,
            }
        )
    }
}
