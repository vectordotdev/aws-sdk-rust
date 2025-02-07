// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListClusters`](crate::operation::list_clusters::builders::ListClustersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_clusters::builders::ListClustersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`created_after(DateTime)`](crate::operation::list_clusters::builders::ListClustersFluentBuilder::created_after) / [`set_created_after(Option<DateTime>)`](crate::operation::list_clusters::builders::ListClustersFluentBuilder::set_created_after): <p>The creation date and time beginning value filter for listing clusters.</p>
    ///   - [`created_before(DateTime)`](crate::operation::list_clusters::builders::ListClustersFluentBuilder::created_before) / [`set_created_before(Option<DateTime>)`](crate::operation::list_clusters::builders::ListClustersFluentBuilder::set_created_before): <p>The creation date and time end value filter for listing clusters.</p>
    ///   - [`cluster_states(Vec<ClusterState>)`](crate::operation::list_clusters::builders::ListClustersFluentBuilder::cluster_states) / [`set_cluster_states(Option<Vec<ClusterState>>)`](crate::operation::list_clusters::builders::ListClustersFluentBuilder::set_cluster_states): <p>The cluster state filters to apply when listing clusters. Clusters that change state while this action runs may be not be returned as expected in the list of clusters.</p>
    ///   - [`marker(impl ::std::convert::Into<String>)`](crate::operation::list_clusters::builders::ListClustersFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_clusters::builders::ListClustersFluentBuilder::set_marker): <p>The pagination token that indicates the next set of results to retrieve.</p>
    /// - On success, responds with [`ListClustersOutput`](crate::operation::list_clusters::ListClustersOutput) with field(s):
    ///   - [`clusters(Option<Vec<ClusterSummary>>)`](crate::operation::list_clusters::ListClustersOutput::clusters): <p>The list of clusters for the account based on the given filters.</p>
    ///   - [`marker(Option<String>)`](crate::operation::list_clusters::ListClustersOutput::marker): <p>The pagination token that indicates the next set of results to retrieve.</p>
    /// - On failure, responds with [`SdkError<ListClustersError>`](crate::operation::list_clusters::ListClustersError)
    pub fn list_clusters(
        &self,
    ) -> crate::operation::list_clusters::builders::ListClustersFluentBuilder {
        crate::operation::list_clusters::builders::ListClustersFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
