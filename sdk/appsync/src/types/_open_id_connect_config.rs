// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an OpenID Connect (OIDC) configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OpenIdConnectConfig {
    /// <p>The issuer for the OIDC configuration. The issuer returned by discovery must exactly match the value of <code>iss</code> in the ID token.</p>
    #[doc(hidden)]
    pub issuer: ::std::option::Option<::std::string::String>,
    /// <p>The client identifier of the relying party at the OpenID identity provider. This identifier is typically obtained when the relying party is registered with the OpenID identity provider. You can specify a regular expression so that AppSync can validate against multiple client identifiers at a time.</p>
    #[doc(hidden)]
    pub client_id: ::std::option::Option<::std::string::String>,
    /// <p>The number of milliseconds that a token is valid after it's issued to a user.</p>
    #[doc(hidden)]
    pub iat_ttl: i64,
    /// <p>The number of milliseconds that a token is valid after being authenticated.</p>
    #[doc(hidden)]
    pub auth_ttl: i64,
}
impl OpenIdConnectConfig {
    /// <p>The issuer for the OIDC configuration. The issuer returned by discovery must exactly match the value of <code>iss</code> in the ID token.</p>
    pub fn issuer(&self) -> ::std::option::Option<&str> {
        self.issuer.as_deref()
    }
    /// <p>The client identifier of the relying party at the OpenID identity provider. This identifier is typically obtained when the relying party is registered with the OpenID identity provider. You can specify a regular expression so that AppSync can validate against multiple client identifiers at a time.</p>
    pub fn client_id(&self) -> ::std::option::Option<&str> {
        self.client_id.as_deref()
    }
    /// <p>The number of milliseconds that a token is valid after it's issued to a user.</p>
    pub fn iat_ttl(&self) -> i64 {
        self.iat_ttl
    }
    /// <p>The number of milliseconds that a token is valid after being authenticated.</p>
    pub fn auth_ttl(&self) -> i64 {
        self.auth_ttl
    }
}
impl OpenIdConnectConfig {
    /// Creates a new builder-style object to manufacture [`OpenIdConnectConfig`](crate::types::OpenIdConnectConfig).
    pub fn builder() -> crate::types::builders::OpenIdConnectConfigBuilder {
        crate::types::builders::OpenIdConnectConfigBuilder::default()
    }
}

/// A builder for [`OpenIdConnectConfig`](crate::types::OpenIdConnectConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OpenIdConnectConfigBuilder {
    pub(crate) issuer: ::std::option::Option<::std::string::String>,
    pub(crate) client_id: ::std::option::Option<::std::string::String>,
    pub(crate) iat_ttl: ::std::option::Option<i64>,
    pub(crate) auth_ttl: ::std::option::Option<i64>,
}
impl OpenIdConnectConfigBuilder {
    /// <p>The issuer for the OIDC configuration. The issuer returned by discovery must exactly match the value of <code>iss</code> in the ID token.</p>
    pub fn issuer(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.issuer = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The issuer for the OIDC configuration. The issuer returned by discovery must exactly match the value of <code>iss</code> in the ID token.</p>
    pub fn set_issuer(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.issuer = input;
        self
    }
    /// <p>The client identifier of the relying party at the OpenID identity provider. This identifier is typically obtained when the relying party is registered with the OpenID identity provider. You can specify a regular expression so that AppSync can validate against multiple client identifiers at a time.</p>
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The client identifier of the relying party at the OpenID identity provider. This identifier is typically obtained when the relying party is registered with the OpenID identity provider. You can specify a regular expression so that AppSync can validate against multiple client identifiers at a time.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_id = input;
        self
    }
    /// <p>The number of milliseconds that a token is valid after it's issued to a user.</p>
    pub fn iat_ttl(mut self, input: i64) -> Self {
        self.iat_ttl = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of milliseconds that a token is valid after it's issued to a user.</p>
    pub fn set_iat_ttl(mut self, input: ::std::option::Option<i64>) -> Self {
        self.iat_ttl = input;
        self
    }
    /// <p>The number of milliseconds that a token is valid after being authenticated.</p>
    pub fn auth_ttl(mut self, input: i64) -> Self {
        self.auth_ttl = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of milliseconds that a token is valid after being authenticated.</p>
    pub fn set_auth_ttl(mut self, input: ::std::option::Option<i64>) -> Self {
        self.auth_ttl = input;
        self
    }
    /// Consumes the builder and constructs a [`OpenIdConnectConfig`](crate::types::OpenIdConnectConfig).
    pub fn build(self) -> crate::types::OpenIdConnectConfig {
        crate::types::OpenIdConnectConfig {
            issuer: self.issuer,
            client_id: self.client_id,
            iat_ttl: self.iat_ttl.unwrap_or_default(),
            auth_ttl: self.auth_ttl.unwrap_or_default(),
        }
    }
}
