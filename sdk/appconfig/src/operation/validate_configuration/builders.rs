// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::validate_configuration::_validate_configuration_output::ValidateConfigurationOutputBuilder;

pub use crate::operation::validate_configuration::_validate_configuration_input::ValidateConfigurationInputBuilder;

/// Fluent builder constructing a request to `ValidateConfiguration`.
///
/// <p>Uses the validators in a configuration profile to validate a configuration.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ValidateConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::validate_configuration::builders::ValidateConfigurationInputBuilder,
}
impl ValidateConfigurationFluentBuilder {
    /// Creates a new `ValidateConfiguration`.
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
            crate::operation::validate_configuration::ValidateConfiguration,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::validate_configuration::ValidateConfigurationError,
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
        crate::operation::validate_configuration::ValidateConfigurationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::validate_configuration::ValidateConfigurationError,
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
        crate::operation::validate_configuration::ValidateConfigurationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::validate_configuration::ValidateConfigurationError,
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
            crate::operation::validate_configuration::ValidateConfiguration,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::validate_configuration::ValidateConfigurationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The application ID.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The application ID.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The configuration profile ID.</p>
    pub fn configuration_profile_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.configuration_profile_id(input.into());
        self
    }
    /// <p>The configuration profile ID.</p>
    pub fn set_configuration_profile_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_configuration_profile_id(input);
        self
    }
    /// <p>The version of the configuration to validate.</p>
    pub fn configuration_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.configuration_version(input.into());
        self
    }
    /// <p>The version of the configuration to validate.</p>
    pub fn set_configuration_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_configuration_version(input);
        self
    }
}
