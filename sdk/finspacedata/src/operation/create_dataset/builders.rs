// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_dataset::_create_dataset_output::CreateDatasetOutputBuilder;

pub use crate::operation::create_dataset::_create_dataset_input::CreateDatasetInputBuilder;

/// Fluent builder constructing a request to `CreateDataset`.
///
/// <p>Creates a new FinSpace Dataset.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDatasetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_dataset::builders::CreateDatasetInputBuilder,
}
impl CreateDatasetFluentBuilder {
    /// Creates a new `CreateDataset`.
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
            crate::operation::create_dataset::CreateDataset,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_dataset::CreateDatasetError>,
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
        crate::operation::create_dataset::CreateDatasetOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_dataset::CreateDatasetError>,
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
        crate::operation::create_dataset::CreateDatasetOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_dataset::CreateDatasetError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_dataset::CreateDataset,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_dataset::CreateDatasetError>,
    > {
        self.customize_middleware().await
    }
    /// <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Display title for a FinSpace Dataset.</p>
    pub fn dataset_title(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.dataset_title(input.into());
        self
    }
    /// <p>Display title for a FinSpace Dataset.</p>
    pub fn set_dataset_title(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_dataset_title(input);
        self
    }
    /// <p>The format in which Dataset data is structured.</p>
    /// <ul>
    /// <li> <p> <code>TABULAR</code> – Data is structured in a tabular format.</p> </li>
    /// <li> <p> <code>NON_TABULAR</code> – Data is structured in a non-tabular format.</p> </li>
    /// </ul>
    pub fn kind(mut self, input: crate::types::DatasetKind) -> Self {
        self.inner = self.inner.kind(input);
        self
    }
    /// <p>The format in which Dataset data is structured.</p>
    /// <ul>
    /// <li> <p> <code>TABULAR</code> – Data is structured in a tabular format.</p> </li>
    /// <li> <p> <code>NON_TABULAR</code> – Data is structured in a non-tabular format.</p> </li>
    /// </ul>
    pub fn set_kind(mut self, input: ::std::option::Option<crate::types::DatasetKind>) -> Self {
        self.inner = self.inner.set_kind(input);
        self
    }
    /// <p>Description of a Dataset.</p>
    pub fn dataset_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.dataset_description(input.into());
        self
    }
    /// <p>Description of a Dataset.</p>
    pub fn set_dataset_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_dataset_description(input);
        self
    }
    /// <p>Contact information for a Dataset owner.</p>
    pub fn owner_info(mut self, input: crate::types::DatasetOwnerInfo) -> Self {
        self.inner = self.inner.owner_info(input);
        self
    }
    /// <p>Contact information for a Dataset owner.</p>
    pub fn set_owner_info(
        mut self,
        input: ::std::option::Option<crate::types::DatasetOwnerInfo>,
    ) -> Self {
        self.inner = self.inner.set_owner_info(input);
        self
    }
    /// <p>Permission group parameters for Dataset permissions.</p>
    pub fn permission_group_params(mut self, input: crate::types::PermissionGroupParams) -> Self {
        self.inner = self.inner.permission_group_params(input);
        self
    }
    /// <p>Permission group parameters for Dataset permissions.</p>
    pub fn set_permission_group_params(
        mut self,
        input: ::std::option::Option<crate::types::PermissionGroupParams>,
    ) -> Self {
        self.inner = self.inner.set_permission_group_params(input);
        self
    }
    /// <p>The unique resource identifier for a Dataset.</p>
    pub fn alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.alias(input.into());
        self
    }
    /// <p>The unique resource identifier for a Dataset.</p>
    pub fn set_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_alias(input);
        self
    }
    /// <p>Definition for a schema on a tabular Dataset.</p>
    pub fn schema_definition(mut self, input: crate::types::SchemaUnion) -> Self {
        self.inner = self.inner.schema_definition(input);
        self
    }
    /// <p>Definition for a schema on a tabular Dataset.</p>
    pub fn set_schema_definition(
        mut self,
        input: ::std::option::Option<crate::types::SchemaUnion>,
    ) -> Self {
        self.inner = self.inner.set_schema_definition(input);
        self
    }
}
