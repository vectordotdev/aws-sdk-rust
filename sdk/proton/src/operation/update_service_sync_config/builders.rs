// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_service_sync_config::_update_service_sync_config_output::UpdateServiceSyncConfigOutputBuilder;

pub use crate::operation::update_service_sync_config::_update_service_sync_config_input::UpdateServiceSyncConfigInputBuilder;

/// Fluent builder constructing a request to `UpdateServiceSyncConfig`.
///
/// <p>Update the Proton Ops config file.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateServiceSyncConfigFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::update_service_sync_config::builders::UpdateServiceSyncConfigInputBuilder,
}
impl UpdateServiceSyncConfigFluentBuilder {
    /// Creates a new `UpdateServiceSyncConfig`.
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
            crate::operation::update_service_sync_config::UpdateServiceSyncConfig,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_sync_config::UpdateServiceSyncConfigError,
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
        crate::operation::update_service_sync_config::UpdateServiceSyncConfigOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_sync_config::UpdateServiceSyncConfigError,
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
        crate::operation::update_service_sync_config::UpdateServiceSyncConfigOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_sync_config::UpdateServiceSyncConfigError,
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
            crate::operation::update_service_sync_config::UpdateServiceSyncConfig,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_sync_config::UpdateServiceSyncConfigError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the service the Proton Ops file is for.</p>
    pub fn service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_name(input.into());
        self
    }
    /// <p>The name of the service the Proton Ops file is for.</p>
    pub fn set_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_name(input);
        self
    }
    /// <p>The name of the repository provider where the Proton Ops file is found.</p>
    pub fn repository_provider(mut self, input: crate::types::RepositoryProvider) -> Self {
        self.inner = self.inner.repository_provider(input);
        self
    }
    /// <p>The name of the repository provider where the Proton Ops file is found.</p>
    pub fn set_repository_provider(
        mut self,
        input: ::std::option::Option<crate::types::RepositoryProvider>,
    ) -> Self {
        self.inner = self.inner.set_repository_provider(input);
        self
    }
    /// <p>The name of the repository where the Proton Ops file is found.</p>
    pub fn repository_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.repository_name(input.into());
        self
    }
    /// <p>The name of the repository where the Proton Ops file is found.</p>
    pub fn set_repository_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_repository_name(input);
        self
    }
    /// <p>The name of the code repository branch where the Proton Ops file is found.</p>
    pub fn branch(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.branch(input.into());
        self
    }
    /// <p>The name of the code repository branch where the Proton Ops file is found.</p>
    pub fn set_branch(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_branch(input);
        self
    }
    /// <p>The path to the Proton Ops file.</p>
    pub fn file_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.file_path(input.into());
        self
    }
    /// <p>The path to the Proton Ops file.</p>
    pub fn set_file_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_file_path(input);
        self
    }
}
