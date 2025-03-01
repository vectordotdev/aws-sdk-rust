// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This input determines which instance groups to retrieve.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListInstanceGroupsInput {
    /// <p>The identifier of the cluster for which to list the instance groups.</p>
    #[doc(hidden)]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[doc(hidden)]
    pub marker: ::std::option::Option<::std::string::String>,
}
impl ListInstanceGroupsInput {
    /// <p>The identifier of the cluster for which to list the instance groups.</p>
    pub fn cluster_id(&self) -> ::std::option::Option<&str> {
        self.cluster_id.as_deref()
    }
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    pub fn marker(&self) -> ::std::option::Option<&str> {
        self.marker.as_deref()
    }
}
impl ListInstanceGroupsInput {
    /// Creates a new builder-style object to manufacture [`ListInstanceGroupsInput`](crate::operation::list_instance_groups::ListInstanceGroupsInput).
    pub fn builder(
    ) -> crate::operation::list_instance_groups::builders::ListInstanceGroupsInputBuilder {
        crate::operation::list_instance_groups::builders::ListInstanceGroupsInputBuilder::default()
    }
}

/// A builder for [`ListInstanceGroupsInput`](crate::operation::list_instance_groups::ListInstanceGroupsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListInstanceGroupsInputBuilder {
    pub(crate) cluster_id: ::std::option::Option<::std::string::String>,
    pub(crate) marker: ::std::option::Option<::std::string::String>,
}
impl ListInstanceGroupsInputBuilder {
    /// <p>The identifier of the cluster for which to list the instance groups.</p>
    pub fn cluster_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cluster_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the cluster for which to list the instance groups.</p>
    pub fn set_cluster_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cluster_id = input;
        self
    }
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// Consumes the builder and constructs a [`ListInstanceGroupsInput`](crate::operation::list_instance_groups::ListInstanceGroupsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_instance_groups::ListInstanceGroupsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_instance_groups::ListInstanceGroupsInput {
                cluster_id: self.cluster_id,
                marker: self.marker,
            },
        )
    }
}
