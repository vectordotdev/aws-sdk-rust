// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::change_password::_change_password_output::ChangePasswordOutputBuilder;

pub use crate::operation::change_password::_change_password_input::ChangePasswordInputBuilder;

/// Fluent builder constructing a request to `ChangePassword`.
///
/// <p>Changes the password for a specified user in a user pool.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ChangePasswordFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::change_password::builders::ChangePasswordInputBuilder,
}
impl ChangePasswordFluentBuilder {
    /// Creates a new `ChangePassword`.
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
            crate::operation::change_password::ChangePassword,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::change_password::ChangePasswordError>,
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
        crate::operation::change_password::ChangePasswordOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::change_password::ChangePasswordError>,
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
        crate::operation::change_password::ChangePasswordOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::change_password::ChangePasswordError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::change_password::ChangePassword,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::change_password::ChangePasswordError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The old password.</p>
    pub fn previous_password(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.previous_password(input.into());
        self
    }
    /// <p>The old password.</p>
    pub fn set_previous_password(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_previous_password(input);
        self
    }
    /// <p>The new password.</p>
    pub fn proposed_password(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.proposed_password(input.into());
        self
    }
    /// <p>The new password.</p>
    pub fn set_proposed_password(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_proposed_password(input);
        self
    }
    /// <p>A valid access token that Amazon Cognito issued to the user whose password you want to change.</p>
    pub fn access_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.access_token(input.into());
        self
    }
    /// <p>A valid access token that Amazon Cognito issued to the user whose password you want to change.</p>
    pub fn set_access_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_access_token(input);
        self
    }
}
