// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeVpcConnectionInput {
    /// <p>The Amazon Web Services account ID of the account that contains the VPC connection that you want described.</p>
    #[doc(hidden)]
    pub aws_account_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the VPC connection that you're creating. This ID is a unique identifier for each Amazon Web Services Region in an Amazon Web Services account.</p>
    #[doc(hidden)]
    pub vpc_connection_id: ::std::option::Option<::std::string::String>,
}
impl DescribeVpcConnectionInput {
    /// <p>The Amazon Web Services account ID of the account that contains the VPC connection that you want described.</p>
    pub fn aws_account_id(&self) -> ::std::option::Option<&str> {
        self.aws_account_id.as_deref()
    }
    /// <p>The ID of the VPC connection that you're creating. This ID is a unique identifier for each Amazon Web Services Region in an Amazon Web Services account.</p>
    pub fn vpc_connection_id(&self) -> ::std::option::Option<&str> {
        self.vpc_connection_id.as_deref()
    }
}
impl DescribeVpcConnectionInput {
    /// Creates a new builder-style object to manufacture [`DescribeVpcConnectionInput`](crate::operation::describe_vpc_connection::DescribeVpcConnectionInput).
    pub fn builder(
    ) -> crate::operation::describe_vpc_connection::builders::DescribeVpcConnectionInputBuilder
    {
        crate::operation::describe_vpc_connection::builders::DescribeVpcConnectionInputBuilder::default()
    }
}

/// A builder for [`DescribeVpcConnectionInput`](crate::operation::describe_vpc_connection::DescribeVpcConnectionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeVpcConnectionInputBuilder {
    pub(crate) aws_account_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_connection_id: ::std::option::Option<::std::string::String>,
}
impl DescribeVpcConnectionInputBuilder {
    /// <p>The Amazon Web Services account ID of the account that contains the VPC connection that you want described.</p>
    pub fn aws_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.aws_account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID of the account that contains the VPC connection that you want described.</p>
    pub fn set_aws_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.aws_account_id = input;
        self
    }
    /// <p>The ID of the VPC connection that you're creating. This ID is a unique identifier for each Amazon Web Services Region in an Amazon Web Services account.</p>
    pub fn vpc_connection_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.vpc_connection_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the VPC connection that you're creating. This ID is a unique identifier for each Amazon Web Services Region in an Amazon Web Services account.</p>
    pub fn set_vpc_connection_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.vpc_connection_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeVpcConnectionInput`](crate::operation::describe_vpc_connection::DescribeVpcConnectionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_vpc_connection::DescribeVpcConnectionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_vpc_connection::DescribeVpcConnectionInput {
                aws_account_id: self.aws_account_id,
                vpc_connection_id: self.vpc_connection_id,
            },
        )
    }
}
