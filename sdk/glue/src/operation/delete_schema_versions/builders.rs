// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_schema_versions::_delete_schema_versions_output::DeleteSchemaVersionsOutputBuilder;

pub use crate::operation::delete_schema_versions::_delete_schema_versions_input::DeleteSchemaVersionsInputBuilder;

/// Fluent builder constructing a request to `DeleteSchemaVersions`.
///
/// <p>Remove versions from the specified schema. A version number or range may be supplied. If the compatibility mode forbids deleting of a version that is necessary, such as BACKWARDS_FULL, an error is returned. Calling the <code>GetSchemaVersions</code> API after this call will list the status of the deleted versions.</p>
/// <p>When the range of version numbers contain check pointed version, the API will return a 409 conflict and will not proceed with the deletion. You have to remove the checkpoint first using the <code>DeleteSchemaCheckpoint</code> API before using this API.</p>
/// <p>You cannot use the <code>DeleteSchemaVersions</code> API to delete the first schema version in the schema set. The first schema version can only be deleted by the <code>DeleteSchema</code> API. This operation will also delete the attached <code>SchemaVersionMetadata</code> under the schema versions. Hard deletes will be enforced on the database.</p>
/// <p>If the compatibility mode forbids deleting of a version that is necessary, such as BACKWARDS_FULL, an error is returned.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteSchemaVersionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_schema_versions::builders::DeleteSchemaVersionsInputBuilder,
}
impl DeleteSchemaVersionsFluentBuilder {
    /// Creates a new `DeleteSchemaVersions`.
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
            crate::operation::delete_schema_versions::DeleteSchemaVersions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_schema_versions::DeleteSchemaVersionsError,
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
        crate::operation::delete_schema_versions::DeleteSchemaVersionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_schema_versions::DeleteSchemaVersionsError,
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
        crate::operation::delete_schema_versions::DeleteSchemaVersionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_schema_versions::DeleteSchemaVersionsError,
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
            crate::operation::delete_schema_versions::DeleteSchemaVersions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_schema_versions::DeleteSchemaVersionsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>This is a wrapper structure that may contain the schema name and Amazon Resource Name (ARN).</p>
    pub fn schema_id(mut self, input: crate::types::SchemaId) -> Self {
        self.inner = self.inner.schema_id(input);
        self
    }
    /// <p>This is a wrapper structure that may contain the schema name and Amazon Resource Name (ARN).</p>
    pub fn set_schema_id(mut self, input: ::std::option::Option<crate::types::SchemaId>) -> Self {
        self.inner = self.inner.set_schema_id(input);
        self
    }
    /// <p>A version range may be supplied which may be of the format:</p>
    /// <ul>
    /// <li> <p>a single version number, 5</p> </li>
    /// <li> <p>a range, 5-8 : deletes versions 5, 6, 7, 8</p> </li>
    /// </ul>
    pub fn versions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.versions(input.into());
        self
    }
    /// <p>A version range may be supplied which may be of the format:</p>
    /// <ul>
    /// <li> <p>a single version number, 5</p> </li>
    /// <li> <p>a range, 5-8 : deletes versions 5, 6, 7, 8</p> </li>
    /// </ul>
    pub fn set_versions(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_versions(input);
        self
    }
}
