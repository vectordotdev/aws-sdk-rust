// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_decoder_manifest::_import_decoder_manifest_output::ImportDecoderManifestOutputBuilder;

pub use crate::operation::import_decoder_manifest::_import_decoder_manifest_input::ImportDecoderManifestInputBuilder;

/// Fluent builder constructing a request to `ImportDecoderManifest`.
///
/// <p> Creates a decoder manifest using your existing CAN DBC file from your local device. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ImportDecoderManifestFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_decoder_manifest::builders::ImportDecoderManifestInputBuilder,
}
impl ImportDecoderManifestFluentBuilder {
    /// Creates a new `ImportDecoderManifest`.
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
            crate::operation::import_decoder_manifest::ImportDecoderManifest,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::import_decoder_manifest::ImportDecoderManifestError,
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
        crate::operation::import_decoder_manifest::ImportDecoderManifestOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::import_decoder_manifest::ImportDecoderManifestError,
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
        crate::operation::import_decoder_manifest::ImportDecoderManifestOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::import_decoder_manifest::ImportDecoderManifestError,
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
            crate::operation::import_decoder_manifest::ImportDecoderManifest,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::import_decoder_manifest::ImportDecoderManifestError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p> The name of the decoder manifest to import. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p> The name of the decoder manifest to import. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// Appends an item to `networkFileDefinitions`.
    ///
    /// To override the contents of this collection use [`set_network_file_definitions`](Self::set_network_file_definitions).
    ///
    /// <p> The file to load into an Amazon Web Services account. </p>
    pub fn network_file_definitions(mut self, input: crate::types::NetworkFileDefinition) -> Self {
        self.inner = self.inner.network_file_definitions(input);
        self
    }
    /// <p> The file to load into an Amazon Web Services account. </p>
    pub fn set_network_file_definitions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::NetworkFileDefinition>>,
    ) -> Self {
        self.inner = self.inner.set_network_file_definitions(input);
        self
    }
}
