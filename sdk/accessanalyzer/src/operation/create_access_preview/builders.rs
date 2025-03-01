// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_access_preview::_create_access_preview_output::CreateAccessPreviewOutputBuilder;

pub use crate::operation::create_access_preview::_create_access_preview_input::CreateAccessPreviewInputBuilder;

/// Fluent builder constructing a request to `CreateAccessPreview`.
///
/// <p>Creates an access preview that allows you to preview IAM Access Analyzer findings for your resource before deploying resource permissions.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAccessPreviewFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_access_preview::builders::CreateAccessPreviewInputBuilder,
}
impl CreateAccessPreviewFluentBuilder {
    /// Creates a new `CreateAccessPreview`.
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
            crate::operation::create_access_preview::CreateAccessPreview,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_access_preview::CreateAccessPreviewError,
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
        crate::operation::create_access_preview::CreateAccessPreviewOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_access_preview::CreateAccessPreviewError,
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
        crate::operation::create_access_preview::CreateAccessPreviewOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_access_preview::CreateAccessPreviewError,
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
            crate::operation::create_access_preview::CreateAccessPreview,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_access_preview::CreateAccessPreviewError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-getting-started.html#permission-resources">ARN of the account analyzer</a> used to generate the access preview. You can only create an access preview for analyzers with an <code>Account</code> type and <code>Active</code> status.</p>
    pub fn analyzer_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.analyzer_arn(input.into());
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-getting-started.html#permission-resources">ARN of the account analyzer</a> used to generate the access preview. You can only create an access preview for analyzers with an <code>Account</code> type and <code>Active</code> status.</p>
    pub fn set_analyzer_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_analyzer_arn(input);
        self
    }
    /// Adds a key-value pair to `configurations`.
    ///
    /// To override the contents of this collection use [`set_configurations`](Self::set_configurations).
    ///
    /// <p>Access control configuration for your resource that is used to generate the access preview. The access preview includes findings for external access allowed to the resource with the proposed access control configuration. The configuration must contain exactly one element.</p>
    pub fn configurations(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::Configuration,
    ) -> Self {
        self.inner = self.inner.configurations(k.into(), v);
        self
    }
    /// <p>Access control configuration for your resource that is used to generate the access preview. The access preview includes findings for external access allowed to the resource with the proposed access control configuration. The configuration must contain exactly one element.</p>
    pub fn set_configurations(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::Configuration>,
        >,
    ) -> Self {
        self.inner = self.inner.set_configurations(input);
        self
    }
    /// <p>A client token.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A client token.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
