// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelVariantImportJobInput {
    /// <p>The job's ID.</p>
    #[doc(hidden)]
    pub job_id: ::std::option::Option<::std::string::String>,
}
impl CancelVariantImportJobInput {
    /// <p>The job's ID.</p>
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }
}
impl CancelVariantImportJobInput {
    /// Creates a new builder-style object to manufacture [`CancelVariantImportJobInput`](crate::operation::cancel_variant_import_job::CancelVariantImportJobInput).
    pub fn builder(
    ) -> crate::operation::cancel_variant_import_job::builders::CancelVariantImportJobInputBuilder
    {
        crate::operation::cancel_variant_import_job::builders::CancelVariantImportJobInputBuilder::default()
    }
}

/// A builder for [`CancelVariantImportJobInput`](crate::operation::cancel_variant_import_job::CancelVariantImportJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CancelVariantImportJobInputBuilder {
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
}
impl CancelVariantImportJobInputBuilder {
    /// <p>The job's ID.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The job's ID.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CancelVariantImportJobInput`](crate::operation::cancel_variant_import_job::CancelVariantImportJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::cancel_variant_import_job::CancelVariantImportJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::cancel_variant_import_job::CancelVariantImportJobInput {
                job_id: self.job_id,
            },
        )
    }
}
