// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeIpRestrictionInput {
    /// <p>The ID of the Amazon Web Services account that contains the IP rules.</p>
    #[doc(hidden)]
    pub aws_account_id: ::std::option::Option<::std::string::String>,
}
impl DescribeIpRestrictionInput {
    /// <p>The ID of the Amazon Web Services account that contains the IP rules.</p>
    pub fn aws_account_id(&self) -> ::std::option::Option<&str> {
        self.aws_account_id.as_deref()
    }
}
impl DescribeIpRestrictionInput {
    /// Creates a new builder-style object to manufacture [`DescribeIpRestrictionInput`](crate::operation::describe_ip_restriction::DescribeIpRestrictionInput).
    pub fn builder(
    ) -> crate::operation::describe_ip_restriction::builders::DescribeIpRestrictionInputBuilder
    {
        crate::operation::describe_ip_restriction::builders::DescribeIpRestrictionInputBuilder::default()
    }
}

/// A builder for [`DescribeIpRestrictionInput`](crate::operation::describe_ip_restriction::DescribeIpRestrictionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeIpRestrictionInputBuilder {
    pub(crate) aws_account_id: ::std::option::Option<::std::string::String>,
}
impl DescribeIpRestrictionInputBuilder {
    /// <p>The ID of the Amazon Web Services account that contains the IP rules.</p>
    pub fn aws_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.aws_account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the IP rules.</p>
    pub fn set_aws_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.aws_account_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeIpRestrictionInput`](crate::operation::describe_ip_restriction::DescribeIpRestrictionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_ip_restriction::DescribeIpRestrictionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_ip_restriction::DescribeIpRestrictionInput {
                aws_account_id: self.aws_account_id,
            },
        )
    }
}
