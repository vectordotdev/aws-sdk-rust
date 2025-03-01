// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the S3 location.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct S3Location {
    /// <p>The S3 bucket of the S3 object.</p>
    #[doc(hidden)]
    pub s3_bucket: ::std::option::Option<::std::string::String>,
    /// <p>The S3 key of the S3 object.</p>
    #[doc(hidden)]
    pub s3_key: ::std::option::Option<::std::string::String>,
}
impl S3Location {
    /// <p>The S3 bucket of the S3 object.</p>
    pub fn s3_bucket(&self) -> ::std::option::Option<&str> {
        self.s3_bucket.as_deref()
    }
    /// <p>The S3 key of the S3 object.</p>
    pub fn s3_key(&self) -> ::std::option::Option<&str> {
        self.s3_key.as_deref()
    }
}
impl S3Location {
    /// Creates a new builder-style object to manufacture [`S3Location`](crate::types::S3Location).
    pub fn builder() -> crate::types::builders::S3LocationBuilder {
        crate::types::builders::S3LocationBuilder::default()
    }
}

/// A builder for [`S3Location`](crate::types::S3Location).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct S3LocationBuilder {
    pub(crate) s3_bucket: ::std::option::Option<::std::string::String>,
    pub(crate) s3_key: ::std::option::Option<::std::string::String>,
}
impl S3LocationBuilder {
    /// <p>The S3 bucket of the S3 object.</p>
    pub fn s3_bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The S3 bucket of the S3 object.</p>
    pub fn set_s3_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_bucket = input;
        self
    }
    /// <p>The S3 key of the S3 object.</p>
    pub fn s3_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The S3 key of the S3 object.</p>
    pub fn set_s3_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_key = input;
        self
    }
    /// Consumes the builder and constructs a [`S3Location`](crate::types::S3Location).
    pub fn build(self) -> crate::types::S3Location {
        crate::types::S3Location {
            s3_bucket: self.s3_bucket,
            s3_key: self.s3_key,
        }
    }
}
