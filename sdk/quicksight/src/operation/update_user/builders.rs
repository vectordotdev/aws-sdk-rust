// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_user::_update_user_output::UpdateUserOutputBuilder;

pub use crate::operation::update_user::_update_user_input::UpdateUserInputBuilder;

/// Fluent builder constructing a request to `UpdateUser`.
///
/// <p>Updates an Amazon QuickSight user.</p>
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
    /// <p>The Amazon QuickSight user name that you want to update.</p>
    pub fn user_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_name(input.into());
        self
    }
    /// <p>The Amazon QuickSight user name that you want to update.</p>
    pub fn set_user_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_name(input);
        self
    }
    /// <p>The ID for the Amazon Web Services account that the user is in. Currently, you use the ID for the Amazon Web Services account that contains your Amazon QuickSight account.</p>
    pub fn aws_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID for the Amazon Web Services account that the user is in. Currently, you use the ID for the Amazon Web Services account that contains your Amazon QuickSight account.</p>
    pub fn set_aws_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    pub fn namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.namespace(input.into());
        self
    }
    /// <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    pub fn set_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_namespace(input);
        self
    }
    /// <p>The email address of the user that you want to update.</p>
    pub fn email(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.email(input.into());
        self
    }
    /// <p>The email address of the user that you want to update.</p>
    pub fn set_email(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_email(input);
        self
    }
    /// <p>The Amazon QuickSight role of the user. The role can be one of the following default security cohorts:</p>
    /// <ul>
    /// <li> <p> <code>READER</code>: A user who has read-only access to dashboards.</p> </li>
    /// <li> <p> <code>AUTHOR</code>: A user who can create data sources, datasets, analyses, and dashboards.</p> </li>
    /// <li> <p> <code>ADMIN</code>: A user who is an author, who can also manage Amazon QuickSight settings.</p> </li>
    /// </ul>
    /// <p>The name of the Amazon QuickSight role is invisible to the user except for the console screens dealing with permissions.</p>
    pub fn role(mut self, input: crate::types::UserRole) -> Self {
        self.inner = self.inner.role(input);
        self
    }
    /// <p>The Amazon QuickSight role of the user. The role can be one of the following default security cohorts:</p>
    /// <ul>
    /// <li> <p> <code>READER</code>: A user who has read-only access to dashboards.</p> </li>
    /// <li> <p> <code>AUTHOR</code>: A user who can create data sources, datasets, analyses, and dashboards.</p> </li>
    /// <li> <p> <code>ADMIN</code>: A user who is an author, who can also manage Amazon QuickSight settings.</p> </li>
    /// </ul>
    /// <p>The name of the Amazon QuickSight role is invisible to the user except for the console screens dealing with permissions.</p>
    pub fn set_role(mut self, input: ::std::option::Option<crate::types::UserRole>) -> Self {
        self.inner = self.inner.set_role(input);
        self
    }
    /// <p>(Enterprise edition only) The name of the custom permissions profile that you want to assign to this user. Customized permissions allows you to control a user's access by restricting access the following operations:</p>
    /// <ul>
    /// <li> <p>Create and update data sources</p> </li>
    /// <li> <p>Create and update datasets</p> </li>
    /// <li> <p>Create and update email reports</p> </li>
    /// <li> <p>Subscribe to email reports</p> </li>
    /// </ul>
    /// <p>A set of custom permissions includes any combination of these restrictions. Currently, you need to create the profile names for custom permission sets by using the Amazon QuickSight console. Then, you use the <code>RegisterUser</code> API operation to assign the named set of permissions to a Amazon QuickSight user. </p>
    /// <p>Amazon QuickSight custom permissions are applied through IAM policies. Therefore, they override the permissions typically granted by assigning Amazon QuickSight users to one of the default security cohorts in Amazon QuickSight (admin, author, reader).</p>
    /// <p>This feature is available only to Amazon QuickSight Enterprise edition subscriptions.</p>
    pub fn custom_permissions_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.custom_permissions_name(input.into());
        self
    }
    /// <p>(Enterprise edition only) The name of the custom permissions profile that you want to assign to this user. Customized permissions allows you to control a user's access by restricting access the following operations:</p>
    /// <ul>
    /// <li> <p>Create and update data sources</p> </li>
    /// <li> <p>Create and update datasets</p> </li>
    /// <li> <p>Create and update email reports</p> </li>
    /// <li> <p>Subscribe to email reports</p> </li>
    /// </ul>
    /// <p>A set of custom permissions includes any combination of these restrictions. Currently, you need to create the profile names for custom permission sets by using the Amazon QuickSight console. Then, you use the <code>RegisterUser</code> API operation to assign the named set of permissions to a Amazon QuickSight user. </p>
    /// <p>Amazon QuickSight custom permissions are applied through IAM policies. Therefore, they override the permissions typically granted by assigning Amazon QuickSight users to one of the default security cohorts in Amazon QuickSight (admin, author, reader).</p>
    /// <p>This feature is available only to Amazon QuickSight Enterprise edition subscriptions.</p>
    pub fn set_custom_permissions_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_custom_permissions_name(input);
        self
    }
    /// <p>A flag that you use to indicate that you want to remove all custom permissions from this user. Using this parameter resets the user to the state it was in before a custom permissions profile was applied. This parameter defaults to NULL and it doesn't accept any other value.</p>
    pub fn unapply_custom_permissions(mut self, input: bool) -> Self {
        self.inner = self.inner.unapply_custom_permissions(input);
        self
    }
    /// <p>A flag that you use to indicate that you want to remove all custom permissions from this user. Using this parameter resets the user to the state it was in before a custom permissions profile was applied. This parameter defaults to NULL and it doesn't accept any other value.</p>
    pub fn set_unapply_custom_permissions(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_unapply_custom_permissions(input);
        self
    }
    /// <p>The type of supported external login provider that provides identity to let a user federate into Amazon QuickSight with an associated Identity and Access Management(IAM) role. The type of supported external login provider can be one of the following.</p>
    /// <ul>
    /// <li> <p> <code>COGNITO</code>: Amazon Cognito. The provider URL is cognito-identity.amazonaws.com. When choosing the <code>COGNITO</code> provider type, don’t use the "CustomFederationProviderUrl" parameter which is only needed when the external provider is custom.</p> </li>
    /// <li> <p> <code>CUSTOM_OIDC</code>: Custom OpenID Connect (OIDC) provider. When choosing <code>CUSTOM_OIDC</code> type, use the <code>CustomFederationProviderUrl</code> parameter to provide the custom OIDC provider URL.</p> </li>
    /// <li> <p> <code>NONE</code>: This clears all the previously saved external login information for a user. Use the <code> <a href="https://docs.aws.amazon.com/quicksight/latest/APIReference/API_DescribeUser.html">DescribeUser</a> </code> API operation to check the external login information.</p> </li>
    /// </ul>
    pub fn external_login_federation_provider_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self
            .inner
            .external_login_federation_provider_type(input.into());
        self
    }
    /// <p>The type of supported external login provider that provides identity to let a user federate into Amazon QuickSight with an associated Identity and Access Management(IAM) role. The type of supported external login provider can be one of the following.</p>
    /// <ul>
    /// <li> <p> <code>COGNITO</code>: Amazon Cognito. The provider URL is cognito-identity.amazonaws.com. When choosing the <code>COGNITO</code> provider type, don’t use the "CustomFederationProviderUrl" parameter which is only needed when the external provider is custom.</p> </li>
    /// <li> <p> <code>CUSTOM_OIDC</code>: Custom OpenID Connect (OIDC) provider. When choosing <code>CUSTOM_OIDC</code> type, use the <code>CustomFederationProviderUrl</code> parameter to provide the custom OIDC provider URL.</p> </li>
    /// <li> <p> <code>NONE</code>: This clears all the previously saved external login information for a user. Use the <code> <a href="https://docs.aws.amazon.com/quicksight/latest/APIReference/API_DescribeUser.html">DescribeUser</a> </code> API operation to check the external login information.</p> </li>
    /// </ul>
    pub fn set_external_login_federation_provider_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self
            .inner
            .set_external_login_federation_provider_type(input);
        self
    }
    /// <p>The URL of the custom OpenID Connect (OIDC) provider that provides identity to let a user federate into Amazon QuickSight with an associated Identity and Access Management(IAM) role. This parameter should only be used when <code>ExternalLoginFederationProviderType</code> parameter is set to <code>CUSTOM_OIDC</code>.</p>
    pub fn custom_federation_provider_url(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.custom_federation_provider_url(input.into());
        self
    }
    /// <p>The URL of the custom OpenID Connect (OIDC) provider that provides identity to let a user federate into Amazon QuickSight with an associated Identity and Access Management(IAM) role. This parameter should only be used when <code>ExternalLoginFederationProviderType</code> parameter is set to <code>CUSTOM_OIDC</code>.</p>
    pub fn set_custom_federation_provider_url(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_custom_federation_provider_url(input);
        self
    }
    /// <p>The identity ID for a user in the external login provider.</p>
    pub fn external_login_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.external_login_id(input.into());
        self
    }
    /// <p>The identity ID for a user in the external login provider.</p>
    pub fn set_external_login_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_external_login_id(input);
        self
    }
}
