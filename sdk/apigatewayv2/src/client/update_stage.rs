// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateStage`](crate::operation::update_stage::builders::UpdateStageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`access_log_settings(AccessLogSettings)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::access_log_settings) / [`set_access_log_settings(Option<AccessLogSettings>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::set_access_log_settings): <p>Settings for logging access in this stage.</p>
    ///   - [`api_id(impl ::std::convert::Into<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::api_id) / [`set_api_id(Option<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::set_api_id): <p>The API identifier.</p>
    ///   - [`auto_deploy(bool)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::auto_deploy) / [`set_auto_deploy(Option<bool>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::set_auto_deploy): <p>Specifies whether updates to an API automatically trigger a new deployment. The default value is false.</p>
    ///   - [`client_certificate_id(impl ::std::convert::Into<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::client_certificate_id) / [`set_client_certificate_id(Option<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::set_client_certificate_id): <p>The identifier of a client certificate for a Stage.</p>
    ///   - [`default_route_settings(RouteSettings)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::default_route_settings) / [`set_default_route_settings(Option<RouteSettings>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::set_default_route_settings): <p>The default route settings for the stage.</p>
    ///   - [`deployment_id(impl ::std::convert::Into<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::deployment_id) / [`set_deployment_id(Option<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::set_deployment_id): <p>The deployment identifier for the API stage. Can't be updated if autoDeploy is enabled.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::set_description): <p>The description for the API stage.</p>
    ///   - [`route_settings(HashMap<String, RouteSettings>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::route_settings) / [`set_route_settings(Option<HashMap<String, RouteSettings>>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::set_route_settings): <p>Route settings for the stage.</p>
    ///   - [`stage_name(impl ::std::convert::Into<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::stage_name) / [`set_stage_name(Option<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::set_stage_name): <p>The stage name. Stage names can contain only alphanumeric characters, hyphens, and underscores, or be $default. Maximum length is 128 characters.</p>
    ///   - [`stage_variables(HashMap<String, String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::stage_variables) / [`set_stage_variables(Option<HashMap<String, String>>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::set_stage_variables): <p>A map that defines the stage variables for a Stage. Variable names can have alphanumeric and underscore characters, and the values must match [A-Za-z0-9-._~:/?#&amp;=,]+.</p>
    /// - On success, responds with [`UpdateStageOutput`](crate::operation::update_stage::UpdateStageOutput) with field(s):
    ///   - [`access_log_settings(Option<AccessLogSettings>)`](crate::operation::update_stage::UpdateStageOutput::access_log_settings): <p>Settings for logging access in this stage.</p>
    ///   - [`api_gateway_managed(Option<bool>)`](crate::operation::update_stage::UpdateStageOutput::api_gateway_managed): <p>Specifies whether a stage is managed by API Gateway. If you created an API using quick create, the $default stage is managed by API Gateway. You can't modify the $default stage.</p>
    ///   - [`auto_deploy(Option<bool>)`](crate::operation::update_stage::UpdateStageOutput::auto_deploy): <p>Specifies whether updates to an API automatically trigger a new deployment. The default value is false.</p>
    ///   - [`client_certificate_id(Option<String>)`](crate::operation::update_stage::UpdateStageOutput::client_certificate_id): <p>The identifier of a client certificate for a Stage. Supported only for WebSocket APIs.</p>
    ///   - [`created_date(Option<DateTime>)`](crate::operation::update_stage::UpdateStageOutput::created_date): <p>The timestamp when the stage was created.</p>
    ///   - [`default_route_settings(Option<RouteSettings>)`](crate::operation::update_stage::UpdateStageOutput::default_route_settings): <p>Default route settings for the stage.</p>
    ///   - [`deployment_id(Option<String>)`](crate::operation::update_stage::UpdateStageOutput::deployment_id): <p>The identifier of the Deployment that the Stage is associated with. Can't be updated if autoDeploy is enabled.</p>
    ///   - [`description(Option<String>)`](crate::operation::update_stage::UpdateStageOutput::description): <p>The description of the stage.</p>
    ///   - [`last_deployment_status_message(Option<String>)`](crate::operation::update_stage::UpdateStageOutput::last_deployment_status_message): <p>Describes the status of the last deployment of a stage. Supported only for stages with autoDeploy enabled.</p>
    ///   - [`last_updated_date(Option<DateTime>)`](crate::operation::update_stage::UpdateStageOutput::last_updated_date): <p>The timestamp when the stage was last updated.</p>
    ///   - [`route_settings(Option<HashMap<String, RouteSettings>>)`](crate::operation::update_stage::UpdateStageOutput::route_settings): <p>Route settings for the stage, by routeKey.</p>
    ///   - [`stage_name(Option<String>)`](crate::operation::update_stage::UpdateStageOutput::stage_name): <p>The name of the stage.</p>
    ///   - [`stage_variables(Option<HashMap<String, String>>)`](crate::operation::update_stage::UpdateStageOutput::stage_variables): <p>A map that defines the stage variables for a stage resource. Variable names can have alphanumeric and underscore characters, and the values must match [A-Za-z0-9-._~:/?#&amp;=,]+.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::update_stage::UpdateStageOutput::tags): <p>The collection of tags. Each tag element is associated with a given resource.</p>
    /// - On failure, responds with [`SdkError<UpdateStageError>`](crate::operation::update_stage::UpdateStageError)
    pub fn update_stage(
        &self,
    ) -> crate::operation::update_stage::builders::UpdateStageFluentBuilder {
        crate::operation::update_stage::builders::UpdateStageFluentBuilder::new(self.handle.clone())
    }
}
