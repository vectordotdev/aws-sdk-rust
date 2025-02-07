// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The policy directives and their values that CloudFront includes as values for the <code>Content-Security-Policy</code> HTTP response header.</p>
/// <p>For more information about the <code>Content-Security-Policy</code> HTTP response header, see <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Security-Policy">Content-Security-Policy</a> in the MDN Web Docs.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResponseHeadersPolicyContentSecurityPolicy {
    /// <p>A Boolean that determines whether CloudFront overrides the <code>Content-Security-Policy</code> HTTP response header received from the origin with the one specified in this response headers policy.</p>
    #[doc(hidden)]
    pub r#override: ::std::option::Option<bool>,
    /// <p>The policy directives and their values that CloudFront includes as values for the <code>Content-Security-Policy</code> HTTP response header.</p>
    #[doc(hidden)]
    pub content_security_policy: ::std::option::Option<::std::string::String>,
}
impl ResponseHeadersPolicyContentSecurityPolicy {
    /// <p>A Boolean that determines whether CloudFront overrides the <code>Content-Security-Policy</code> HTTP response header received from the origin with the one specified in this response headers policy.</p>
    pub fn r#override(&self) -> ::std::option::Option<bool> {
        self.r#override
    }
    /// <p>The policy directives and their values that CloudFront includes as values for the <code>Content-Security-Policy</code> HTTP response header.</p>
    pub fn content_security_policy(&self) -> ::std::option::Option<&str> {
        self.content_security_policy.as_deref()
    }
}
impl ResponseHeadersPolicyContentSecurityPolicy {
    /// Creates a new builder-style object to manufacture [`ResponseHeadersPolicyContentSecurityPolicy`](crate::types::ResponseHeadersPolicyContentSecurityPolicy).
    pub fn builder() -> crate::types::builders::ResponseHeadersPolicyContentSecurityPolicyBuilder {
        crate::types::builders::ResponseHeadersPolicyContentSecurityPolicyBuilder::default()
    }
}

/// A builder for [`ResponseHeadersPolicyContentSecurityPolicy`](crate::types::ResponseHeadersPolicyContentSecurityPolicy).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResponseHeadersPolicyContentSecurityPolicyBuilder {
    pub(crate) r#override: ::std::option::Option<bool>,
    pub(crate) content_security_policy: ::std::option::Option<::std::string::String>,
}
impl ResponseHeadersPolicyContentSecurityPolicyBuilder {
    /// <p>A Boolean that determines whether CloudFront overrides the <code>Content-Security-Policy</code> HTTP response header received from the origin with the one specified in this response headers policy.</p>
    pub fn r#override(mut self, input: bool) -> Self {
        self.r#override = ::std::option::Option::Some(input);
        self
    }
    /// <p>A Boolean that determines whether CloudFront overrides the <code>Content-Security-Policy</code> HTTP response header received from the origin with the one specified in this response headers policy.</p>
    pub fn set_override(mut self, input: ::std::option::Option<bool>) -> Self {
        self.r#override = input;
        self
    }
    /// <p>The policy directives and their values that CloudFront includes as values for the <code>Content-Security-Policy</code> HTTP response header.</p>
    pub fn content_security_policy(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.content_security_policy = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The policy directives and their values that CloudFront includes as values for the <code>Content-Security-Policy</code> HTTP response header.</p>
    pub fn set_content_security_policy(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.content_security_policy = input;
        self
    }
    /// Consumes the builder and constructs a [`ResponseHeadersPolicyContentSecurityPolicy`](crate::types::ResponseHeadersPolicyContentSecurityPolicy).
    pub fn build(self) -> crate::types::ResponseHeadersPolicyContentSecurityPolicy {
        crate::types::ResponseHeadersPolicyContentSecurityPolicy {
            r#override: self.r#override,
            content_security_policy: self.content_security_policy,
        }
    }
}
