// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateEnvironment`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::set_name): <p>The name of the environment.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::set_description): <p>The description of the environment.</p>
    ///   - [`network_fabric_type(NetworkFabricType)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::network_fabric_type) / [`set_network_fabric_type(Option<NetworkFabricType>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::set_network_fabric_type): <p>The network fabric type of the environment.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::set_tags): <p>The tags to assign to the environment. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::set_client_token): <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    /// - On success, responds with [`CreateEnvironmentOutput`](crate::operation::create_environment::CreateEnvironmentOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::operation::create_environment::CreateEnvironmentOutput::name): <p>The name of the environment.</p>
    ///   - [`arn(Option<String>)`](crate::operation::create_environment::CreateEnvironmentOutput::arn): <p>The Amazon Resource Name (ARN) of the environment.</p>
    ///   - [`description(Option<String>)`](crate::operation::create_environment::CreateEnvironmentOutput::description): <p>A description of the environment.</p>
    ///   - [`environment_id(Option<String>)`](crate::operation::create_environment::CreateEnvironmentOutput::environment_id): <p>The unique identifier of the environment.</p>
    ///   - [`network_fabric_type(Option<NetworkFabricType>)`](crate::operation::create_environment::CreateEnvironmentOutput::network_fabric_type): <p>The network fabric type of the environment.</p>
    ///   - [`owner_account_id(Option<String>)`](crate::operation::create_environment::CreateEnvironmentOutput::owner_account_id): <p>The Amazon Web Services account ID of environment owner.</p>
    ///   - [`state(Option<EnvironmentState>)`](crate::operation::create_environment::CreateEnvironmentOutput::state): <p>The current state of the environment. </p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::create_environment::CreateEnvironmentOutput::tags): <p>The tags assigned to the created environment. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair..</p>
    ///   - [`last_updated_time(Option<DateTime>)`](crate::operation::create_environment::CreateEnvironmentOutput::last_updated_time): <p>A timestamp that indicates when the environment was last updated.</p>
    ///   - [`created_time(Option<DateTime>)`](crate::operation::create_environment::CreateEnvironmentOutput::created_time): <p>A timestamp that indicates when the environment is created.</p>
    /// - On failure, responds with [`SdkError<CreateEnvironmentError>`](crate::operation::create_environment::CreateEnvironmentError)
    pub fn create_environment(
        &self,
    ) -> crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder {
        crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
