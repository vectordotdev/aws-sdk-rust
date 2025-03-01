// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Certificate Authority can't issue or revoke a certificate.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InvalidCertificateAuthorityException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl InvalidCertificateAuthorityException {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for InvalidCertificateAuthorityException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "InvalidCertificateAuthorityException")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for InvalidCertificateAuthorityException {}
impl ::aws_http::request_id::RequestId
    for crate::types::error::InvalidCertificateAuthorityException
{
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata
    for InvalidCertificateAuthorityException
{
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl InvalidCertificateAuthorityException {
    /// Creates a new builder-style object to manufacture [`InvalidCertificateAuthorityException`](crate::types::error::InvalidCertificateAuthorityException).
    pub fn builder() -> crate::types::error::builders::InvalidCertificateAuthorityExceptionBuilder {
        crate::types::error::builders::InvalidCertificateAuthorityExceptionBuilder::default()
    }
}

/// A builder for [`InvalidCertificateAuthorityException`](crate::types::error::InvalidCertificateAuthorityException).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InvalidCertificateAuthorityExceptionBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl InvalidCertificateAuthorityExceptionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(
        &mut self,
        meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`InvalidCertificateAuthorityException`](crate::types::error::InvalidCertificateAuthorityException).
    pub fn build(self) -> crate::types::error::InvalidCertificateAuthorityException {
        crate::types::error::InvalidCertificateAuthorityException {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
