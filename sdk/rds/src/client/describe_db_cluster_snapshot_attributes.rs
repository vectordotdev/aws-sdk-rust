// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDBClusterSnapshotAttributes`](crate::operation::describe_db_cluster_snapshot_attributes::builders::DescribeDBClusterSnapshotAttributesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`db_cluster_snapshot_identifier(impl ::std::convert::Into<String>)`](crate::operation::describe_db_cluster_snapshot_attributes::builders::DescribeDBClusterSnapshotAttributesFluentBuilder::db_cluster_snapshot_identifier) / [`set_db_cluster_snapshot_identifier(Option<String>)`](crate::operation::describe_db_cluster_snapshot_attributes::builders::DescribeDBClusterSnapshotAttributesFluentBuilder::set_db_cluster_snapshot_identifier): <p>The identifier for the DB cluster snapshot to describe the attributes for.</p>
    /// - On success, responds with [`DescribeDbClusterSnapshotAttributesOutput`](crate::operation::describe_db_cluster_snapshot_attributes::DescribeDbClusterSnapshotAttributesOutput) with field(s):
    ///   - [`db_cluster_snapshot_attributes_result(Option<DbClusterSnapshotAttributesResult>)`](crate::operation::describe_db_cluster_snapshot_attributes::DescribeDbClusterSnapshotAttributesOutput::db_cluster_snapshot_attributes_result): <p>Contains the results of a successful call to the <code>DescribeDBClusterSnapshotAttributes</code> API action.</p>  <p>Manual DB cluster snapshot attributes are used to authorize other Amazon Web Services accounts to copy or restore a manual DB cluster snapshot. For more information, see the <code>ModifyDBClusterSnapshotAttribute</code> API action.</p>
    /// - On failure, responds with [`SdkError<DescribeDBClusterSnapshotAttributesError>`](crate::operation::describe_db_cluster_snapshot_attributes::DescribeDBClusterSnapshotAttributesError)
    pub fn describe_db_cluster_snapshot_attributes(&self) -> crate::operation::describe_db_cluster_snapshot_attributes::builders::DescribeDBClusterSnapshotAttributesFluentBuilder{
        crate::operation::describe_db_cluster_snapshot_attributes::builders::DescribeDBClusterSnapshotAttributesFluentBuilder::new(self.handle.clone())
    }
}
