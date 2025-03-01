// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_user_pool_domain::_update_user_pool_domain_output::UpdateUserPoolDomainOutputBuilder;

pub use crate::operation::update_user_pool_domain::_update_user_pool_domain_input::UpdateUserPoolDomainInputBuilder;

/// Fluent builder constructing a request to `UpdateUserPoolDomain`.
///
/// <p>Updates the Secure Sockets Layer (SSL) certificate for the custom domain for your user pool.</p>
/// <p>You can use this operation to provide the Amazon Resource Name (ARN) of a new certificate to Amazon Cognito. You can't use it to change the domain for a user pool.</p>
/// <p>A custom domain is used to host the Amazon Cognito hosted UI, which provides sign-up and sign-in pages for your application. When you set up a custom domain, you provide a certificate that you manage with Certificate Manager (ACM). When necessary, you can use this operation to change the certificate that you applied to your custom domain.</p>
/// <p>Usually, this is unnecessary following routine certificate renewal with ACM. When you renew your existing certificate in ACM, the ARN for your certificate remains the same, and your custom domain uses the new certificate automatically.</p>
/// <p>However, if you replace your existing certificate with a new one, ACM gives the new certificate a new ARN. To apply the new certificate to your custom domain, you must provide this ARN to Amazon Cognito.</p>
/// <p>When you add your new certificate in ACM, you must choose US East (N. Virginia) as the Amazon Web Services Region.</p>
/// <p>After you submit your request, Amazon Cognito requires up to 1 hour to distribute your new certificate to your custom domain.</p>
/// <p>For more information about adding a custom domain to your user pool, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-add-custom-domain.html">Using Your Own Domain for the Hosted UI</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateUserPoolDomainFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_user_pool_domain::builders::UpdateUserPoolDomainInputBuilder,
}
impl UpdateUserPoolDomainFluentBuilder {
    /// Creates a new `UpdateUserPoolDomain`.
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
            crate::operation::update_user_pool_domain::UpdateUserPoolDomain,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_user_pool_domain::UpdateUserPoolDomainError,
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
        crate::operation::update_user_pool_domain::UpdateUserPoolDomainOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_user_pool_domain::UpdateUserPoolDomainError,
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
        crate::operation::update_user_pool_domain::UpdateUserPoolDomainOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_user_pool_domain::UpdateUserPoolDomainError,
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
            crate::operation::update_user_pool_domain::UpdateUserPoolDomain,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_user_pool_domain::UpdateUserPoolDomainError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The domain name for the custom domain that hosts the sign-up and sign-in pages for your application. One example might be <code>auth.example.com</code>. </p>
    /// <p>This string can include only lowercase letters, numbers, and hyphens. Don't use a hyphen for the first or last character. Use periods to separate subdomain names.</p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain(input.into());
        self
    }
    /// <p>The domain name for the custom domain that hosts the sign-up and sign-in pages for your application. One example might be <code>auth.example.com</code>. </p>
    /// <p>This string can include only lowercase letters, numbers, and hyphens. Don't use a hyphen for the first or last character. Use periods to separate subdomain names.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain(input);
        self
    }
    /// <p>The ID of the user pool that is associated with the custom domain whose certificate you're updating.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The ID of the user pool that is associated with the custom domain whose certificate you're updating.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>The configuration for a custom domain that hosts the sign-up and sign-in pages for your application. Use this object to specify an SSL certificate that is managed by ACM.</p>
    pub fn custom_domain_config(mut self, input: crate::types::CustomDomainConfigType) -> Self {
        self.inner = self.inner.custom_domain_config(input);
        self
    }
    /// <p>The configuration for a custom domain that hosts the sign-up and sign-in pages for your application. Use this object to specify an SSL certificate that is managed by ACM.</p>
    pub fn set_custom_domain_config(
        mut self,
        input: ::std::option::Option<crate::types::CustomDomainConfigType>,
    ) -> Self {
        self.inner = self.inner.set_custom_domain_config(input);
        self
    }
}
