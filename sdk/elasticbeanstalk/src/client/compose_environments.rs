// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ComposeEnvironments`](crate::operation::compose_environments::builders::ComposeEnvironmentsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_name(impl ::std::convert::Into<String>)`](crate::operation::compose_environments::builders::ComposeEnvironmentsFluentBuilder::application_name) / [`set_application_name(Option<String>)`](crate::operation::compose_environments::builders::ComposeEnvironmentsFluentBuilder::set_application_name): <p>The name of the application to which the specified source bundles belong.</p>
    ///   - [`group_name(impl ::std::convert::Into<String>)`](crate::operation::compose_environments::builders::ComposeEnvironmentsFluentBuilder::group_name) / [`set_group_name(Option<String>)`](crate::operation::compose_environments::builders::ComposeEnvironmentsFluentBuilder::set_group_name): <p>The name of the group to which the target environments belong. Specify a group name only if the environment name defined in each target environment's manifest ends with a + (plus) character. See <a href="https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/environment-cfg-manifest.html">Environment Manifest (env.yaml)</a> for details.</p>
    ///   - [`version_labels(Vec<String>)`](crate::operation::compose_environments::builders::ComposeEnvironmentsFluentBuilder::version_labels) / [`set_version_labels(Option<Vec<String>>)`](crate::operation::compose_environments::builders::ComposeEnvironmentsFluentBuilder::set_version_labels): <p>A list of version labels, specifying one or more application source bundles that belong to the target application. Each source bundle must include an environment manifest that specifies the name of the environment and the name of the solution stack to use, and optionally can specify environment links to create.</p>
    /// - On success, responds with [`ComposeEnvironmentsOutput`](crate::operation::compose_environments::ComposeEnvironmentsOutput) with field(s):
    ///   - [`environments(Option<Vec<EnvironmentDescription>>)`](crate::operation::compose_environments::ComposeEnvironmentsOutput::environments): <p> Returns an <code>EnvironmentDescription</code> list. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::compose_environments::ComposeEnvironmentsOutput::next_token): <p>In a paginated request, the token that you can pass in a subsequent request to get the next response page.</p>
    /// - On failure, responds with [`SdkError<ComposeEnvironmentsError>`](crate::operation::compose_environments::ComposeEnvironmentsError)
    pub fn compose_environments(
        &self,
    ) -> crate::operation::compose_environments::builders::ComposeEnvironmentsFluentBuilder {
        crate::operation::compose_environments::builders::ComposeEnvironmentsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
