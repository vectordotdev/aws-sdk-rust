// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Defines the resource configuration when updating an authentication resource in your Amplify project.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateBackendAuthResourceConfig {
    /// <p>Defines the service name to use when configuring an authentication resource in your Amplify project.</p>
    #[doc(hidden)]
    pub auth_resources: ::std::option::Option<crate::types::AuthResources>,
    /// <p>Describes the authorization configuration for the Amazon Cognito identity pool, provisioned as a part of your auth resource in the Amplify project.</p>
    #[doc(hidden)]
    pub identity_pool_configs:
        ::std::option::Option<crate::types::UpdateBackendAuthIdentityPoolConfig>,
    /// <p>Defines the service name to use when configuring an authentication resource in your Amplify project.</p>
    #[doc(hidden)]
    pub service: ::std::option::Option<crate::types::Service>,
    /// <p>Describes the authentication configuration for the Amazon Cognito user pool, provisioned as a part of your auth resource in the Amplify project.</p>
    #[doc(hidden)]
    pub user_pool_configs: ::std::option::Option<crate::types::UpdateBackendAuthUserPoolConfig>,
}
impl UpdateBackendAuthResourceConfig {
    /// <p>Defines the service name to use when configuring an authentication resource in your Amplify project.</p>
    pub fn auth_resources(&self) -> ::std::option::Option<&crate::types::AuthResources> {
        self.auth_resources.as_ref()
    }
    /// <p>Describes the authorization configuration for the Amazon Cognito identity pool, provisioned as a part of your auth resource in the Amplify project.</p>
    pub fn identity_pool_configs(
        &self,
    ) -> ::std::option::Option<&crate::types::UpdateBackendAuthIdentityPoolConfig> {
        self.identity_pool_configs.as_ref()
    }
    /// <p>Defines the service name to use when configuring an authentication resource in your Amplify project.</p>
    pub fn service(&self) -> ::std::option::Option<&crate::types::Service> {
        self.service.as_ref()
    }
    /// <p>Describes the authentication configuration for the Amazon Cognito user pool, provisioned as a part of your auth resource in the Amplify project.</p>
    pub fn user_pool_configs(
        &self,
    ) -> ::std::option::Option<&crate::types::UpdateBackendAuthUserPoolConfig> {
        self.user_pool_configs.as_ref()
    }
}
impl UpdateBackendAuthResourceConfig {
    /// Creates a new builder-style object to manufacture [`UpdateBackendAuthResourceConfig`](crate::types::UpdateBackendAuthResourceConfig).
    pub fn builder() -> crate::types::builders::UpdateBackendAuthResourceConfigBuilder {
        crate::types::builders::UpdateBackendAuthResourceConfigBuilder::default()
    }
}

/// A builder for [`UpdateBackendAuthResourceConfig`](crate::types::UpdateBackendAuthResourceConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateBackendAuthResourceConfigBuilder {
    pub(crate) auth_resources: ::std::option::Option<crate::types::AuthResources>,
    pub(crate) identity_pool_configs:
        ::std::option::Option<crate::types::UpdateBackendAuthIdentityPoolConfig>,
    pub(crate) service: ::std::option::Option<crate::types::Service>,
    pub(crate) user_pool_configs:
        ::std::option::Option<crate::types::UpdateBackendAuthUserPoolConfig>,
}
impl UpdateBackendAuthResourceConfigBuilder {
    /// <p>Defines the service name to use when configuring an authentication resource in your Amplify project.</p>
    pub fn auth_resources(mut self, input: crate::types::AuthResources) -> Self {
        self.auth_resources = ::std::option::Option::Some(input);
        self
    }
    /// <p>Defines the service name to use when configuring an authentication resource in your Amplify project.</p>
    pub fn set_auth_resources(
        mut self,
        input: ::std::option::Option<crate::types::AuthResources>,
    ) -> Self {
        self.auth_resources = input;
        self
    }
    /// <p>Describes the authorization configuration for the Amazon Cognito identity pool, provisioned as a part of your auth resource in the Amplify project.</p>
    pub fn identity_pool_configs(
        mut self,
        input: crate::types::UpdateBackendAuthIdentityPoolConfig,
    ) -> Self {
        self.identity_pool_configs = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the authorization configuration for the Amazon Cognito identity pool, provisioned as a part of your auth resource in the Amplify project.</p>
    pub fn set_identity_pool_configs(
        mut self,
        input: ::std::option::Option<crate::types::UpdateBackendAuthIdentityPoolConfig>,
    ) -> Self {
        self.identity_pool_configs = input;
        self
    }
    /// <p>Defines the service name to use when configuring an authentication resource in your Amplify project.</p>
    pub fn service(mut self, input: crate::types::Service) -> Self {
        self.service = ::std::option::Option::Some(input);
        self
    }
    /// <p>Defines the service name to use when configuring an authentication resource in your Amplify project.</p>
    pub fn set_service(mut self, input: ::std::option::Option<crate::types::Service>) -> Self {
        self.service = input;
        self
    }
    /// <p>Describes the authentication configuration for the Amazon Cognito user pool, provisioned as a part of your auth resource in the Amplify project.</p>
    pub fn user_pool_configs(
        mut self,
        input: crate::types::UpdateBackendAuthUserPoolConfig,
    ) -> Self {
        self.user_pool_configs = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the authentication configuration for the Amazon Cognito user pool, provisioned as a part of your auth resource in the Amplify project.</p>
    pub fn set_user_pool_configs(
        mut self,
        input: ::std::option::Option<crate::types::UpdateBackendAuthUserPoolConfig>,
    ) -> Self {
        self.user_pool_configs = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateBackendAuthResourceConfig`](crate::types::UpdateBackendAuthResourceConfig).
    pub fn build(self) -> crate::types::UpdateBackendAuthResourceConfig {
        crate::types::UpdateBackendAuthResourceConfig {
            auth_resources: self.auth_resources,
            identity_pool_configs: self.identity_pool_configs,
            service: self.service,
            user_pool_configs: self.user_pool_configs,
        }
    }
}
