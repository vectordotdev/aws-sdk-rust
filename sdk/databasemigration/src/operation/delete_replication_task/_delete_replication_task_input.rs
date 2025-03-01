// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteReplicationTaskInput {
    /// <p>The Amazon Resource Name (ARN) of the replication task to be deleted.</p>
    #[doc(hidden)]
    pub replication_task_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteReplicationTaskInput {
    /// <p>The Amazon Resource Name (ARN) of the replication task to be deleted.</p>
    pub fn replication_task_arn(&self) -> ::std::option::Option<&str> {
        self.replication_task_arn.as_deref()
    }
}
impl DeleteReplicationTaskInput {
    /// Creates a new builder-style object to manufacture [`DeleteReplicationTaskInput`](crate::operation::delete_replication_task::DeleteReplicationTaskInput).
    pub fn builder(
    ) -> crate::operation::delete_replication_task::builders::DeleteReplicationTaskInputBuilder
    {
        crate::operation::delete_replication_task::builders::DeleteReplicationTaskInputBuilder::default()
    }
}

/// A builder for [`DeleteReplicationTaskInput`](crate::operation::delete_replication_task::DeleteReplicationTaskInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteReplicationTaskInputBuilder {
    pub(crate) replication_task_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteReplicationTaskInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the replication task to be deleted.</p>
    pub fn replication_task_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.replication_task_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the replication task to be deleted.</p>
    pub fn set_replication_task_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.replication_task_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteReplicationTaskInput`](crate::operation::delete_replication_task::DeleteReplicationTaskInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_replication_task::DeleteReplicationTaskInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_replication_task::DeleteReplicationTaskInput {
                replication_task_arn: self.replication_task_arn,
            },
        )
    }
}
