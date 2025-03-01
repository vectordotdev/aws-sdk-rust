// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_security_hub::_disable_security_hub_output::DisableSecurityHubOutputBuilder;

pub use crate::operation::disable_security_hub::_disable_security_hub_input::DisableSecurityHubInputBuilder;

/// Fluent builder constructing a request to `DisableSecurityHub`.
///
/// <p>Disables Security Hub in your account only in the current Region. To disable Security Hub in all Regions, you must submit one request per Region where you have enabled Security Hub.</p>
/// <p>When you disable Security Hub for an administrator account, it doesn't disable Security Hub for any associated member accounts.</p>
/// <p>When you disable Security Hub, your existing findings and insights and any Security Hub configuration settings are deleted after 90 days and cannot be recovered. Any standards that were enabled are disabled, and your administrator and member account associations are removed.</p>
/// <p>If you want to save your existing findings, you must export them before you disable Security Hub.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisableSecurityHubFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_security_hub::builders::DisableSecurityHubInputBuilder,
}
impl DisableSecurityHubFluentBuilder {
    /// Creates a new `DisableSecurityHub`.
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
            crate::operation::disable_security_hub::DisableSecurityHub,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_security_hub::DisableSecurityHubError,
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
        crate::operation::disable_security_hub::DisableSecurityHubOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_security_hub::DisableSecurityHubError,
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
        crate::operation::disable_security_hub::DisableSecurityHubOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_security_hub::DisableSecurityHubError,
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
            crate::operation::disable_security_hub::DisableSecurityHub,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disable_security_hub::DisableSecurityHubError,
        >,
    > {
        self.customize_middleware().await
    }
}
