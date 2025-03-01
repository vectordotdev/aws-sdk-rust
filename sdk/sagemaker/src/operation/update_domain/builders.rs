// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_domain::_update_domain_output::UpdateDomainOutputBuilder;

pub use crate::operation::update_domain::_update_domain_input::UpdateDomainInputBuilder;

/// Fluent builder constructing a request to `UpdateDomain`.
///
/// <p>Updates the default settings for new user profiles in the domain.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDomainFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_domain::builders::UpdateDomainInputBuilder,
}
impl UpdateDomainFluentBuilder {
    /// Creates a new `UpdateDomain`.
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
            crate::operation::update_domain::UpdateDomain,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_domain::UpdateDomainError>,
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
        crate::operation::update_domain::UpdateDomainOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_domain::UpdateDomainError>,
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
        crate::operation::update_domain::UpdateDomainOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_domain::UpdateDomainError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_domain::UpdateDomain,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_domain::UpdateDomainError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the domain to be updated.</p>
    pub fn domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_id(input.into());
        self
    }
    /// <p>The ID of the domain to be updated.</p>
    pub fn set_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_id(input);
        self
    }
    /// <p>A collection of settings.</p>
    pub fn default_user_settings(mut self, input: crate::types::UserSettings) -> Self {
        self.inner = self.inner.default_user_settings(input);
        self
    }
    /// <p>A collection of settings.</p>
    pub fn set_default_user_settings(
        mut self,
        input: ::std::option::Option<crate::types::UserSettings>,
    ) -> Self {
        self.inner = self.inner.set_default_user_settings(input);
        self
    }
    /// <p>A collection of <code>DomainSettings</code> configuration values to update.</p>
    pub fn domain_settings_for_update(
        mut self,
        input: crate::types::DomainSettingsForUpdate,
    ) -> Self {
        self.inner = self.inner.domain_settings_for_update(input);
        self
    }
    /// <p>A collection of <code>DomainSettings</code> configuration values to update.</p>
    pub fn set_domain_settings_for_update(
        mut self,
        input: ::std::option::Option<crate::types::DomainSettingsForUpdate>,
    ) -> Self {
        self.inner = self.inner.set_domain_settings_for_update(input);
        self
    }
    /// <p>The default settings used to create a space within the Domain.</p>
    pub fn default_space_settings(mut self, input: crate::types::DefaultSpaceSettings) -> Self {
        self.inner = self.inner.default_space_settings(input);
        self
    }
    /// <p>The default settings used to create a space within the Domain.</p>
    pub fn set_default_space_settings(
        mut self,
        input: ::std::option::Option<crate::types::DefaultSpaceSettings>,
    ) -> Self {
        self.inner = self.inner.set_default_space_settings(input);
        self
    }
    /// <p>The entity that creates and manages the required security groups for inter-app communication in <code>VPCOnly</code> mode. Required when <code>CreateDomain.AppNetworkAccessType</code> is <code>VPCOnly</code> and <code>DomainSettings.RStudioServerProDomainSettings.DomainExecutionRoleArn</code> is provided. If setting up the domain for use with RStudio, this value must be set to <code>Service</code>.</p>
    pub fn app_security_group_management(
        mut self,
        input: crate::types::AppSecurityGroupManagement,
    ) -> Self {
        self.inner = self.inner.app_security_group_management(input);
        self
    }
    /// <p>The entity that creates and manages the required security groups for inter-app communication in <code>VPCOnly</code> mode. Required when <code>CreateDomain.AppNetworkAccessType</code> is <code>VPCOnly</code> and <code>DomainSettings.RStudioServerProDomainSettings.DomainExecutionRoleArn</code> is provided. If setting up the domain for use with RStudio, this value must be set to <code>Service</code>.</p>
    pub fn set_app_security_group_management(
        mut self,
        input: ::std::option::Option<crate::types::AppSecurityGroupManagement>,
    ) -> Self {
        self.inner = self.inner.set_app_security_group_management(input);
        self
    }
}
