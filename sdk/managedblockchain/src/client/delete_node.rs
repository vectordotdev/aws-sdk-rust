// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteNode`](crate::operation::delete_node::builders::DeleteNodeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`network_id(impl ::std::convert::Into<String>)`](crate::operation::delete_node::builders::DeleteNodeFluentBuilder::network_id) / [`set_network_id(Option<String>)`](crate::operation::delete_node::builders::DeleteNodeFluentBuilder::set_network_id): <p>The unique identifier of the network that the node is on.</p>  <p>Ethereum public networks have the following <code>NetworkId</code>s:</p>  <ul>   <li> <p> <code>n-ethereum-mainnet</code> </p> </li>   <li> <p> <code>n-ethereum-goerli</code> </p> </li>   <li> <p> <code>n-ethereum-rinkeby</code> </p> </li>  </ul>
    ///   - [`member_id(impl ::std::convert::Into<String>)`](crate::operation::delete_node::builders::DeleteNodeFluentBuilder::member_id) / [`set_member_id(Option<String>)`](crate::operation::delete_node::builders::DeleteNodeFluentBuilder::set_member_id): <p>The unique identifier of the member that owns this node.</p>  <p>Applies only to Hyperledger Fabric and is required for Hyperledger Fabric.</p>
    ///   - [`node_id(impl ::std::convert::Into<String>)`](crate::operation::delete_node::builders::DeleteNodeFluentBuilder::node_id) / [`set_node_id(Option<String>)`](crate::operation::delete_node::builders::DeleteNodeFluentBuilder::set_node_id): <p>The unique identifier of the node.</p>
    /// - On success, responds with [`DeleteNodeOutput`](crate::operation::delete_node::DeleteNodeOutput)
    /// - On failure, responds with [`SdkError<DeleteNodeError>`](crate::operation::delete_node::DeleteNodeError)
    pub fn delete_node(&self) -> crate::operation::delete_node::builders::DeleteNodeFluentBuilder {
        crate::operation::delete_node::builders::DeleteNodeFluentBuilder::new(self.handle.clone())
    }
}
