// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Summary data of an Proton service resource.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ServiceSummary {
    /// <p>The name of the service.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>A description of the service.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the service.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the service template.</p>
    #[doc(hidden)]
    pub template_name: ::std::option::Option<::std::string::String>,
    /// <p>The time when the service was created.</p>
    #[doc(hidden)]
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time when the service was last modified.</p>
    #[doc(hidden)]
    pub last_modified_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The status of the service.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::ServiceStatus>,
    /// <p>A service status message.</p>
    #[doc(hidden)]
    pub status_message: ::std::option::Option<::std::string::String>,
}
impl ServiceSummary {
    /// <p>The name of the service.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>A description of the service.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the service.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The name of the service template.</p>
    pub fn template_name(&self) -> ::std::option::Option<&str> {
        self.template_name.as_deref()
    }
    /// <p>The time when the service was created.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>The time when the service was last modified.</p>
    pub fn last_modified_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified_at.as_ref()
    }
    /// <p>The status of the service.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ServiceStatus> {
        self.status.as_ref()
    }
    /// <p>A service status message.</p>
    pub fn status_message(&self) -> ::std::option::Option<&str> {
        self.status_message.as_deref()
    }
}
impl ::std::fmt::Debug for ServiceSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ServiceSummary");
        formatter.field("name", &self.name);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("arn", &self.arn);
        formatter.field("template_name", &self.template_name);
        formatter.field("created_at", &self.created_at);
        formatter.field("last_modified_at", &self.last_modified_at);
        formatter.field("status", &self.status);
        formatter.field("status_message", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl ServiceSummary {
    /// Creates a new builder-style object to manufacture [`ServiceSummary`](crate::types::ServiceSummary).
    pub fn builder() -> crate::types::builders::ServiceSummaryBuilder {
        crate::types::builders::ServiceSummaryBuilder::default()
    }
}

/// A builder for [`ServiceSummary`](crate::types::ServiceSummary).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct ServiceSummaryBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) template_name: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_modified_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) status: ::std::option::Option<crate::types::ServiceStatus>,
    pub(crate) status_message: ::std::option::Option<::std::string::String>,
}
impl ServiceSummaryBuilder {
    /// <p>The name of the service.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A description of the service.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the service.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the service.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the service.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The name of the service template.</p>
    pub fn template_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.template_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service template.</p>
    pub fn set_template_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.template_name = input;
        self
    }
    /// <p>The time when the service was created.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when the service was created.</p>
    pub fn set_created_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_at = input;
        self
    }
    /// <p>The time when the service was last modified.</p>
    pub fn last_modified_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when the service was last modified.</p>
    pub fn set_last_modified_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_modified_at = input;
        self
    }
    /// <p>The status of the service.</p>
    pub fn status(mut self, input: crate::types::ServiceStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the service.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ServiceStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>A service status message.</p>
    pub fn status_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.status_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A service status message.</p>
    pub fn set_status_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.status_message = input;
        self
    }
    /// Consumes the builder and constructs a [`ServiceSummary`](crate::types::ServiceSummary).
    pub fn build(self) -> crate::types::ServiceSummary {
        crate::types::ServiceSummary {
            name: self.name,
            description: self.description,
            arn: self.arn,
            template_name: self.template_name,
            created_at: self.created_at,
            last_modified_at: self.last_modified_at,
            status: self.status,
            status_message: self.status_message,
        }
    }
}
impl ::std::fmt::Debug for ServiceSummaryBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ServiceSummaryBuilder");
        formatter.field("name", &self.name);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("arn", &self.arn);
        formatter.field("template_name", &self.template_name);
        formatter.field("created_at", &self.created_at);
        formatter.field("last_modified_at", &self.last_modified_at);
        formatter.field("status", &self.status);
        formatter.field("status_message", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
