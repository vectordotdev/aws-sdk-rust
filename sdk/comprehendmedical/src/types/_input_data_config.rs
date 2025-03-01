// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The input properties for an entities detection job. This includes the name of the S3 bucket and the path to the files to be analyzed. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InputDataConfig {
    /// <p>The URI of the S3 bucket that contains the input data. The bucket must be in the same region as the API endpoint that you are calling.</p>
    /// <p>Each file in the document collection must be less than 40 KB. You can store a maximum of 30 GB in the bucket.</p>
    #[doc(hidden)]
    pub s3_bucket: ::std::option::Option<::std::string::String>,
    /// <p>The path to the input data files in the S3 bucket.</p>
    #[doc(hidden)]
    pub s3_key: ::std::option::Option<::std::string::String>,
}
impl InputDataConfig {
    /// <p>The URI of the S3 bucket that contains the input data. The bucket must be in the same region as the API endpoint that you are calling.</p>
    /// <p>Each file in the document collection must be less than 40 KB. You can store a maximum of 30 GB in the bucket.</p>
    pub fn s3_bucket(&self) -> ::std::option::Option<&str> {
        self.s3_bucket.as_deref()
    }
    /// <p>The path to the input data files in the S3 bucket.</p>
    pub fn s3_key(&self) -> ::std::option::Option<&str> {
        self.s3_key.as_deref()
    }
}
impl InputDataConfig {
    /// Creates a new builder-style object to manufacture [`InputDataConfig`](crate::types::InputDataConfig).
    pub fn builder() -> crate::types::builders::InputDataConfigBuilder {
        crate::types::builders::InputDataConfigBuilder::default()
    }
}

/// A builder for [`InputDataConfig`](crate::types::InputDataConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InputDataConfigBuilder {
    pub(crate) s3_bucket: ::std::option::Option<::std::string::String>,
    pub(crate) s3_key: ::std::option::Option<::std::string::String>,
}
impl InputDataConfigBuilder {
    /// <p>The URI of the S3 bucket that contains the input data. The bucket must be in the same region as the API endpoint that you are calling.</p>
    /// <p>Each file in the document collection must be less than 40 KB. You can store a maximum of 30 GB in the bucket.</p>
    pub fn s3_bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URI of the S3 bucket that contains the input data. The bucket must be in the same region as the API endpoint that you are calling.</p>
    /// <p>Each file in the document collection must be less than 40 KB. You can store a maximum of 30 GB in the bucket.</p>
    pub fn set_s3_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_bucket = input;
        self
    }
    /// <p>The path to the input data files in the S3 bucket.</p>
    pub fn s3_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The path to the input data files in the S3 bucket.</p>
    pub fn set_s3_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_key = input;
        self
    }
    /// Consumes the builder and constructs a [`InputDataConfig`](crate::types::InputDataConfig).
    pub fn build(self) -> crate::types::InputDataConfig {
        crate::types::InputDataConfig {
            s3_bucket: self.s3_bucket,
            s3_key: self.s3_key,
        }
    }
}
