// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::export_vector_enrichment_job::_export_vector_enrichment_job_output::ExportVectorEnrichmentJobOutputBuilder;

pub use crate::operation::export_vector_enrichment_job::_export_vector_enrichment_job_input::ExportVectorEnrichmentJobInputBuilder;

/// Fluent builder constructing a request to `ExportVectorEnrichmentJob`.
///
/// <p>Use this operation to copy results of a Vector Enrichment job to an Amazon S3 location.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ExportVectorEnrichmentJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::export_vector_enrichment_job::builders::ExportVectorEnrichmentJobInputBuilder,
}
impl ExportVectorEnrichmentJobFluentBuilder {
    /// Creates a new `ExportVectorEnrichmentJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::export_vector_enrichment_job::ExportVectorEnrichmentJob,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::export_vector_enrichment_job::ExportVectorEnrichmentJobError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::export_vector_enrichment_job::ExportVectorEnrichmentJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::export_vector_enrichment_job::ExportVectorEnrichmentJobError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::export_vector_enrichment_job::ExportVectorEnrichmentJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::export_vector_enrichment_job::ExportVectorEnrichmentJobError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::export_vector_enrichment_job::ExportVectorEnrichmentJob,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::export_vector_enrichment_job::ExportVectorEnrichmentJobError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (ARN) of the Vector Enrichment job.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Vector Enrichment job.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
    /// <p>A unique token that guarantees that the call to this API is idempotent.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique token that guarantees that the call to this API is idempotent.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM rolewith permission to upload to the location in OutputConfig.</p>
    pub fn execution_role_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.execution_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM rolewith permission to upload to the location in OutputConfig.</p>
    pub fn set_execution_role_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_execution_role_arn(input);
        self
    }
    /// <p>Output location information for exporting Vector Enrichment Job results. </p>
    pub fn output_config(
        mut self,
        input: crate::types::ExportVectorEnrichmentJobOutputConfig,
    ) -> Self {
        self.inner = self.inner.output_config(input);
        self
    }
    /// <p>Output location information for exporting Vector Enrichment Job results. </p>
    pub fn set_output_config(
        mut self,
        input: ::std::option::Option<crate::types::ExportVectorEnrichmentJobOutputConfig>,
    ) -> Self {
        self.inner = self.inner.set_output_config(input);
        self
    }
}
