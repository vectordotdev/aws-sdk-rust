// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that contains configuration for Bring Your Own DKIM (BYODKIM), or, for Easy DKIM</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct DkimSigningAttributes {
    /// <p>[Bring Your Own DKIM] A string that's used to identify a public key in the DNS configuration for a domain.</p>
    #[doc(hidden)]
    pub domain_signing_selector: ::std::option::Option<::std::string::String>,
    /// <p>[Bring Your Own DKIM] A private key that's used to generate a DKIM signature.</p>
    /// <p>The private key must use 1024 or 2048-bit RSA encryption, and must be encoded using base64 encoding.</p>
    #[doc(hidden)]
    pub domain_signing_private_key: ::std::option::Option<::std::string::String>,
    /// <p>[Easy DKIM] The key length of the future DKIM key pair to be generated. This can be changed at most once per day.</p>
    #[doc(hidden)]
    pub next_signing_key_length: ::std::option::Option<crate::types::DkimSigningKeyLength>,
}
impl DkimSigningAttributes {
    /// <p>[Bring Your Own DKIM] A string that's used to identify a public key in the DNS configuration for a domain.</p>
    pub fn domain_signing_selector(&self) -> ::std::option::Option<&str> {
        self.domain_signing_selector.as_deref()
    }
    /// <p>[Bring Your Own DKIM] A private key that's used to generate a DKIM signature.</p>
    /// <p>The private key must use 1024 or 2048-bit RSA encryption, and must be encoded using base64 encoding.</p>
    pub fn domain_signing_private_key(&self) -> ::std::option::Option<&str> {
        self.domain_signing_private_key.as_deref()
    }
    /// <p>[Easy DKIM] The key length of the future DKIM key pair to be generated. This can be changed at most once per day.</p>
    pub fn next_signing_key_length(
        &self,
    ) -> ::std::option::Option<&crate::types::DkimSigningKeyLength> {
        self.next_signing_key_length.as_ref()
    }
}
impl ::std::fmt::Debug for DkimSigningAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DkimSigningAttributes");
        formatter.field("domain_signing_selector", &self.domain_signing_selector);
        formatter.field(
            "domain_signing_private_key",
            &"*** Sensitive Data Redacted ***",
        );
        formatter.field("next_signing_key_length", &self.next_signing_key_length);
        formatter.finish()
    }
}
impl DkimSigningAttributes {
    /// Creates a new builder-style object to manufacture [`DkimSigningAttributes`](crate::types::DkimSigningAttributes).
    pub fn builder() -> crate::types::builders::DkimSigningAttributesBuilder {
        crate::types::builders::DkimSigningAttributesBuilder::default()
    }
}

/// A builder for [`DkimSigningAttributes`](crate::types::DkimSigningAttributes).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct DkimSigningAttributesBuilder {
    pub(crate) domain_signing_selector: ::std::option::Option<::std::string::String>,
    pub(crate) domain_signing_private_key: ::std::option::Option<::std::string::String>,
    pub(crate) next_signing_key_length: ::std::option::Option<crate::types::DkimSigningKeyLength>,
}
impl DkimSigningAttributesBuilder {
    /// <p>[Bring Your Own DKIM] A string that's used to identify a public key in the DNS configuration for a domain.</p>
    pub fn domain_signing_selector(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.domain_signing_selector = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>[Bring Your Own DKIM] A string that's used to identify a public key in the DNS configuration for a domain.</p>
    pub fn set_domain_signing_selector(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.domain_signing_selector = input;
        self
    }
    /// <p>[Bring Your Own DKIM] A private key that's used to generate a DKIM signature.</p>
    /// <p>The private key must use 1024 or 2048-bit RSA encryption, and must be encoded using base64 encoding.</p>
    pub fn domain_signing_private_key(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.domain_signing_private_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>[Bring Your Own DKIM] A private key that's used to generate a DKIM signature.</p>
    /// <p>The private key must use 1024 or 2048-bit RSA encryption, and must be encoded using base64 encoding.</p>
    pub fn set_domain_signing_private_key(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.domain_signing_private_key = input;
        self
    }
    /// <p>[Easy DKIM] The key length of the future DKIM key pair to be generated. This can be changed at most once per day.</p>
    pub fn next_signing_key_length(mut self, input: crate::types::DkimSigningKeyLength) -> Self {
        self.next_signing_key_length = ::std::option::Option::Some(input);
        self
    }
    /// <p>[Easy DKIM] The key length of the future DKIM key pair to be generated. This can be changed at most once per day.</p>
    pub fn set_next_signing_key_length(
        mut self,
        input: ::std::option::Option<crate::types::DkimSigningKeyLength>,
    ) -> Self {
        self.next_signing_key_length = input;
        self
    }
    /// Consumes the builder and constructs a [`DkimSigningAttributes`](crate::types::DkimSigningAttributes).
    pub fn build(self) -> crate::types::DkimSigningAttributes {
        crate::types::DkimSigningAttributes {
            domain_signing_selector: self.domain_signing_selector,
            domain_signing_private_key: self.domain_signing_private_key,
            next_signing_key_length: self.next_signing_key_length,
        }
    }
}
impl ::std::fmt::Debug for DkimSigningAttributesBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DkimSigningAttributesBuilder");
        formatter.field("domain_signing_selector", &self.domain_signing_selector);
        formatter.field(
            "domain_signing_private_key",
            &"*** Sensitive Data Redacted ***",
        );
        formatter.field("next_signing_key_length", &self.next_signing_key_length);
        formatter.finish()
    }
}
