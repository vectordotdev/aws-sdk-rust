// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateManagedEndpoint`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::set_name): <p>The name of the managed endpoint.</p>
    ///   - [`virtual_cluster_id(impl ::std::convert::Into<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::virtual_cluster_id) / [`set_virtual_cluster_id(Option<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::set_virtual_cluster_id): <p>The ID of the virtual cluster for which a managed endpoint is created.</p>
    ///   - [`r#type(impl ::std::convert::Into<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::type) / [`set_type(Option<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::set_type): <p>The type of the managed endpoint.</p>
    ///   - [`release_label(impl ::std::convert::Into<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::release_label) / [`set_release_label(Option<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::set_release_label): <p>The Amazon EMR release version.</p>
    ///   - [`execution_role_arn(impl ::std::convert::Into<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::execution_role_arn) / [`set_execution_role_arn(Option<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::set_execution_role_arn): <p>The ARN of the execution role.</p>
    ///   - [`certificate_arn(impl ::std::convert::Into<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::certificate_arn) / [`set_certificate_arn(Option<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::set_certificate_arn): <p>The certificate ARN provided by users for the managed endpoint. This field is under deprecation and will be removed in future releases.</p>
    ///   - [`configuration_overrides(ConfigurationOverrides)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::configuration_overrides) / [`set_configuration_overrides(Option<ConfigurationOverrides>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::set_configuration_overrides): <p>The configuration settings that will be used to override existing configurations.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::set_client_token): <p>The client idempotency token for this create call.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::set_tags): <p>The tags of the managed endpoint. </p>
    /// - On success, responds with [`CreateManagedEndpointOutput`](crate::operation::create_managed_endpoint::CreateManagedEndpointOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::operation::create_managed_endpoint::CreateManagedEndpointOutput::id): <p>The output contains the ID of the managed endpoint.</p>
    ///   - [`name(Option<String>)`](crate::operation::create_managed_endpoint::CreateManagedEndpointOutput::name): <p>The output contains the name of the managed endpoint.</p>
    ///   - [`arn(Option<String>)`](crate::operation::create_managed_endpoint::CreateManagedEndpointOutput::arn): <p>The output contains the ARN of the managed endpoint.</p>
    ///   - [`virtual_cluster_id(Option<String>)`](crate::operation::create_managed_endpoint::CreateManagedEndpointOutput::virtual_cluster_id): <p>The output contains the ID of the virtual cluster.</p>
    /// - On failure, responds with [`SdkError<CreateManagedEndpointError>`](crate::operation::create_managed_endpoint::CreateManagedEndpointError)
    pub fn create_managed_endpoint(
        &self,
    ) -> crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder
    {
        crate::operation::create_managed_endpoint::builders::CreateManagedEndpointFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
