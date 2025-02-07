// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::transfer_certificate::_transfer_certificate_output::TransferCertificateOutputBuilder;

pub use crate::operation::transfer_certificate::_transfer_certificate_input::TransferCertificateInputBuilder;

/// Fluent builder constructing a request to `TransferCertificate`.
///
/// <p>Transfers the specified certificate to the specified Amazon Web Services account.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">TransferCertificate</a> action.</p>
/// <p>You can cancel the transfer until it is acknowledged by the recipient.</p>
/// <p>No notification is sent to the transfer destination's account. It is up to the caller to notify the transfer target.</p>
/// <p>The certificate being transferred must not be in the ACTIVE state. You can use the <code>UpdateCertificate</code> action to deactivate it.</p>
/// <p>The certificate must not have any policies attached to it. You can use the <code>DetachPolicy</code> action to detach them.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TransferCertificateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::transfer_certificate::builders::TransferCertificateInputBuilder,
}
impl TransferCertificateFluentBuilder {
    /// Creates a new `TransferCertificate`.
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
            crate::operation::transfer_certificate::TransferCertificate,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::transfer_certificate::TransferCertificateError,
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
        crate::operation::transfer_certificate::TransferCertificateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::transfer_certificate::TransferCertificateError,
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
        crate::operation::transfer_certificate::TransferCertificateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::transfer_certificate::TransferCertificateError,
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
            crate::operation::transfer_certificate::TransferCertificate,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::transfer_certificate::TransferCertificateError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the certificate. (The last part of the certificate ARN contains the certificate ID.)</p>
    pub fn certificate_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.certificate_id(input.into());
        self
    }
    /// <p>The ID of the certificate. (The last part of the certificate ARN contains the certificate ID.)</p>
    pub fn set_certificate_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_certificate_id(input);
        self
    }
    /// <p>The Amazon Web Services account.</p>
    pub fn target_aws_account(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.target_aws_account(input.into());
        self
    }
    /// <p>The Amazon Web Services account.</p>
    pub fn set_target_aws_account(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_target_aws_account(input);
        self
    }
    /// <p>The transfer message.</p>
    pub fn transfer_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.transfer_message(input.into());
        self
    }
    /// <p>The transfer message.</p>
    pub fn set_transfer_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_transfer_message(input);
        self
    }
}
