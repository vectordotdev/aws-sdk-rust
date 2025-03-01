// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeUpdateInput {
    /// <p>The name of the Amazon EKS cluster associated with the update.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the update to describe.</p>
    #[doc(hidden)]
    pub update_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the Amazon EKS node group associated with the update. This parameter is required if the update is a node group update.</p>
    #[doc(hidden)]
    pub nodegroup_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the add-on. The name must match one of the names returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_ListAddons.html"> <code>ListAddons</code> </a>. This parameter is required if the update is an add-on update.</p>
    #[doc(hidden)]
    pub addon_name: ::std::option::Option<::std::string::String>,
}
impl DescribeUpdateInput {
    /// <p>The name of the Amazon EKS cluster associated with the update.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The ID of the update to describe.</p>
    pub fn update_id(&self) -> ::std::option::Option<&str> {
        self.update_id.as_deref()
    }
    /// <p>The name of the Amazon EKS node group associated with the update. This parameter is required if the update is a node group update.</p>
    pub fn nodegroup_name(&self) -> ::std::option::Option<&str> {
        self.nodegroup_name.as_deref()
    }
    /// <p>The name of the add-on. The name must match one of the names returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_ListAddons.html"> <code>ListAddons</code> </a>. This parameter is required if the update is an add-on update.</p>
    pub fn addon_name(&self) -> ::std::option::Option<&str> {
        self.addon_name.as_deref()
    }
}
impl DescribeUpdateInput {
    /// Creates a new builder-style object to manufacture [`DescribeUpdateInput`](crate::operation::describe_update::DescribeUpdateInput).
    pub fn builder() -> crate::operation::describe_update::builders::DescribeUpdateInputBuilder {
        crate::operation::describe_update::builders::DescribeUpdateInputBuilder::default()
    }
}

/// A builder for [`DescribeUpdateInput`](crate::operation::describe_update::DescribeUpdateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeUpdateInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) update_id: ::std::option::Option<::std::string::String>,
    pub(crate) nodegroup_name: ::std::option::Option<::std::string::String>,
    pub(crate) addon_name: ::std::option::Option<::std::string::String>,
}
impl DescribeUpdateInputBuilder {
    /// <p>The name of the Amazon EKS cluster associated with the update.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon EKS cluster associated with the update.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The ID of the update to describe.</p>
    pub fn update_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.update_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the update to describe.</p>
    pub fn set_update_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.update_id = input;
        self
    }
    /// <p>The name of the Amazon EKS node group associated with the update. This parameter is required if the update is a node group update.</p>
    pub fn nodegroup_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.nodegroup_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon EKS node group associated with the update. This parameter is required if the update is a node group update.</p>
    pub fn set_nodegroup_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.nodegroup_name = input;
        self
    }
    /// <p>The name of the add-on. The name must match one of the names returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_ListAddons.html"> <code>ListAddons</code> </a>. This parameter is required if the update is an add-on update.</p>
    pub fn addon_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.addon_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the add-on. The name must match one of the names returned by <a href="https://docs.aws.amazon.com/eks/latest/APIReference/API_ListAddons.html"> <code>ListAddons</code> </a>. This parameter is required if the update is an add-on update.</p>
    pub fn set_addon_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.addon_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeUpdateInput`](crate::operation::describe_update::DescribeUpdateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_update::DescribeUpdateInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_update::DescribeUpdateInput {
            name: self.name,
            update_id: self.update_id,
            nodegroup_name: self.nodegroup_name,
            addon_name: self.addon_name,
        })
    }
}
