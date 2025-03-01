// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateRoute`](crate::operation::update_route::builders::UpdateRouteFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`environment_identifier(impl ::std::convert::Into<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::environment_identifier) / [`set_environment_identifier(Option<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::set_environment_identifier): <p> The ID of the environment in which the route is being updated. </p>
    ///   - [`application_identifier(impl ::std::convert::Into<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::application_identifier) / [`set_application_identifier(Option<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::set_application_identifier): <p> The ID of the application within which the route is being updated. </p>
    ///   - [`route_identifier(impl ::std::convert::Into<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::route_identifier) / [`set_route_identifier(Option<String>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::set_route_identifier): <p> The unique identifier of the route to update. </p>
    ///   - [`activation_state(RouteActivationState)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::activation_state) / [`set_activation_state(Option<RouteActivationState>)`](crate::operation::update_route::builders::UpdateRouteFluentBuilder::set_activation_state): <p> If set to <code>ACTIVE</code>, traffic is forwarded to this route’s service after the route is updated. </p>
    /// - On success, responds with [`UpdateRouteOutput`](crate::operation::update_route::UpdateRouteOutput) with field(s):
    ///   - [`route_id(Option<String>)`](crate::operation::update_route::UpdateRouteOutput::route_id): <p> The unique identifier of the route. </p>
    ///   - [`arn(Option<String>)`](crate::operation::update_route::UpdateRouteOutput::arn): <p> The Amazon Resource Name (ARN) of the route. The format for this ARN is <code>arn:aws:refactor-spaces:<i>region</i>:<i>account-id</i>:<i>resource-type/resource-id</i> </code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. </p>
    ///   - [`service_id(Option<String>)`](crate::operation::update_route::UpdateRouteOutput::service_id): <p> The ID of service in which the route was created. Traffic that matches this route is forwarded to this service. </p>
    ///   - [`application_id(Option<String>)`](crate::operation::update_route::UpdateRouteOutput::application_id): <p> The ID of the application in which the route is being updated. </p>
    ///   - [`state(Option<RouteState>)`](crate::operation::update_route::UpdateRouteOutput::state): <p> The current state of the route. </p>
    ///   - [`last_updated_time(Option<DateTime>)`](crate::operation::update_route::UpdateRouteOutput::last_updated_time): <p> A timestamp that indicates when the route was last updated. </p>
    /// - On failure, responds with [`SdkError<UpdateRouteError>`](crate::operation::update_route::UpdateRouteError)
    pub fn update_route(
        &self,
    ) -> crate::operation::update_route::builders::UpdateRouteFluentBuilder {
        crate::operation::update_route::builders::UpdateRouteFluentBuilder::new(self.handle.clone())
    }
}
