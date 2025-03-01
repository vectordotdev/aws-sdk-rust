// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The mapping between a Amazon Web Services Region and the key that's used to encrypt the data.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RegionMapInputValue {
    /// <p>The KMS key used to encrypt the data in your replication set.</p>
    #[doc(hidden)]
    pub sse_kms_key_id: ::std::option::Option<::std::string::String>,
}
impl RegionMapInputValue {
    /// <p>The KMS key used to encrypt the data in your replication set.</p>
    pub fn sse_kms_key_id(&self) -> ::std::option::Option<&str> {
        self.sse_kms_key_id.as_deref()
    }
}
impl RegionMapInputValue {
    /// Creates a new builder-style object to manufacture [`RegionMapInputValue`](crate::types::RegionMapInputValue).
    pub fn builder() -> crate::types::builders::RegionMapInputValueBuilder {
        crate::types::builders::RegionMapInputValueBuilder::default()
    }
}

/// A builder for [`RegionMapInputValue`](crate::types::RegionMapInputValue).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RegionMapInputValueBuilder {
    pub(crate) sse_kms_key_id: ::std::option::Option<::std::string::String>,
}
impl RegionMapInputValueBuilder {
    /// <p>The KMS key used to encrypt the data in your replication set.</p>
    pub fn sse_kms_key_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.sse_kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The KMS key used to encrypt the data in your replication set.</p>
    pub fn set_sse_kms_key_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.sse_kms_key_id = input;
        self
    }
    /// Consumes the builder and constructs a [`RegionMapInputValue`](crate::types::RegionMapInputValue).
    pub fn build(self) -> crate::types::RegionMapInputValue {
        crate::types::RegionMapInputValue {
            sse_kms_key_id: self.sse_kms_key_id,
        }
    }
}
