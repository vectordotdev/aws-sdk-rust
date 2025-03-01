// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_reference_store::_create_reference_store_output::CreateReferenceStoreOutputBuilder;

pub use crate::operation::create_reference_store::_create_reference_store_input::CreateReferenceStoreInputBuilder;

/// Fluent builder constructing a request to `CreateReferenceStore`.
///
/// <p>Creates a reference store.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateReferenceStoreFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_reference_store::builders::CreateReferenceStoreInputBuilder,
}
impl CreateReferenceStoreFluentBuilder {
    /// Creates a new `CreateReferenceStore`.
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
            crate::operation::create_reference_store::CreateReferenceStore,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_reference_store::CreateReferenceStoreError,
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
        crate::operation::create_reference_store::CreateReferenceStoreOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_reference_store::CreateReferenceStoreError,
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
        crate::operation::create_reference_store::CreateReferenceStoreOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_reference_store::CreateReferenceStoreError,
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
            crate::operation::create_reference_store::CreateReferenceStore,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_reference_store::CreateReferenceStoreError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A name for the store.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A name for the store.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A description for the store.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the store.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Server-side encryption (SSE) settings for the store.</p>
    pub fn sse_config(mut self, input: crate::types::SseConfig) -> Self {
        self.inner = self.inner.sse_config(input);
        self
    }
    /// <p>Server-side encryption (SSE) settings for the store.</p>
    pub fn set_sse_config(mut self, input: ::std::option::Option<crate::types::SseConfig>) -> Self {
        self.inner = self.inner.set_sse_config(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags for the store.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Tags for the store.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>To ensure that requests don't run multiple times, specify a unique token for each request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>To ensure that requests don't run multiple times, specify a unique token for each request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
