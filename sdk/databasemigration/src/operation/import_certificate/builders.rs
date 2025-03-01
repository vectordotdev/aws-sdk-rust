// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_certificate::_import_certificate_output::ImportCertificateOutputBuilder;

pub use crate::operation::import_certificate::_import_certificate_input::ImportCertificateInputBuilder;

/// Fluent builder constructing a request to `ImportCertificate`.
///
/// <p>Uploads the specified certificate.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ImportCertificateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_certificate::builders::ImportCertificateInputBuilder,
}
impl ImportCertificateFluentBuilder {
    /// Creates a new `ImportCertificate`.
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
            crate::operation::import_certificate::ImportCertificate,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::import_certificate::ImportCertificateError,
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
        crate::operation::import_certificate::ImportCertificateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::import_certificate::ImportCertificateError,
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
        crate::operation::import_certificate::ImportCertificateOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::import_certificate::ImportCertificateError,
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
            crate::operation::import_certificate::ImportCertificate,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::import_certificate::ImportCertificateError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A customer-assigned name for the certificate. Identifiers must begin with a letter and must contain only ASCII letters, digits, and hyphens. They can't end with a hyphen or contain two consecutive hyphens.</p>
    pub fn certificate_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.certificate_identifier(input.into());
        self
    }
    /// <p>A customer-assigned name for the certificate. Identifiers must begin with a letter and must contain only ASCII letters, digits, and hyphens. They can't end with a hyphen or contain two consecutive hyphens.</p>
    pub fn set_certificate_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_certificate_identifier(input);
        self
    }
    /// <p>The contents of a <code>.pem</code> file, which contains an X.509 certificate.</p>
    pub fn certificate_pem(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.certificate_pem(input.into());
        self
    }
    /// <p>The contents of a <code>.pem</code> file, which contains an X.509 certificate.</p>
    pub fn set_certificate_pem(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_certificate_pem(input);
        self
    }
    /// <p>The location of an imported Oracle Wallet certificate for use with SSL. Provide the name of a <code>.sso</code> file using the <code>fileb://</code> prefix. You can't provide the certificate inline.</p>
    /// <p>Example: <code>filebase64("${path.root}/rds-ca-2019-root.sso")</code> </p>
    pub fn certificate_wallet(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.certificate_wallet(input);
        self
    }
    /// <p>The location of an imported Oracle Wallet certificate for use with SSL. Provide the name of a <code>.sso</code> file using the <code>fileb://</code> prefix. You can't provide the certificate inline.</p>
    /// <p>Example: <code>filebase64("${path.root}/rds-ca-2019-root.sso")</code> </p>
    pub fn set_certificate_wallet(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.inner = self.inner.set_certificate_wallet(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags associated with the certificate.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags associated with the certificate.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
