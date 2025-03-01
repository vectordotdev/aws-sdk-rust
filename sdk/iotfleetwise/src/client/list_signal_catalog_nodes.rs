// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSignalCatalogNodes`](crate::operation::list_signal_catalog_nodes::builders::ListSignalCatalogNodesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_signal_catalog_nodes::builders::ListSignalCatalogNodesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::list_signal_catalog_nodes::builders::ListSignalCatalogNodesFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::list_signal_catalog_nodes::builders::ListSignalCatalogNodesFluentBuilder::set_name): <p> The name of the signal catalog to list information about. </p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_signal_catalog_nodes::builders::ListSignalCatalogNodesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_signal_catalog_nodes::builders::ListSignalCatalogNodesFluentBuilder::set_next_token): <p>A pagination token for the next set of results.</p>  <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next set of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value. </p>
    ///   - [`max_results(i32)`](crate::operation::list_signal_catalog_nodes::builders::ListSignalCatalogNodesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_signal_catalog_nodes::builders::ListSignalCatalogNodesFluentBuilder::set_max_results): <p> The maximum number of items to return, between 1 and 100, inclusive. </p>
    /// - On success, responds with [`ListSignalCatalogNodesOutput`](crate::operation::list_signal_catalog_nodes::ListSignalCatalogNodesOutput) with field(s):
    ///   - [`nodes(Option<Vec<Node>>)`](crate::operation::list_signal_catalog_nodes::ListSignalCatalogNodesOutput::nodes): <p> A list of information about nodes. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_signal_catalog_nodes::ListSignalCatalogNodesOutput::next_token): <p> The token to retrieve the next set of results, or <code>null</code> if there are no more results. </p>
    /// - On failure, responds with [`SdkError<ListSignalCatalogNodesError>`](crate::operation::list_signal_catalog_nodes::ListSignalCatalogNodesError)
    pub fn list_signal_catalog_nodes(
        &self,
    ) -> crate::operation::list_signal_catalog_nodes::builders::ListSignalCatalogNodesFluentBuilder
    {
        crate::operation::list_signal_catalog_nodes::builders::ListSignalCatalogNodesFluentBuilder::new(self.handle.clone())
    }
}
