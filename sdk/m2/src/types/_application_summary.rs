// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A subset of the possible application attributes. Used in the application list.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ApplicationSummary {
    /// <p>The name of the application.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The description of the application.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier of the application.</p>
    #[doc(hidden)]
    pub application_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[doc(hidden)]
    pub application_arn: ::std::option::Option<::std::string::String>,
    /// <p>The version of the application.</p>
    #[doc(hidden)]
    pub application_version: ::std::option::Option<i32>,
    /// <p>The status of the application.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::ApplicationLifecycle>,
    /// <p>The type of the target platform for this application.</p>
    #[doc(hidden)]
    pub engine_type: ::std::option::Option<crate::types::EngineType>,
    /// <p>The timestamp when the application was created.</p>
    #[doc(hidden)]
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The unique identifier of the runtime environment that hosts this application.</p>
    #[doc(hidden)]
    pub environment_id: ::std::option::Option<::std::string::String>,
    /// <p>The timestamp when you last started the application. Null until the application runs for the first time.</p>
    #[doc(hidden)]
    pub last_start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Indicates the status of the latest version of the application.</p>
    #[doc(hidden)]
    pub version_status: ::std::option::Option<crate::types::ApplicationVersionLifecycle>,
    /// <p>Indicates either an ongoing deployment or if the application has ever deployed successfully.</p>
    #[doc(hidden)]
    pub deployment_status: ::std::option::Option<crate::types::ApplicationDeploymentLifecycle>,
}
impl ApplicationSummary {
    /// <p>The name of the application.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The description of the application.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The unique identifier of the application.</p>
    pub fn application_id(&self) -> ::std::option::Option<&str> {
        self.application_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    pub fn application_arn(&self) -> ::std::option::Option<&str> {
        self.application_arn.as_deref()
    }
    /// <p>The version of the application.</p>
    pub fn application_version(&self) -> ::std::option::Option<i32> {
        self.application_version
    }
    /// <p>The status of the application.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ApplicationLifecycle> {
        self.status.as_ref()
    }
    /// <p>The type of the target platform for this application.</p>
    pub fn engine_type(&self) -> ::std::option::Option<&crate::types::EngineType> {
        self.engine_type.as_ref()
    }
    /// <p>The timestamp when the application was created.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The unique identifier of the runtime environment that hosts this application.</p>
    pub fn environment_id(&self) -> ::std::option::Option<&str> {
        self.environment_id.as_deref()
    }
    /// <p>The timestamp when you last started the application. Null until the application runs for the first time.</p>
    pub fn last_start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_start_time.as_ref()
    }
    /// <p>Indicates the status of the latest version of the application.</p>
    pub fn version_status(
        &self,
    ) -> ::std::option::Option<&crate::types::ApplicationVersionLifecycle> {
        self.version_status.as_ref()
    }
    /// <p>Indicates either an ongoing deployment or if the application has ever deployed successfully.</p>
    pub fn deployment_status(
        &self,
    ) -> ::std::option::Option<&crate::types::ApplicationDeploymentLifecycle> {
        self.deployment_status.as_ref()
    }
}
impl ApplicationSummary {
    /// Creates a new builder-style object to manufacture [`ApplicationSummary`](crate::types::ApplicationSummary).
    pub fn builder() -> crate::types::builders::ApplicationSummaryBuilder {
        crate::types::builders::ApplicationSummaryBuilder::default()
    }
}

/// A builder for [`ApplicationSummary`](crate::types::ApplicationSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ApplicationSummaryBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) application_id: ::std::option::Option<::std::string::String>,
    pub(crate) application_arn: ::std::option::Option<::std::string::String>,
    pub(crate) application_version: ::std::option::Option<i32>,
    pub(crate) status: ::std::option::Option<crate::types::ApplicationLifecycle>,
    pub(crate) engine_type: ::std::option::Option<crate::types::EngineType>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) environment_id: ::std::option::Option<::std::string::String>,
    pub(crate) last_start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) version_status: ::std::option::Option<crate::types::ApplicationVersionLifecycle>,
    pub(crate) deployment_status:
        ::std::option::Option<crate::types::ApplicationDeploymentLifecycle>,
}
impl ApplicationSummaryBuilder {
    /// <p>The name of the application.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the application.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The description of the application.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the application.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The unique identifier of the application.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the application.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    pub fn application_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    pub fn set_application_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_arn = input;
        self
    }
    /// <p>The version of the application.</p>
    pub fn application_version(mut self, input: i32) -> Self {
        self.application_version = ::std::option::Option::Some(input);
        self
    }
    /// <p>The version of the application.</p>
    pub fn set_application_version(mut self, input: ::std::option::Option<i32>) -> Self {
        self.application_version = input;
        self
    }
    /// <p>The status of the application.</p>
    pub fn status(mut self, input: crate::types::ApplicationLifecycle) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the application.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::ApplicationLifecycle>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The type of the target platform for this application.</p>
    pub fn engine_type(mut self, input: crate::types::EngineType) -> Self {
        self.engine_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the target platform for this application.</p>
    pub fn set_engine_type(
        mut self,
        input: ::std::option::Option<crate::types::EngineType>,
    ) -> Self {
        self.engine_type = input;
        self
    }
    /// <p>The timestamp when the application was created.</p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp when the application was created.</p>
    pub fn set_creation_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The unique identifier of the runtime environment that hosts this application.</p>
    pub fn environment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.environment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the runtime environment that hosts this application.</p>
    pub fn set_environment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.environment_id = input;
        self
    }
    /// <p>The timestamp when you last started the application. Null until the application runs for the first time.</p>
    pub fn last_start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp when you last started the application. Null until the application runs for the first time.</p>
    pub fn set_last_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_start_time = input;
        self
    }
    /// <p>Indicates the status of the latest version of the application.</p>
    pub fn version_status(mut self, input: crate::types::ApplicationVersionLifecycle) -> Self {
        self.version_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates the status of the latest version of the application.</p>
    pub fn set_version_status(
        mut self,
        input: ::std::option::Option<crate::types::ApplicationVersionLifecycle>,
    ) -> Self {
        self.version_status = input;
        self
    }
    /// <p>Indicates either an ongoing deployment or if the application has ever deployed successfully.</p>
    pub fn deployment_status(
        mut self,
        input: crate::types::ApplicationDeploymentLifecycle,
    ) -> Self {
        self.deployment_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates either an ongoing deployment or if the application has ever deployed successfully.</p>
    pub fn set_deployment_status(
        mut self,
        input: ::std::option::Option<crate::types::ApplicationDeploymentLifecycle>,
    ) -> Self {
        self.deployment_status = input;
        self
    }
    /// Consumes the builder and constructs a [`ApplicationSummary`](crate::types::ApplicationSummary).
    pub fn build(self) -> crate::types::ApplicationSummary {
        crate::types::ApplicationSummary {
            name: self.name,
            description: self.description,
            application_id: self.application_id,
            application_arn: self.application_arn,
            application_version: self.application_version,
            status: self.status,
            engine_type: self.engine_type,
            creation_time: self.creation_time,
            environment_id: self.environment_id,
            last_start_time: self.last_start_time,
            version_status: self.version_status,
            deployment_status: self.deployment_status,
        }
    }
}
