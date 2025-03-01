// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::admin_user_global_sign_out::_admin_user_global_sign_out_output::AdminUserGlobalSignOutOutputBuilder;

pub use crate::operation::admin_user_global_sign_out::_admin_user_global_sign_out_input::AdminUserGlobalSignOutInputBuilder;

/// Fluent builder constructing a request to `AdminUserGlobalSignOut`.
///
/// <p>Signs out a user from all devices. You must sign <code>AdminUserGlobalSignOut</code> requests with Amazon Web Services credentials. It also invalidates all refresh tokens that Amazon Cognito has issued to a user. The user's current access and ID tokens remain valid until they expire. By default, access and ID tokens expire one hour after they're issued. A user can still use a hosted UI cookie to retrieve new tokens for the duration of the cookie validity period of 1 hour.</p>
/// <p>Calling this action requires developer credentials.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AdminUserGlobalSignOutFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::admin_user_global_sign_out::builders::AdminUserGlobalSignOutInputBuilder,
}
impl AdminUserGlobalSignOutFluentBuilder {
    /// Creates a new `AdminUserGlobalSignOut`.
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
            crate::operation::admin_user_global_sign_out::AdminUserGlobalSignOut,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::admin_user_global_sign_out::AdminUserGlobalSignOutError,
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
        crate::operation::admin_user_global_sign_out::AdminUserGlobalSignOutOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::admin_user_global_sign_out::AdminUserGlobalSignOutError,
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
        crate::operation::admin_user_global_sign_out::AdminUserGlobalSignOutOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::admin_user_global_sign_out::AdminUserGlobalSignOutError,
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
            crate::operation::admin_user_global_sign_out::AdminUserGlobalSignOut,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::admin_user_global_sign_out::AdminUserGlobalSignOutError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The user pool ID.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The user pool ID.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>The user name.</p>
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.username(input.into());
        self
    }
    /// <p>The user name.</p>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_username(input);
        self
    }
}
