// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_trust_anchor::_update_trust_anchor_output::UpdateTrustAnchorOutputBuilder;

pub use crate::operation::update_trust_anchor::_update_trust_anchor_input::UpdateTrustAnchorInputBuilder;

/// Fluent builder constructing a request to `UpdateTrustAnchor`.
///
/// <p>Updates a trust anchor. You establish trust between IAM Roles Anywhere and your certificate authority (CA) by configuring a trust anchor. You can define a trust anchor as a reference to an Private Certificate Authority (Private CA) or by uploading a CA certificate. Your Amazon Web Services workloads can authenticate with the trust anchor using certificates issued by the CA in exchange for temporary Amazon Web Services credentials.</p>
/// <p> <b>Required permissions: </b> <code>rolesanywhere:UpdateTrustAnchor</code>. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateTrustAnchorFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_trust_anchor::builders::UpdateTrustAnchorInputBuilder,
}
impl UpdateTrustAnchorFluentBuilder {
    /// Creates a new `UpdateTrustAnchor`.
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
            crate::operation::update_trust_anchor::UpdateTrustAnchor,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_trust_anchor::UpdateTrustAnchorError,
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
        crate::operation::update_trust_anchor::UpdateTrustAnchorOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_trust_anchor::UpdateTrustAnchorError,
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
        crate::operation::update_trust_anchor::UpdateTrustAnchorOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_trust_anchor::UpdateTrustAnchorError,
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
            crate::operation::update_trust_anchor::UpdateTrustAnchor,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_trust_anchor::UpdateTrustAnchorError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique identifier of the trust anchor.</p>
    pub fn trust_anchor_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.trust_anchor_id(input.into());
        self
    }
    /// <p>The unique identifier of the trust anchor.</p>
    pub fn set_trust_anchor_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_trust_anchor_id(input);
        self
    }
    /// <p>The name of the trust anchor.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the trust anchor.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The trust anchor type and its related certificate data.</p>
    pub fn source(mut self, input: crate::types::Source) -> Self {
        self.inner = self.inner.source(input);
        self
    }
    /// <p>The trust anchor type and its related certificate data.</p>
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::Source>) -> Self {
        self.inner = self.inner.set_source(input);
        self
    }
}
