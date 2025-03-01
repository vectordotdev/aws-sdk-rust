// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_user::_update_user_output::UpdateUserOutputBuilder;

pub use crate::operation::update_user::_update_user_input::UpdateUserInputBuilder;

/// Fluent builder constructing a request to `UpdateUser`.
///
/// <p>Modifies the details of the specified user account. You cannot update the <code>userId</code> for a user.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateUserFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_user::builders::UpdateUserInputBuilder,
}
impl UpdateUserFluentBuilder {
    /// Creates a new `UpdateUser`.
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
            crate::operation::update_user::UpdateUser,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_user::UpdateUserError>,
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
        crate::operation::update_user::UpdateUserOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_user::UpdateUserError>,
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
        crate::operation::update_user::UpdateUserOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_user::UpdateUserError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_user::UpdateUser,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_user::UpdateUserError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique identifier for the user account to update.</p>
    pub fn user_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_id(input.into());
        self
    }
    /// <p>The unique identifier for the user account to update.</p>
    pub fn set_user_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_id(input);
        self
    }
    /// <p>The option to indicate the type of user.</p>
    /// <ul>
    /// <li> <p> <code>SUPER_USER</code>– A user with permission to all the functionality and data in FinSpace.</p> </li>
    /// <li> <p> <code>APP_USER</code> – A user with specific permissions in FinSpace. The users are assigned permissions by adding them to a permission group.</p> </li>
    /// </ul>
    pub fn r#type(mut self, input: crate::types::UserType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The option to indicate the type of user.</p>
    /// <ul>
    /// <li> <p> <code>SUPER_USER</code>– A user with permission to all the functionality and data in FinSpace.</p> </li>
    /// <li> <p> <code>APP_USER</code> – A user with specific permissions in FinSpace. The users are assigned permissions by adding them to a permission group.</p> </li>
    /// </ul>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::UserType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The first name of the user.</p>
    pub fn first_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.first_name(input.into());
        self
    }
    /// <p>The first name of the user.</p>
    pub fn set_first_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_first_name(input);
        self
    }
    /// <p>The last name of the user.</p>
    pub fn last_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.last_name(input.into());
        self
    }
    /// <p>The last name of the user.</p>
    pub fn set_last_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_last_name(input);
        self
    }
    /// <p>The option to indicate whether the user can use the <code>GetProgrammaticAccessCredentials</code> API to obtain credentials that can then be used to access other FinSpace Data API operations.</p>
    /// <ul>
    /// <li> <p> <code>ENABLED</code> – The user has permissions to use the APIs.</p> </li>
    /// <li> <p> <code>DISABLED</code> – The user does not have permissions to use any APIs.</p> </li>
    /// </ul>
    pub fn api_access(mut self, input: crate::types::ApiAccess) -> Self {
        self.inner = self.inner.api_access(input);
        self
    }
    /// <p>The option to indicate whether the user can use the <code>GetProgrammaticAccessCredentials</code> API to obtain credentials that can then be used to access other FinSpace Data API operations.</p>
    /// <ul>
    /// <li> <p> <code>ENABLED</code> – The user has permissions to use the APIs.</p> </li>
    /// <li> <p> <code>DISABLED</code> – The user does not have permissions to use any APIs.</p> </li>
    /// </ul>
    pub fn set_api_access(mut self, input: ::std::option::Option<crate::types::ApiAccess>) -> Self {
        self.inner = self.inner.set_api_access(input);
        self
    }
    /// <p>The ARN identifier of an AWS user or role that is allowed to call the <code>GetProgrammaticAccessCredentials</code> API to obtain a credentials token for a specific FinSpace user. This must be an IAM role within your FinSpace account.</p>
    pub fn api_access_principal_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.api_access_principal_arn(input.into());
        self
    }
    /// <p>The ARN identifier of an AWS user or role that is allowed to call the <code>GetProgrammaticAccessCredentials</code> API to obtain a credentials token for a specific FinSpace user. This must be an IAM role within your FinSpace account.</p>
    pub fn set_api_access_principal_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_api_access_principal_arn(input);
        self
    }
    /// <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
