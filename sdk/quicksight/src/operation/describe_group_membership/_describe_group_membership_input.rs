// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeGroupMembershipInput {
    /// <p>The user name of the user that you want to search for.</p>
    #[doc(hidden)]
    pub member_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the group that you want to search.</p>
    #[doc(hidden)]
    pub group_name: ::std::option::Option<::std::string::String>,
    /// <p>The ID for the Amazon Web Services account that the group is in. Currently, you use the ID for the Amazon Web Services account that contains your Amazon QuickSight account.</p>
    #[doc(hidden)]
    pub aws_account_id: ::std::option::Option<::std::string::String>,
    /// <p>The namespace that includes the group you are searching within.</p>
    #[doc(hidden)]
    pub namespace: ::std::option::Option<::std::string::String>,
}
impl DescribeGroupMembershipInput {
    /// <p>The user name of the user that you want to search for.</p>
    pub fn member_name(&self) -> ::std::option::Option<&str> {
        self.member_name.as_deref()
    }
    /// <p>The name of the group that you want to search.</p>
    pub fn group_name(&self) -> ::std::option::Option<&str> {
        self.group_name.as_deref()
    }
    /// <p>The ID for the Amazon Web Services account that the group is in. Currently, you use the ID for the Amazon Web Services account that contains your Amazon QuickSight account.</p>
    pub fn aws_account_id(&self) -> ::std::option::Option<&str> {
        self.aws_account_id.as_deref()
    }
    /// <p>The namespace that includes the group you are searching within.</p>
    pub fn namespace(&self) -> ::std::option::Option<&str> {
        self.namespace.as_deref()
    }
}
impl DescribeGroupMembershipInput {
    /// Creates a new builder-style object to manufacture [`DescribeGroupMembershipInput`](crate::operation::describe_group_membership::DescribeGroupMembershipInput).
    pub fn builder(
    ) -> crate::operation::describe_group_membership::builders::DescribeGroupMembershipInputBuilder
    {
        crate::operation::describe_group_membership::builders::DescribeGroupMembershipInputBuilder::default()
    }
}

/// A builder for [`DescribeGroupMembershipInput`](crate::operation::describe_group_membership::DescribeGroupMembershipInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeGroupMembershipInputBuilder {
    pub(crate) member_name: ::std::option::Option<::std::string::String>,
    pub(crate) group_name: ::std::option::Option<::std::string::String>,
    pub(crate) aws_account_id: ::std::option::Option<::std::string::String>,
    pub(crate) namespace: ::std::option::Option<::std::string::String>,
}
impl DescribeGroupMembershipInputBuilder {
    /// <p>The user name of the user that you want to search for.</p>
    pub fn member_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.member_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user name of the user that you want to search for.</p>
    pub fn set_member_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.member_name = input;
        self
    }
    /// <p>The name of the group that you want to search.</p>
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the group that you want to search.</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// <p>The ID for the Amazon Web Services account that the group is in. Currently, you use the ID for the Amazon Web Services account that contains your Amazon QuickSight account.</p>
    pub fn aws_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.aws_account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID for the Amazon Web Services account that the group is in. Currently, you use the ID for the Amazon Web Services account that contains your Amazon QuickSight account.</p>
    pub fn set_aws_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.aws_account_id = input;
        self
    }
    /// <p>The namespace that includes the group you are searching within.</p>
    pub fn namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.namespace = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The namespace that includes the group you are searching within.</p>
    pub fn set_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.namespace = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeGroupMembershipInput`](crate::operation::describe_group_membership::DescribeGroupMembershipInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_group_membership::DescribeGroupMembershipInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_group_membership::DescribeGroupMembershipInput {
                member_name: self.member_name,
                group_name: self.group_name,
                aws_account_id: self.aws_account_id,
                namespace: self.namespace,
            },
        )
    }
}
