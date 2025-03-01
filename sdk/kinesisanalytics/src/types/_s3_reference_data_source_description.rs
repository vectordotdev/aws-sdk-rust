// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the bucket name and object key name that stores the reference data.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct S3ReferenceDataSourceDescription {
    /// <p>Amazon Resource Name (ARN) of the S3 bucket.</p>
    #[doc(hidden)]
    pub bucket_arn: ::std::option::Option<::std::string::String>,
    /// <p>Amazon S3 object key name.</p>
    #[doc(hidden)]
    pub file_key: ::std::option::Option<::std::string::String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object on your behalf to populate the in-application reference table.</p>
    #[doc(hidden)]
    pub reference_role_arn: ::std::option::Option<::std::string::String>,
}
impl S3ReferenceDataSourceDescription {
    /// <p>Amazon Resource Name (ARN) of the S3 bucket.</p>
    pub fn bucket_arn(&self) -> ::std::option::Option<&str> {
        self.bucket_arn.as_deref()
    }
    /// <p>Amazon S3 object key name.</p>
    pub fn file_key(&self) -> ::std::option::Option<&str> {
        self.file_key.as_deref()
    }
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object on your behalf to populate the in-application reference table.</p>
    pub fn reference_role_arn(&self) -> ::std::option::Option<&str> {
        self.reference_role_arn.as_deref()
    }
}
impl S3ReferenceDataSourceDescription {
    /// Creates a new builder-style object to manufacture [`S3ReferenceDataSourceDescription`](crate::types::S3ReferenceDataSourceDescription).
    pub fn builder() -> crate::types::builders::S3ReferenceDataSourceDescriptionBuilder {
        crate::types::builders::S3ReferenceDataSourceDescriptionBuilder::default()
    }
}

/// A builder for [`S3ReferenceDataSourceDescription`](crate::types::S3ReferenceDataSourceDescription).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct S3ReferenceDataSourceDescriptionBuilder {
    pub(crate) bucket_arn: ::std::option::Option<::std::string::String>,
    pub(crate) file_key: ::std::option::Option<::std::string::String>,
    pub(crate) reference_role_arn: ::std::option::Option<::std::string::String>,
}
impl S3ReferenceDataSourceDescriptionBuilder {
    /// <p>Amazon Resource Name (ARN) of the S3 bucket.</p>
    pub fn bucket_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of the S3 bucket.</p>
    pub fn set_bucket_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket_arn = input;
        self
    }
    /// <p>Amazon S3 object key name.</p>
    pub fn file_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.file_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon S3 object key name.</p>
    pub fn set_file_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.file_key = input;
        self
    }
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object on your behalf to populate the in-application reference table.</p>
    pub fn reference_role_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.reference_role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object on your behalf to populate the in-application reference table.</p>
    pub fn set_reference_role_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.reference_role_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`S3ReferenceDataSourceDescription`](crate::types::S3ReferenceDataSourceDescription).
    pub fn build(self) -> crate::types::S3ReferenceDataSourceDescription {
        crate::types::S3ReferenceDataSourceDescription {
            bucket_arn: self.bucket_arn,
            file_key: self.file_key,
            reference_role_arn: self.reference_role_arn,
        }
    }
}
