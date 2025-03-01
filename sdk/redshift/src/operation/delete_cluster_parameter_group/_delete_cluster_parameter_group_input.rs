// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteClusterParameterGroupInput {
    /// <p>The name of the parameter group to be deleted.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must be the name of an existing cluster parameter group.</p> </li>
    /// <li> <p>Cannot delete a default cluster parameter group.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub parameter_group_name: ::std::option::Option<::std::string::String>,
}
impl DeleteClusterParameterGroupInput {
    /// <p>The name of the parameter group to be deleted.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must be the name of an existing cluster parameter group.</p> </li>
    /// <li> <p>Cannot delete a default cluster parameter group.</p> </li>
    /// </ul>
    pub fn parameter_group_name(&self) -> ::std::option::Option<&str> {
        self.parameter_group_name.as_deref()
    }
}
impl DeleteClusterParameterGroupInput {
    /// Creates a new builder-style object to manufacture [`DeleteClusterParameterGroupInput`](crate::operation::delete_cluster_parameter_group::DeleteClusterParameterGroupInput).
    pub fn builder() -> crate::operation::delete_cluster_parameter_group::builders::DeleteClusterParameterGroupInputBuilder{
        crate::operation::delete_cluster_parameter_group::builders::DeleteClusterParameterGroupInputBuilder::default()
    }
}

/// A builder for [`DeleteClusterParameterGroupInput`](crate::operation::delete_cluster_parameter_group::DeleteClusterParameterGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteClusterParameterGroupInputBuilder {
    pub(crate) parameter_group_name: ::std::option::Option<::std::string::String>,
}
impl DeleteClusterParameterGroupInputBuilder {
    /// <p>The name of the parameter group to be deleted.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must be the name of an existing cluster parameter group.</p> </li>
    /// <li> <p>Cannot delete a default cluster parameter group.</p> </li>
    /// </ul>
    pub fn parameter_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.parameter_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the parameter group to be deleted.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must be the name of an existing cluster parameter group.</p> </li>
    /// <li> <p>Cannot delete a default cluster parameter group.</p> </li>
    /// </ul>
    pub fn set_parameter_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.parameter_group_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteClusterParameterGroupInput`](crate::operation::delete_cluster_parameter_group::DeleteClusterParameterGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_cluster_parameter_group::DeleteClusterParameterGroupInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_cluster_parameter_group::DeleteClusterParameterGroupInput {
                parameter_group_name: self.parameter_group_name,
            },
        )
    }
}
