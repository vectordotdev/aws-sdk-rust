// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Summary data of an Proton component resource.</p>
/// <p>For more information about components, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/ag-components.html">Proton components</a> in the <i>Proton User Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ComponentSummary {
    /// <p>The name of the component.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the component.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the Proton environment that this component is associated with.</p>
    #[doc(hidden)]
    pub environment_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the service that <code>serviceInstanceName</code> is associated with. Provided when a component is attached to a service instance.</p>
    #[doc(hidden)]
    pub service_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the service instance that this component is attached to. Provided when a component is attached to a service instance.</p>
    #[doc(hidden)]
    pub service_instance_name: ::std::option::Option<::std::string::String>,
    /// <p>The time when the component was created.</p>
    #[doc(hidden)]
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time when the component was last modified.</p>
    #[doc(hidden)]
    pub last_modified_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time when a deployment of the component was last attempted.</p>
    #[doc(hidden)]
    pub last_deployment_attempted_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time when the component was last deployed successfully.</p>
    #[doc(hidden)]
    pub last_deployment_succeeded_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The component deployment status.</p>
    #[doc(hidden)]
    pub deployment_status: ::std::option::Option<crate::types::DeploymentStatus>,
    /// <p>The message associated with the component deployment status.</p>
    #[doc(hidden)]
    pub deployment_status_message: ::std::option::Option<::std::string::String>,
}
impl ComponentSummary {
    /// <p>The name of the component.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the component.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The name of the Proton environment that this component is associated with.</p>
    pub fn environment_name(&self) -> ::std::option::Option<&str> {
        self.environment_name.as_deref()
    }
    /// <p>The name of the service that <code>serviceInstanceName</code> is associated with. Provided when a component is attached to a service instance.</p>
    pub fn service_name(&self) -> ::std::option::Option<&str> {
        self.service_name.as_deref()
    }
    /// <p>The name of the service instance that this component is attached to. Provided when a component is attached to a service instance.</p>
    pub fn service_instance_name(&self) -> ::std::option::Option<&str> {
        self.service_instance_name.as_deref()
    }
    /// <p>The time when the component was created.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>The time when the component was last modified.</p>
    pub fn last_modified_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified_at.as_ref()
    }
    /// <p>The time when a deployment of the component was last attempted.</p>
    pub fn last_deployment_attempted_at(
        &self,
    ) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_deployment_attempted_at.as_ref()
    }
    /// <p>The time when the component was last deployed successfully.</p>
    pub fn last_deployment_succeeded_at(
        &self,
    ) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_deployment_succeeded_at.as_ref()
    }
    /// <p>The component deployment status.</p>
    pub fn deployment_status(&self) -> ::std::option::Option<&crate::types::DeploymentStatus> {
        self.deployment_status.as_ref()
    }
    /// <p>The message associated with the component deployment status.</p>
    pub fn deployment_status_message(&self) -> ::std::option::Option<&str> {
        self.deployment_status_message.as_deref()
    }
}
impl ::std::fmt::Debug for ComponentSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ComponentSummary");
        formatter.field("name", &self.name);
        formatter.field("arn", &self.arn);
        formatter.field("environment_name", &self.environment_name);
        formatter.field("service_name", &self.service_name);
        formatter.field("service_instance_name", &self.service_instance_name);
        formatter.field("created_at", &self.created_at);
        formatter.field("last_modified_at", &self.last_modified_at);
        formatter.field(
            "last_deployment_attempted_at",
            &self.last_deployment_attempted_at,
        );
        formatter.field(
            "last_deployment_succeeded_at",
            &self.last_deployment_succeeded_at,
        );
        formatter.field("deployment_status", &self.deployment_status);
        formatter.field(
            "deployment_status_message",
            &"*** Sensitive Data Redacted ***",
        );
        formatter.finish()
    }
}
impl ComponentSummary {
    /// Creates a new builder-style object to manufacture [`ComponentSummary`](crate::types::ComponentSummary).
    pub fn builder() -> crate::types::builders::ComponentSummaryBuilder {
        crate::types::builders::ComponentSummaryBuilder::default()
    }
}

/// A builder for [`ComponentSummary`](crate::types::ComponentSummary).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct ComponentSummaryBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) environment_name: ::std::option::Option<::std::string::String>,
    pub(crate) service_name: ::std::option::Option<::std::string::String>,
    pub(crate) service_instance_name: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_modified_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_deployment_attempted_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_deployment_succeeded_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) deployment_status: ::std::option::Option<crate::types::DeploymentStatus>,
    pub(crate) deployment_status_message: ::std::option::Option<::std::string::String>,
}
impl ComponentSummaryBuilder {
    /// <p>The name of the component.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the component.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the component.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the component.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The name of the Proton environment that this component is associated with.</p>
    pub fn environment_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.environment_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Proton environment that this component is associated with.</p>
    pub fn set_environment_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.environment_name = input;
        self
    }
    /// <p>The name of the service that <code>serviceInstanceName</code> is associated with. Provided when a component is attached to a service instance.</p>
    pub fn service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service that <code>serviceInstanceName</code> is associated with. Provided when a component is attached to a service instance.</p>
    pub fn set_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_name = input;
        self
    }
    /// <p>The name of the service instance that this component is attached to. Provided when a component is attached to a service instance.</p>
    pub fn service_instance_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.service_instance_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service instance that this component is attached to. Provided when a component is attached to a service instance.</p>
    pub fn set_service_instance_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.service_instance_name = input;
        self
    }
    /// <p>The time when the component was created.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when the component was created.</p>
    pub fn set_created_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_at = input;
        self
    }
    /// <p>The time when the component was last modified.</p>
    pub fn last_modified_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when the component was last modified.</p>
    pub fn set_last_modified_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_modified_at = input;
        self
    }
    /// <p>The time when a deployment of the component was last attempted.</p>
    pub fn last_deployment_attempted_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_deployment_attempted_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when a deployment of the component was last attempted.</p>
    pub fn set_last_deployment_attempted_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_deployment_attempted_at = input;
        self
    }
    /// <p>The time when the component was last deployed successfully.</p>
    pub fn last_deployment_succeeded_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_deployment_succeeded_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when the component was last deployed successfully.</p>
    pub fn set_last_deployment_succeeded_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_deployment_succeeded_at = input;
        self
    }
    /// <p>The component deployment status.</p>
    pub fn deployment_status(mut self, input: crate::types::DeploymentStatus) -> Self {
        self.deployment_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The component deployment status.</p>
    pub fn set_deployment_status(
        mut self,
        input: ::std::option::Option<crate::types::DeploymentStatus>,
    ) -> Self {
        self.deployment_status = input;
        self
    }
    /// <p>The message associated with the component deployment status.</p>
    pub fn deployment_status_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.deployment_status_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The message associated with the component deployment status.</p>
    pub fn set_deployment_status_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.deployment_status_message = input;
        self
    }
    /// Consumes the builder and constructs a [`ComponentSummary`](crate::types::ComponentSummary).
    pub fn build(self) -> crate::types::ComponentSummary {
        crate::types::ComponentSummary {
            name: self.name,
            arn: self.arn,
            environment_name: self.environment_name,
            service_name: self.service_name,
            service_instance_name: self.service_instance_name,
            created_at: self.created_at,
            last_modified_at: self.last_modified_at,
            last_deployment_attempted_at: self.last_deployment_attempted_at,
            last_deployment_succeeded_at: self.last_deployment_succeeded_at,
            deployment_status: self.deployment_status,
            deployment_status_message: self.deployment_status_message,
        }
    }
}
impl ::std::fmt::Debug for ComponentSummaryBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ComponentSummaryBuilder");
        formatter.field("name", &self.name);
        formatter.field("arn", &self.arn);
        formatter.field("environment_name", &self.environment_name);
        formatter.field("service_name", &self.service_name);
        formatter.field("service_instance_name", &self.service_instance_name);
        formatter.field("created_at", &self.created_at);
        formatter.field("last_modified_at", &self.last_modified_at);
        formatter.field(
            "last_deployment_attempted_at",
            &self.last_deployment_attempted_at,
        );
        formatter.field(
            "last_deployment_succeeded_at",
            &self.last_deployment_succeeded_at,
        );
        formatter.field("deployment_status", &self.deployment_status);
        formatter.field(
            "deployment_status_message",
            &"*** Sensitive Data Redacted ***",
        );
        formatter.finish()
    }
}
