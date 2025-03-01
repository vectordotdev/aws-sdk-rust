// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListInventoryEntries`](crate::operation::list_inventory_entries::builders::ListInventoryEntriesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::list_inventory_entries::builders::ListInventoryEntriesFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::list_inventory_entries::builders::ListInventoryEntriesFluentBuilder::set_instance_id): <p>The managed node ID for which you want inventory information.</p>
    ///   - [`type_name(impl ::std::convert::Into<String>)`](crate::operation::list_inventory_entries::builders::ListInventoryEntriesFluentBuilder::type_name) / [`set_type_name(Option<String>)`](crate::operation::list_inventory_entries::builders::ListInventoryEntriesFluentBuilder::set_type_name): <p>The type of inventory item for which you want information.</p>
    ///   - [`filters(Vec<InventoryFilter>)`](crate::operation::list_inventory_entries::builders::ListInventoryEntriesFluentBuilder::filters) / [`set_filters(Option<Vec<InventoryFilter>>)`](crate::operation::list_inventory_entries::builders::ListInventoryEntriesFluentBuilder::set_filters): <p>One or more filters. Use a filter to return a more specific list of results.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_inventory_entries::builders::ListInventoryEntriesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_inventory_entries::builders::ListInventoryEntriesFluentBuilder::set_next_token): <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    ///   - [`max_results(i32)`](crate::operation::list_inventory_entries::builders::ListInventoryEntriesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_inventory_entries::builders::ListInventoryEntriesFluentBuilder::set_max_results): <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    /// - On success, responds with [`ListInventoryEntriesOutput`](crate::operation::list_inventory_entries::ListInventoryEntriesOutput) with field(s):
    ///   - [`type_name(Option<String>)`](crate::operation::list_inventory_entries::ListInventoryEntriesOutput::type_name): <p>The type of inventory item returned by the request.</p>
    ///   - [`instance_id(Option<String>)`](crate::operation::list_inventory_entries::ListInventoryEntriesOutput::instance_id): <p>The managed node ID targeted by the request to query inventory information.</p>
    ///   - [`schema_version(Option<String>)`](crate::operation::list_inventory_entries::ListInventoryEntriesOutput::schema_version): <p>The inventory schema version used by the managed node(s).</p>
    ///   - [`capture_time(Option<String>)`](crate::operation::list_inventory_entries::ListInventoryEntriesOutput::capture_time): <p>The time that inventory information was collected for the managed node(s).</p>
    ///   - [`entries(Option<Vec<HashMap<String, String>>>)`](crate::operation::list_inventory_entries::ListInventoryEntriesOutput::entries): <p>A list of inventory items on the managed node(s).</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_inventory_entries::ListInventoryEntriesOutput::next_token): <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>
    /// - On failure, responds with [`SdkError<ListInventoryEntriesError>`](crate::operation::list_inventory_entries::ListInventoryEntriesError)
    pub fn list_inventory_entries(
        &self,
    ) -> crate::operation::list_inventory_entries::builders::ListInventoryEntriesFluentBuilder {
        crate::operation::list_inventory_entries::builders::ListInventoryEntriesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
