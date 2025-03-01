// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDeploymentGroups`](crate::operation::list_deployment_groups::builders::ListDeploymentGroupsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_deployment_groups::builders::ListDeploymentGroupsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_name(impl ::std::convert::Into<String>)`](crate::operation::list_deployment_groups::builders::ListDeploymentGroupsFluentBuilder::application_name) / [`set_application_name(Option<String>)`](crate::operation::list_deployment_groups::builders::ListDeploymentGroupsFluentBuilder::set_application_name): <p>The name of an CodeDeploy application associated with the IAM user or Amazon Web Services account.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_deployment_groups::builders::ListDeploymentGroupsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_deployment_groups::builders::ListDeploymentGroupsFluentBuilder::set_next_token): <p>An identifier returned from the previous list deployment groups call. It can be used to return the next set of deployment groups in the list.</p>
    /// - On success, responds with [`ListDeploymentGroupsOutput`](crate::operation::list_deployment_groups::ListDeploymentGroupsOutput) with field(s):
    ///   - [`application_name(Option<String>)`](crate::operation::list_deployment_groups::ListDeploymentGroupsOutput::application_name): <p>The application name.</p>
    ///   - [`deployment_groups(Option<Vec<String>>)`](crate::operation::list_deployment_groups::ListDeploymentGroupsOutput::deployment_groups): <p>A list of deployment group names.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_deployment_groups::ListDeploymentGroupsOutput::next_token): <p>If a large amount of information is returned, an identifier is also returned. It can be used in a subsequent list deployment groups call to return the next set of deployment groups in the list.</p>
    /// - On failure, responds with [`SdkError<ListDeploymentGroupsError>`](crate::operation::list_deployment_groups::ListDeploymentGroupsError)
    pub fn list_deployment_groups(
        &self,
    ) -> crate::operation::list_deployment_groups::builders::ListDeploymentGroupsFluentBuilder {
        crate::operation::list_deployment_groups::builders::ListDeploymentGroupsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
