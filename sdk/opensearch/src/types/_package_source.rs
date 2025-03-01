// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Amazon S3 location to import the package from.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PackageSource {
    /// <p>The name of the Amazon S3 bucket containing the package.</p>
    #[doc(hidden)]
    pub s3_bucket_name: ::std::option::Option<::std::string::String>,
    /// <p>Key (file name) of the package.</p>
    #[doc(hidden)]
    pub s3_key: ::std::option::Option<::std::string::String>,
}
impl PackageSource {
    /// <p>The name of the Amazon S3 bucket containing the package.</p>
    pub fn s3_bucket_name(&self) -> ::std::option::Option<&str> {
        self.s3_bucket_name.as_deref()
    }
    /// <p>Key (file name) of the package.</p>
    pub fn s3_key(&self) -> ::std::option::Option<&str> {
        self.s3_key.as_deref()
    }
}
impl PackageSource {
    /// Creates a new builder-style object to manufacture [`PackageSource`](crate::types::PackageSource).
    pub fn builder() -> crate::types::builders::PackageSourceBuilder {
        crate::types::builders::PackageSourceBuilder::default()
    }
}

/// A builder for [`PackageSource`](crate::types::PackageSource).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PackageSourceBuilder {
    pub(crate) s3_bucket_name: ::std::option::Option<::std::string::String>,
    pub(crate) s3_key: ::std::option::Option<::std::string::String>,
}
impl PackageSourceBuilder {
    /// <p>The name of the Amazon S3 bucket containing the package.</p>
    pub fn s3_bucket_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.s3_bucket_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket containing the package.</p>
    pub fn set_s3_bucket_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.s3_bucket_name = input;
        self
    }
    /// <p>Key (file name) of the package.</p>
    pub fn s3_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Key (file name) of the package.</p>
    pub fn set_s3_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_key = input;
        self
    }
    /// Consumes the builder and constructs a [`PackageSource`](crate::types::PackageSource).
    pub fn build(self) -> crate::types::PackageSource {
        crate::types::PackageSource {
            s3_bucket_name: self.s3_bucket_name,
            s3_key: self.s3_key,
        }
    }
}
