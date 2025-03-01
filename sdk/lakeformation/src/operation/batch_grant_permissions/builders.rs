// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_grant_permissions::_batch_grant_permissions_output::BatchGrantPermissionsOutputBuilder;

pub use crate::operation::batch_grant_permissions::_batch_grant_permissions_input::BatchGrantPermissionsInputBuilder;

/// Fluent builder constructing a request to `BatchGrantPermissions`.
///
/// <p>Batch operation to grant permissions to the principal.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchGrantPermissionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_grant_permissions::builders::BatchGrantPermissionsInputBuilder,
}
impl BatchGrantPermissionsFluentBuilder {
    /// Creates a new `BatchGrantPermissions`.
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
            crate::operation::batch_grant_permissions::BatchGrantPermissions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_grant_permissions::BatchGrantPermissionsError,
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
        crate::operation::batch_grant_permissions::BatchGrantPermissionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_grant_permissions::BatchGrantPermissionsError,
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
        crate::operation::batch_grant_permissions::BatchGrantPermissionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_grant_permissions::BatchGrantPermissionsError,
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
            crate::operation::batch_grant_permissions::BatchGrantPermissions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_grant_permissions::BatchGrantPermissionsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p>
    pub fn catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.catalog_id(input.into());
        self
    }
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p>
    pub fn set_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_catalog_id(input);
        self
    }
    /// Appends an item to `Entries`.
    ///
    /// To override the contents of this collection use [`set_entries`](Self::set_entries).
    ///
    /// <p>A list of up to 20 entries for resource permissions to be granted by batch operation to the principal.</p>
    pub fn entries(mut self, input: crate::types::BatchPermissionsRequestEntry) -> Self {
        self.inner = self.inner.entries(input);
        self
    }
    /// <p>A list of up to 20 entries for resource permissions to be granted by batch operation to the principal.</p>
    pub fn set_entries(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::BatchPermissionsRequestEntry>>,
    ) -> Self {
        self.inner = self.inner.set_entries(input);
        self
    }
}
