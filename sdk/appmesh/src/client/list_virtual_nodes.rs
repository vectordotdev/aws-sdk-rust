// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListVirtualNodes`](crate::operation::list_virtual_nodes::builders::ListVirtualNodesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_virtual_nodes::builders::ListVirtualNodesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`mesh_name(impl ::std::convert::Into<String>)`](crate::operation::list_virtual_nodes::builders::ListVirtualNodesFluentBuilder::mesh_name) / [`set_mesh_name(Option<String>)`](crate::operation::list_virtual_nodes::builders::ListVirtualNodesFluentBuilder::set_mesh_name): <p>The name of the service mesh to list virtual nodes in.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_virtual_nodes::builders::ListVirtualNodesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_virtual_nodes::builders::ListVirtualNodesFluentBuilder::set_next_token): <p>The <code>nextToken</code> value returned from a previous paginated <code>ListVirtualNodes</code> request where <code>limit</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p>
    ///   - [`limit(i32)`](crate::operation::list_virtual_nodes::builders::ListVirtualNodesFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_virtual_nodes::builders::ListVirtualNodesFluentBuilder::set_limit): <p>The maximum number of results returned by <code>ListVirtualNodes</code> in paginated output. When you use this parameter, <code>ListVirtualNodes</code> returns only <code>limit</code> results in a single page along with a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListVirtualNodes</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If you don't use this parameter, <code>ListVirtualNodes</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    ///   - [`mesh_owner(impl ::std::convert::Into<String>)`](crate::operation::list_virtual_nodes::builders::ListVirtualNodesFluentBuilder::mesh_owner) / [`set_mesh_owner(Option<String>)`](crate::operation::list_virtual_nodes::builders::ListVirtualNodesFluentBuilder::set_mesh_owner): <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then it's the ID of the account that shared the mesh with your account. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    /// - On success, responds with [`ListVirtualNodesOutput`](crate::operation::list_virtual_nodes::ListVirtualNodesOutput) with field(s):
    ///   - [`virtual_nodes(Option<Vec<VirtualNodeRef>>)`](crate::operation::list_virtual_nodes::ListVirtualNodesOutput::virtual_nodes): <p>The list of existing virtual nodes for the specified service mesh.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_virtual_nodes::ListVirtualNodesOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>ListVirtualNodes</code> request. When the results of a <code>ListVirtualNodes</code> request exceed <code>limit</code>, you can use this value to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<ListVirtualNodesError>`](crate::operation::list_virtual_nodes::ListVirtualNodesError)
    pub fn list_virtual_nodes(
        &self,
    ) -> crate::operation::list_virtual_nodes::builders::ListVirtualNodesFluentBuilder {
        crate::operation::list_virtual_nodes::builders::ListVirtualNodesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
