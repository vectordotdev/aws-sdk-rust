// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteBatchImportJobInput {
    /// <p>The ID of the batch import job to delete. </p>
    #[doc(hidden)]
    pub job_id: ::std::option::Option<::std::string::String>,
}
impl DeleteBatchImportJobInput {
    /// <p>The ID of the batch import job to delete. </p>
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }
}
impl DeleteBatchImportJobInput {
    /// Creates a new builder-style object to manufacture [`DeleteBatchImportJobInput`](crate::operation::delete_batch_import_job::DeleteBatchImportJobInput).
    pub fn builder(
    ) -> crate::operation::delete_batch_import_job::builders::DeleteBatchImportJobInputBuilder {
        crate::operation::delete_batch_import_job::builders::DeleteBatchImportJobInputBuilder::default()
    }
}

/// A builder for [`DeleteBatchImportJobInput`](crate::operation::delete_batch_import_job::DeleteBatchImportJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteBatchImportJobInputBuilder {
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
}
impl DeleteBatchImportJobInputBuilder {
    /// <p>The ID of the batch import job to delete. </p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the batch import job to delete. </p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteBatchImportJobInput`](crate::operation::delete_batch_import_job::DeleteBatchImportJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_batch_import_job::DeleteBatchImportJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_batch_import_job::DeleteBatchImportJobInput {
                job_id: self.job_id,
            },
        )
    }
}
