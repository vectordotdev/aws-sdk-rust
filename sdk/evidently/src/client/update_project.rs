// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateProject`](crate::operation::update_project::builders::UpdateProjectFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`project(impl ::std::convert::Into<String>)`](crate::operation::update_project::builders::UpdateProjectFluentBuilder::project) / [`set_project(Option<String>)`](crate::operation::update_project::builders::UpdateProjectFluentBuilder::set_project): <p>The name or ARN of the project to update.</p>
    ///   - [`app_config_resource(ProjectAppConfigResourceConfig)`](crate::operation::update_project::builders::UpdateProjectFluentBuilder::app_config_resource) / [`set_app_config_resource(Option<ProjectAppConfigResourceConfig>)`](crate::operation::update_project::builders::UpdateProjectFluentBuilder::set_app_config_resource): <p>Use this parameter if the project will use client-side evaluation powered by AppConfig. Client-side evaluation allows your application to assign variations to user sessions locally instead of by calling the <a href="https://docs.aws.amazon.com/cloudwatchevidently/latest/APIReference/API_EvaluateFeature.html">EvaluateFeature</a> operation. This mitigates the latency and availability risks that come with an API call. allows you to</p>  <p>This parameter is a structure that contains information about the AppConfig application that will be used for client-side evaluation.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_project::builders::UpdateProjectFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_project::builders::UpdateProjectFluentBuilder::set_description): <p>An optional description of the project.</p>
    /// - On success, responds with [`UpdateProjectOutput`](crate::operation::update_project::UpdateProjectOutput) with field(s):
    ///   - [`project(Option<Project>)`](crate::operation::update_project::UpdateProjectOutput::project): <p>A structure containing information about the updated project.</p>
    /// - On failure, responds with [`SdkError<UpdateProjectError>`](crate::operation::update_project::UpdateProjectError)
    pub fn update_project(
        &self,
    ) -> crate::operation::update_project::builders::UpdateProjectFluentBuilder {
        crate::operation::update_project::builders::UpdateProjectFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
