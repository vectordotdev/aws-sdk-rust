// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_package_import_job::_create_package_import_job_output::CreatePackageImportJobOutputBuilder;

pub use crate::operation::create_package_import_job::_create_package_import_job_input::CreatePackageImportJobInputBuilder;

/// Fluent builder constructing a request to `CreatePackageImportJob`.
///
/// <p>Imports a node package.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreatePackageImportJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::create_package_import_job::builders::CreatePackageImportJobInputBuilder,
}
impl CreatePackageImportJobFluentBuilder {
    /// Creates a new `CreatePackageImportJob`.
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
            crate::operation::create_package_import_job::CreatePackageImportJob,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_package_import_job::CreatePackageImportJobError,
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
        crate::operation::create_package_import_job::CreatePackageImportJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_package_import_job::CreatePackageImportJobError,
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
        crate::operation::create_package_import_job::CreatePackageImportJobOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_package_import_job::CreatePackageImportJobError,
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
            crate::operation::create_package_import_job::CreatePackageImportJob,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_package_import_job::CreatePackageImportJobError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A job type for the package import job.</p>
    pub fn job_type(mut self, input: crate::types::PackageImportJobType) -> Self {
        self.inner = self.inner.job_type(input);
        self
    }
    /// <p>A job type for the package import job.</p>
    pub fn set_job_type(
        mut self,
        input: ::std::option::Option<crate::types::PackageImportJobType>,
    ) -> Self {
        self.inner = self.inner.set_job_type(input);
        self
    }
    /// <p>An input config for the package import job.</p>
    pub fn input_config(mut self, input: crate::types::PackageImportJobInputConfig) -> Self {
        self.inner = self.inner.input_config(input);
        self
    }
    /// <p>An input config for the package import job.</p>
    pub fn set_input_config(
        mut self,
        input: ::std::option::Option<crate::types::PackageImportJobInputConfig>,
    ) -> Self {
        self.inner = self.inner.set_input_config(input);
        self
    }
    /// <p>An output config for the package import job.</p>
    pub fn output_config(mut self, input: crate::types::PackageImportJobOutputConfig) -> Self {
        self.inner = self.inner.output_config(input);
        self
    }
    /// <p>An output config for the package import job.</p>
    pub fn set_output_config(
        mut self,
        input: ::std::option::Option<crate::types::PackageImportJobOutputConfig>,
    ) -> Self {
        self.inner = self.inner.set_output_config(input);
        self
    }
    /// <p>A client token for the package import job.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A client token for the package import job.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// Appends an item to `JobTags`.
    ///
    /// To override the contents of this collection use [`set_job_tags`](Self::set_job_tags).
    ///
    /// <p>Tags for the package import job.</p>
    pub fn job_tags(mut self, input: crate::types::JobResourceTags) -> Self {
        self.inner = self.inner.job_tags(input);
        self
    }
    /// <p>Tags for the package import job.</p>
    pub fn set_job_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::JobResourceTags>>,
    ) -> Self {
        self.inner = self.inner.set_job_tags(input);
        self
    }
}
