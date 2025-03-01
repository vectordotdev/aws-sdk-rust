// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdatePartition`](crate::operation::update_partition::builders::UpdatePartitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl ::std::convert::Into<String>)`](crate::operation::update_partition::builders::UpdatePartitionFluentBuilder::catalog_id) / [`set_catalog_id(Option<String>)`](crate::operation::update_partition::builders::UpdatePartitionFluentBuilder::set_catalog_id): <p>The ID of the Data Catalog where the partition to be updated resides. If none is provided, the Amazon Web Services account ID is used by default.</p>
    ///   - [`database_name(impl ::std::convert::Into<String>)`](crate::operation::update_partition::builders::UpdatePartitionFluentBuilder::database_name) / [`set_database_name(Option<String>)`](crate::operation::update_partition::builders::UpdatePartitionFluentBuilder::set_database_name): <p>The name of the catalog database in which the table in question resides.</p>
    ///   - [`table_name(impl ::std::convert::Into<String>)`](crate::operation::update_partition::builders::UpdatePartitionFluentBuilder::table_name) / [`set_table_name(Option<String>)`](crate::operation::update_partition::builders::UpdatePartitionFluentBuilder::set_table_name): <p>The name of the table in which the partition to be updated is located.</p>
    ///   - [`partition_value_list(Vec<String>)`](crate::operation::update_partition::builders::UpdatePartitionFluentBuilder::partition_value_list) / [`set_partition_value_list(Option<Vec<String>>)`](crate::operation::update_partition::builders::UpdatePartitionFluentBuilder::set_partition_value_list): <p>List of partition key values that define the partition to update.</p>
    ///   - [`partition_input(PartitionInput)`](crate::operation::update_partition::builders::UpdatePartitionFluentBuilder::partition_input) / [`set_partition_input(Option<PartitionInput>)`](crate::operation::update_partition::builders::UpdatePartitionFluentBuilder::set_partition_input): <p>The new partition object to update the partition to.</p>  <p>The <code>Values</code> property can't be changed. If you want to change the partition key values for a partition, delete and recreate the partition.</p>
    /// - On success, responds with [`UpdatePartitionOutput`](crate::operation::update_partition::UpdatePartitionOutput)
    /// - On failure, responds with [`SdkError<UpdatePartitionError>`](crate::operation::update_partition::UpdatePartitionError)
    pub fn update_partition(
        &self,
    ) -> crate::operation::update_partition::builders::UpdatePartitionFluentBuilder {
        crate::operation::update_partition::builders::UpdatePartitionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
