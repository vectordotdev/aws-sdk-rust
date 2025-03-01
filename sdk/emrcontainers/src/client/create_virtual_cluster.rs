// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateVirtualCluster`](crate::operation::create_virtual_cluster::builders::CreateVirtualClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_virtual_cluster::builders::CreateVirtualClusterFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_virtual_cluster::builders::CreateVirtualClusterFluentBuilder::set_name): <p>The specified name of the virtual cluster.</p>
    ///   - [`container_provider(ContainerProvider)`](crate::operation::create_virtual_cluster::builders::CreateVirtualClusterFluentBuilder::container_provider) / [`set_container_provider(Option<ContainerProvider>)`](crate::operation::create_virtual_cluster::builders::CreateVirtualClusterFluentBuilder::set_container_provider): <p>The container provider of the virtual cluster.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::create_virtual_cluster::builders::CreateVirtualClusterFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_virtual_cluster::builders::CreateVirtualClusterFluentBuilder::set_client_token): <p>The client token of the virtual cluster.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_virtual_cluster::builders::CreateVirtualClusterFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_virtual_cluster::builders::CreateVirtualClusterFluentBuilder::set_tags): <p>The tags assigned to the virtual cluster.</p>
    /// - On success, responds with [`CreateVirtualClusterOutput`](crate::operation::create_virtual_cluster::CreateVirtualClusterOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::operation::create_virtual_cluster::CreateVirtualClusterOutput::id): <p>This output contains the virtual cluster ID.</p>
    ///   - [`name(Option<String>)`](crate::operation::create_virtual_cluster::CreateVirtualClusterOutput::name): <p>This output contains the name of the virtual cluster.</p>
    ///   - [`arn(Option<String>)`](crate::operation::create_virtual_cluster::CreateVirtualClusterOutput::arn): <p>This output contains the ARN of virtual cluster.</p>
    /// - On failure, responds with [`SdkError<CreateVirtualClusterError>`](crate::operation::create_virtual_cluster::CreateVirtualClusterError)
    pub fn create_virtual_cluster(
        &self,
    ) -> crate::operation::create_virtual_cluster::builders::CreateVirtualClusterFluentBuilder {
        crate::operation::create_virtual_cluster::builders::CreateVirtualClusterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
