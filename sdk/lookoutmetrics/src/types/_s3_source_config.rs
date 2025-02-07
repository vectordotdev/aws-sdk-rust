// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the configuration of the S3 bucket that contains source files.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct S3SourceConfig {
    /// <p>The ARN of an IAM role that has read and write access permissions to the source S3 bucket.</p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>A list of templated paths to the source files.</p>
    #[doc(hidden)]
    pub templated_path_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A list of paths to the historical data files.</p>
    #[doc(hidden)]
    pub historical_data_path_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Contains information about a source file's formatting.</p>
    #[doc(hidden)]
    pub file_format_descriptor: ::std::option::Option<crate::types::FileFormatDescriptor>,
}
impl S3SourceConfig {
    /// <p>The ARN of an IAM role that has read and write access permissions to the source S3 bucket.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>A list of templated paths to the source files.</p>
    pub fn templated_path_list(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.templated_path_list.as_deref()
    }
    /// <p>A list of paths to the historical data files.</p>
    pub fn historical_data_path_list(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.historical_data_path_list.as_deref()
    }
    /// <p>Contains information about a source file's formatting.</p>
    pub fn file_format_descriptor(
        &self,
    ) -> ::std::option::Option<&crate::types::FileFormatDescriptor> {
        self.file_format_descriptor.as_ref()
    }
}
impl S3SourceConfig {
    /// Creates a new builder-style object to manufacture [`S3SourceConfig`](crate::types::S3SourceConfig).
    pub fn builder() -> crate::types::builders::S3SourceConfigBuilder {
        crate::types::builders::S3SourceConfigBuilder::default()
    }
}

/// A builder for [`S3SourceConfig`](crate::types::S3SourceConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct S3SourceConfigBuilder {
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) templated_path_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) historical_data_path_list:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) file_format_descriptor: ::std::option::Option<crate::types::FileFormatDescriptor>,
}
impl S3SourceConfigBuilder {
    /// <p>The ARN of an IAM role that has read and write access permissions to the source S3 bucket.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of an IAM role that has read and write access permissions to the source S3 bucket.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// Appends an item to `templated_path_list`.
    ///
    /// To override the contents of this collection use [`set_templated_path_list`](Self::set_templated_path_list).
    ///
    /// <p>A list of templated paths to the source files.</p>
    pub fn templated_path_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.templated_path_list.unwrap_or_default();
        v.push(input.into());
        self.templated_path_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of templated paths to the source files.</p>
    pub fn set_templated_path_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.templated_path_list = input;
        self
    }
    /// Appends an item to `historical_data_path_list`.
    ///
    /// To override the contents of this collection use [`set_historical_data_path_list`](Self::set_historical_data_path_list).
    ///
    /// <p>A list of paths to the historical data files.</p>
    pub fn historical_data_path_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.historical_data_path_list.unwrap_or_default();
        v.push(input.into());
        self.historical_data_path_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of paths to the historical data files.</p>
    pub fn set_historical_data_path_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.historical_data_path_list = input;
        self
    }
    /// <p>Contains information about a source file's formatting.</p>
    pub fn file_format_descriptor(mut self, input: crate::types::FileFormatDescriptor) -> Self {
        self.file_format_descriptor = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains information about a source file's formatting.</p>
    pub fn set_file_format_descriptor(
        mut self,
        input: ::std::option::Option<crate::types::FileFormatDescriptor>,
    ) -> Self {
        self.file_format_descriptor = input;
        self
    }
    /// Consumes the builder and constructs a [`S3SourceConfig`](crate::types::S3SourceConfig).
    pub fn build(self) -> crate::types::S3SourceConfig {
        crate::types::S3SourceConfig {
            role_arn: self.role_arn,
            templated_path_list: self.templated_path_list,
            historical_data_path_list: self.historical_data_path_list,
            file_format_descriptor: self.file_format_descriptor,
        }
    }
}
