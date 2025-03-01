// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeFileSystemAssociationsInput {
    /// <p>An array containing the Amazon Resource Name (ARN) of each file system association to be described.</p>
    #[doc(hidden)]
    pub file_system_association_arn_list:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DescribeFileSystemAssociationsInput {
    /// <p>An array containing the Amazon Resource Name (ARN) of each file system association to be described.</p>
    pub fn file_system_association_arn_list(
        &self,
    ) -> ::std::option::Option<&[::std::string::String]> {
        self.file_system_association_arn_list.as_deref()
    }
}
impl DescribeFileSystemAssociationsInput {
    /// Creates a new builder-style object to manufacture [`DescribeFileSystemAssociationsInput`](crate::operation::describe_file_system_associations::DescribeFileSystemAssociationsInput).
    pub fn builder() -> crate::operation::describe_file_system_associations::builders::DescribeFileSystemAssociationsInputBuilder{
        crate::operation::describe_file_system_associations::builders::DescribeFileSystemAssociationsInputBuilder::default()
    }
}

/// A builder for [`DescribeFileSystemAssociationsInput`](crate::operation::describe_file_system_associations::DescribeFileSystemAssociationsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeFileSystemAssociationsInputBuilder {
    pub(crate) file_system_association_arn_list:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DescribeFileSystemAssociationsInputBuilder {
    /// Appends an item to `file_system_association_arn_list`.
    ///
    /// To override the contents of this collection use [`set_file_system_association_arn_list`](Self::set_file_system_association_arn_list).
    ///
    /// <p>An array containing the Amazon Resource Name (ARN) of each file system association to be described.</p>
    pub fn file_system_association_arn_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.file_system_association_arn_list.unwrap_or_default();
        v.push(input.into());
        self.file_system_association_arn_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array containing the Amazon Resource Name (ARN) of each file system association to be described.</p>
    pub fn set_file_system_association_arn_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.file_system_association_arn_list = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeFileSystemAssociationsInput`](crate::operation::describe_file_system_associations::DescribeFileSystemAssociationsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_file_system_associations::DescribeFileSystemAssociationsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_file_system_associations::DescribeFileSystemAssociationsInput {
                file_system_association_arn_list: self.file_system_association_arn_list
                ,
            }
        )
    }
}
