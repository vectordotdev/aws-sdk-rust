// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>List of sidewalk certificates.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CertificateList {
    /// <p>The certificate chain algorithm provided by sidewalk.</p>
    #[doc(hidden)]
    pub signing_alg: ::std::option::Option<crate::types::SigningAlg>,
    /// <p>The value of the chosen sidewalk certificate.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl CertificateList {
    /// <p>The certificate chain algorithm provided by sidewalk.</p>
    pub fn signing_alg(&self) -> ::std::option::Option<&crate::types::SigningAlg> {
        self.signing_alg.as_ref()
    }
    /// <p>The value of the chosen sidewalk certificate.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl CertificateList {
    /// Creates a new builder-style object to manufacture [`CertificateList`](crate::types::CertificateList).
    pub fn builder() -> crate::types::builders::CertificateListBuilder {
        crate::types::builders::CertificateListBuilder::default()
    }
}

/// A builder for [`CertificateList`](crate::types::CertificateList).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CertificateListBuilder {
    pub(crate) signing_alg: ::std::option::Option<crate::types::SigningAlg>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl CertificateListBuilder {
    /// <p>The certificate chain algorithm provided by sidewalk.</p>
    pub fn signing_alg(mut self, input: crate::types::SigningAlg) -> Self {
        self.signing_alg = ::std::option::Option::Some(input);
        self
    }
    /// <p>The certificate chain algorithm provided by sidewalk.</p>
    pub fn set_signing_alg(
        mut self,
        input: ::std::option::Option<crate::types::SigningAlg>,
    ) -> Self {
        self.signing_alg = input;
        self
    }
    /// <p>The value of the chosen sidewalk certificate.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value of the chosen sidewalk certificate.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`CertificateList`](crate::types::CertificateList).
    pub fn build(self) -> crate::types::CertificateList {
        crate::types::CertificateList {
            signing_alg: self.signing_alg,
            value: self.value,
        }
    }
}
