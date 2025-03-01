// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateContainerServiceDeployment`](crate::operation::create_container_service_deployment::builders::CreateContainerServiceDeploymentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`service_name(impl ::std::convert::Into<String>)`](crate::operation::create_container_service_deployment::builders::CreateContainerServiceDeploymentFluentBuilder::service_name) / [`set_service_name(Option<String>)`](crate::operation::create_container_service_deployment::builders::CreateContainerServiceDeploymentFluentBuilder::set_service_name): <p>The name of the container service for which to create the deployment.</p>
    ///   - [`containers(HashMap<String, Container>)`](crate::operation::create_container_service_deployment::builders::CreateContainerServiceDeploymentFluentBuilder::containers) / [`set_containers(Option<HashMap<String, Container>>)`](crate::operation::create_container_service_deployment::builders::CreateContainerServiceDeploymentFluentBuilder::set_containers): <p>An object that describes the settings of the containers that will be launched on the container service.</p>
    ///   - [`public_endpoint(EndpointRequest)`](crate::operation::create_container_service_deployment::builders::CreateContainerServiceDeploymentFluentBuilder::public_endpoint) / [`set_public_endpoint(Option<EndpointRequest>)`](crate::operation::create_container_service_deployment::builders::CreateContainerServiceDeploymentFluentBuilder::set_public_endpoint): <p>An object that describes the settings of the public endpoint for the container service.</p>
    /// - On success, responds with [`CreateContainerServiceDeploymentOutput`](crate::operation::create_container_service_deployment::CreateContainerServiceDeploymentOutput) with field(s):
    ///   - [`container_service(Option<ContainerService>)`](crate::operation::create_container_service_deployment::CreateContainerServiceDeploymentOutput::container_service): <p>An object that describes a container service.</p>
    /// - On failure, responds with [`SdkError<CreateContainerServiceDeploymentError>`](crate::operation::create_container_service_deployment::CreateContainerServiceDeploymentError)
    pub fn create_container_service_deployment(&self) -> crate::operation::create_container_service_deployment::builders::CreateContainerServiceDeploymentFluentBuilder{
        crate::operation::create_container_service_deployment::builders::CreateContainerServiceDeploymentFluentBuilder::new(self.handle.clone())
    }
}
