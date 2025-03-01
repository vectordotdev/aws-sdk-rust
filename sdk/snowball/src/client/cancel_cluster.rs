// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelCluster`](crate::operation::cancel_cluster::builders::CancelClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_id(impl ::std::convert::Into<String>)`](crate::operation::cancel_cluster::builders::CancelClusterFluentBuilder::cluster_id) / [`set_cluster_id(Option<String>)`](crate::operation::cancel_cluster::builders::CancelClusterFluentBuilder::set_cluster_id): <p>The 39-character ID for the cluster that you want to cancel, for example <code>CID123e4567-e89b-12d3-a456-426655440000</code>.</p>
    /// - On success, responds with [`CancelClusterOutput`](crate::operation::cancel_cluster::CancelClusterOutput)
    /// - On failure, responds with [`SdkError<CancelClusterError>`](crate::operation::cancel_cluster::CancelClusterError)
    pub fn cancel_cluster(
        &self,
    ) -> crate::operation::cancel_cluster::builders::CancelClusterFluentBuilder {
        crate::operation::cancel_cluster::builders::CancelClusterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
