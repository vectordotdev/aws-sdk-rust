// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCluster`](crate::operation::delete_cluster::builders::DeleteClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster(impl ::std::convert::Into<String>)`](crate::operation::delete_cluster::builders::DeleteClusterFluentBuilder::cluster) / [`set_cluster(Option<String>)`](crate::operation::delete_cluster::builders::DeleteClusterFluentBuilder::set_cluster): <p>The short name or full Amazon Resource Name (ARN) of the cluster to delete.</p>
    /// - On success, responds with [`DeleteClusterOutput`](crate::operation::delete_cluster::DeleteClusterOutput) with field(s):
    ///   - [`cluster(Option<Cluster>)`](crate::operation::delete_cluster::DeleteClusterOutput::cluster): <p>The full description of the deleted cluster.</p>
    /// - On failure, responds with [`SdkError<DeleteClusterError>`](crate::operation::delete_cluster::DeleteClusterError)
    pub fn delete_cluster(
        &self,
    ) -> crate::operation::delete_cluster::builders::DeleteClusterFluentBuilder {
        crate::operation::delete_cluster::builders::DeleteClusterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
