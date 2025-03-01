// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FailoverGlobalClusterInput {
    /// <p>Identifier of the Aurora global database (<code>GlobalCluster</code>) that should be failed over. The identifier is the unique key assigned by the user when the Aurora global database was created. In other words, it's the name of the Aurora global database that you want to fail over.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the identifier of an existing <code>GlobalCluster</code> (Aurora global database).</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub global_cluster_identifier: ::std::option::Option<::std::string::String>,
    /// <p>Identifier of the secondary Aurora DB cluster that you want to promote to primary for the Aurora global database (<code>GlobalCluster</code>.) Use the Amazon Resource Name (ARN) for the identifier so that Aurora can locate the cluster in its Amazon Web Services Region.</p>
    #[doc(hidden)]
    pub target_db_cluster_identifier: ::std::option::Option<::std::string::String>,
}
impl FailoverGlobalClusterInput {
    /// <p>Identifier of the Aurora global database (<code>GlobalCluster</code>) that should be failed over. The identifier is the unique key assigned by the user when the Aurora global database was created. In other words, it's the name of the Aurora global database that you want to fail over.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the identifier of an existing <code>GlobalCluster</code> (Aurora global database).</p> </li>
    /// </ul>
    pub fn global_cluster_identifier(&self) -> ::std::option::Option<&str> {
        self.global_cluster_identifier.as_deref()
    }
    /// <p>Identifier of the secondary Aurora DB cluster that you want to promote to primary for the Aurora global database (<code>GlobalCluster</code>.) Use the Amazon Resource Name (ARN) for the identifier so that Aurora can locate the cluster in its Amazon Web Services Region.</p>
    pub fn target_db_cluster_identifier(&self) -> ::std::option::Option<&str> {
        self.target_db_cluster_identifier.as_deref()
    }
}
impl FailoverGlobalClusterInput {
    /// Creates a new builder-style object to manufacture [`FailoverGlobalClusterInput`](crate::operation::failover_global_cluster::FailoverGlobalClusterInput).
    pub fn builder(
    ) -> crate::operation::failover_global_cluster::builders::FailoverGlobalClusterInputBuilder
    {
        crate::operation::failover_global_cluster::builders::FailoverGlobalClusterInputBuilder::default()
    }
}

/// A builder for [`FailoverGlobalClusterInput`](crate::operation::failover_global_cluster::FailoverGlobalClusterInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FailoverGlobalClusterInputBuilder {
    pub(crate) global_cluster_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) target_db_cluster_identifier: ::std::option::Option<::std::string::String>,
}
impl FailoverGlobalClusterInputBuilder {
    /// <p>Identifier of the Aurora global database (<code>GlobalCluster</code>) that should be failed over. The identifier is the unique key assigned by the user when the Aurora global database was created. In other words, it's the name of the Aurora global database that you want to fail over.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the identifier of an existing <code>GlobalCluster</code> (Aurora global database).</p> </li>
    /// </ul>
    pub fn global_cluster_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.global_cluster_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Identifier of the Aurora global database (<code>GlobalCluster</code>) that should be failed over. The identifier is the unique key assigned by the user when the Aurora global database was created. In other words, it's the name of the Aurora global database that you want to fail over.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the identifier of an existing <code>GlobalCluster</code> (Aurora global database).</p> </li>
    /// </ul>
    pub fn set_global_cluster_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.global_cluster_identifier = input;
        self
    }
    /// <p>Identifier of the secondary Aurora DB cluster that you want to promote to primary for the Aurora global database (<code>GlobalCluster</code>.) Use the Amazon Resource Name (ARN) for the identifier so that Aurora can locate the cluster in its Amazon Web Services Region.</p>
    pub fn target_db_cluster_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.target_db_cluster_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Identifier of the secondary Aurora DB cluster that you want to promote to primary for the Aurora global database (<code>GlobalCluster</code>.) Use the Amazon Resource Name (ARN) for the identifier so that Aurora can locate the cluster in its Amazon Web Services Region.</p>
    pub fn set_target_db_cluster_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.target_db_cluster_identifier = input;
        self
    }
    /// Consumes the builder and constructs a [`FailoverGlobalClusterInput`](crate::operation::failover_global_cluster::FailoverGlobalClusterInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::failover_global_cluster::FailoverGlobalClusterInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::failover_global_cluster::FailoverGlobalClusterInput {
                global_cluster_identifier: self.global_cluster_identifier,
                target_db_cluster_identifier: self.target_db_cluster_identifier,
            },
        )
    }
}
