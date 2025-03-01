// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_profile::_create_profile_output::CreateProfileOutputBuilder;

pub use crate::operation::create_profile::_create_profile_input::CreateProfileInputBuilder;

/// Fluent builder constructing a request to `CreateProfile`.
///
/// <p>Creates the local or partner profile to use for AS2 transfers.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateProfileFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_profile::builders::CreateProfileInputBuilder,
}
impl CreateProfileFluentBuilder {
    /// Creates a new `CreateProfile`.
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
            crate::operation::create_profile::CreateProfile,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_profile::CreateProfileError>,
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
        crate::operation::create_profile::CreateProfileOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_profile::CreateProfileError>,
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
        crate::operation::create_profile::CreateProfileOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_profile::CreateProfileError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_profile::CreateProfile,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_profile::CreateProfileError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The <code>As2Id</code> is the <i>AS2-name</i>, as defined in the <a href="https://datatracker.ietf.org/doc/html/rfc4130">RFC 4130</a>. For inbound transfers, this is the <code>AS2-From</code> header for the AS2 messages sent from the partner. For outbound connectors, this is the <code>AS2-To</code> header for the AS2 messages sent to the partner using the <code>StartFileTransfer</code> API operation. This ID cannot include spaces.</p>
    pub fn as2_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.as2_id(input.into());
        self
    }
    /// <p>The <code>As2Id</code> is the <i>AS2-name</i>, as defined in the <a href="https://datatracker.ietf.org/doc/html/rfc4130">RFC 4130</a>. For inbound transfers, this is the <code>AS2-From</code> header for the AS2 messages sent from the partner. For outbound connectors, this is the <code>AS2-To</code> header for the AS2 messages sent to the partner using the <code>StartFileTransfer</code> API operation. This ID cannot include spaces.</p>
    pub fn set_as2_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_as2_id(input);
        self
    }
    /// <p>Determines the type of profile to create:</p>
    /// <ul>
    /// <li> <p>Specify <code>LOCAL</code> to create a local profile. A local profile represents the AS2-enabled Transfer Family server organization or party.</p> </li>
    /// <li> <p>Specify <code>PARTNER</code> to create a partner profile. A partner profile represents a remote organization, external to Transfer Family.</p> </li>
    /// </ul>
    pub fn profile_type(mut self, input: crate::types::ProfileType) -> Self {
        self.inner = self.inner.profile_type(input);
        self
    }
    /// <p>Determines the type of profile to create:</p>
    /// <ul>
    /// <li> <p>Specify <code>LOCAL</code> to create a local profile. A local profile represents the AS2-enabled Transfer Family server organization or party.</p> </li>
    /// <li> <p>Specify <code>PARTNER</code> to create a partner profile. A partner profile represents a remote organization, external to Transfer Family.</p> </li>
    /// </ul>
    pub fn set_profile_type(
        mut self,
        input: ::std::option::Option<crate::types::ProfileType>,
    ) -> Self {
        self.inner = self.inner.set_profile_type(input);
        self
    }
    /// Appends an item to `CertificateIds`.
    ///
    /// To override the contents of this collection use [`set_certificate_ids`](Self::set_certificate_ids).
    ///
    /// <p>An array of identifiers for the imported certificates. You use this identifier for working with profiles and partner profiles.</p>
    pub fn certificate_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.certificate_ids(input.into());
        self
    }
    /// <p>An array of identifiers for the imported certificates. You use this identifier for working with profiles and partner profiles.</p>
    pub fn set_certificate_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_certificate_ids(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Key-value pairs that can be used to group and search for AS2 profiles.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Key-value pairs that can be used to group and search for AS2 profiles.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
