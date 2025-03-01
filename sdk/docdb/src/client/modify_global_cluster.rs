// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyGlobalCluster`](crate::operation::modify_global_cluster::builders::ModifyGlobalClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_cluster_identifier(impl ::std::convert::Into<String>)`](crate::operation::modify_global_cluster::builders::ModifyGlobalClusterFluentBuilder::global_cluster_identifier) / [`set_global_cluster_identifier(Option<String>)`](crate::operation::modify_global_cluster::builders::ModifyGlobalClusterFluentBuilder::set_global_cluster_identifier): <p>The identifier for the global cluster being modified. This parameter isn't case-sensitive.</p>  <p>Constraints:</p>  <ul>   <li> <p>Must match the identifier of an existing global cluster.</p> </li>  </ul>
    ///   - [`new_global_cluster_identifier(impl ::std::convert::Into<String>)`](crate::operation::modify_global_cluster::builders::ModifyGlobalClusterFluentBuilder::new_global_cluster_identifier) / [`set_new_global_cluster_identifier(Option<String>)`](crate::operation::modify_global_cluster::builders::ModifyGlobalClusterFluentBuilder::set_new_global_cluster_identifier): <p>The new identifier for a global cluster when you modify a global cluster. This value is stored as a lowercase string.</p>  <ul>   <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens</p> <p>The first character must be a letter</p> <p>Can't end with a hyphen or contain two consecutive hyphens</p> </li>  </ul>  <p>Example: <code>my-cluster2</code> </p>
    ///   - [`deletion_protection(bool)`](crate::operation::modify_global_cluster::builders::ModifyGlobalClusterFluentBuilder::deletion_protection) / [`set_deletion_protection(Option<bool>)`](crate::operation::modify_global_cluster::builders::ModifyGlobalClusterFluentBuilder::set_deletion_protection): <p>Indicates if the global cluster has deletion protection enabled. The global cluster can't be deleted when deletion protection is enabled. </p>
    /// - On success, responds with [`ModifyGlobalClusterOutput`](crate::operation::modify_global_cluster::ModifyGlobalClusterOutput) with field(s):
    ///   - [`global_cluster(Option<GlobalCluster>)`](crate::operation::modify_global_cluster::ModifyGlobalClusterOutput::global_cluster): <p>A data type representing an Amazon DocumentDB global cluster.</p>
    /// - On failure, responds with [`SdkError<ModifyGlobalClusterError>`](crate::operation::modify_global_cluster::ModifyGlobalClusterError)
    pub fn modify_global_cluster(
        &self,
    ) -> crate::operation::modify_global_cluster::builders::ModifyGlobalClusterFluentBuilder {
        crate::operation::modify_global_cluster::builders::ModifyGlobalClusterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
