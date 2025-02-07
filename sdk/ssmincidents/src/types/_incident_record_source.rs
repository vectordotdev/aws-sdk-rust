// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about what created the incident record and when it was created.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IncidentRecordSource {
    /// <p>The principal that started the incident.</p>
    #[doc(hidden)]
    pub created_by: ::std::option::Option<::std::string::String>,
    /// <p>The service principal that assumed the role specified in <code>createdBy</code>. If no service principal assumed the role this will be left blank.</p>
    #[doc(hidden)]
    pub invoked_by: ::std::option::Option<::std::string::String>,
    /// <p>The resource that caused the incident to be created.</p>
    #[doc(hidden)]
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p>The service that started the incident. This can be manually created from Incident Manager, automatically created using an Amazon CloudWatch alarm, or Amazon EventBridge event.</p>
    #[doc(hidden)]
    pub source: ::std::option::Option<::std::string::String>,
}
impl IncidentRecordSource {
    /// <p>The principal that started the incident.</p>
    pub fn created_by(&self) -> ::std::option::Option<&str> {
        self.created_by.as_deref()
    }
    /// <p>The service principal that assumed the role specified in <code>createdBy</code>. If no service principal assumed the role this will be left blank.</p>
    pub fn invoked_by(&self) -> ::std::option::Option<&str> {
        self.invoked_by.as_deref()
    }
    /// <p>The resource that caused the incident to be created.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>The service that started the incident. This can be manually created from Incident Manager, automatically created using an Amazon CloudWatch alarm, or Amazon EventBridge event.</p>
    pub fn source(&self) -> ::std::option::Option<&str> {
        self.source.as_deref()
    }
}
impl IncidentRecordSource {
    /// Creates a new builder-style object to manufacture [`IncidentRecordSource`](crate::types::IncidentRecordSource).
    pub fn builder() -> crate::types::builders::IncidentRecordSourceBuilder {
        crate::types::builders::IncidentRecordSourceBuilder::default()
    }
}

/// A builder for [`IncidentRecordSource`](crate::types::IncidentRecordSource).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct IncidentRecordSourceBuilder {
    pub(crate) created_by: ::std::option::Option<::std::string::String>,
    pub(crate) invoked_by: ::std::option::Option<::std::string::String>,
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) source: ::std::option::Option<::std::string::String>,
}
impl IncidentRecordSourceBuilder {
    /// <p>The principal that started the incident.</p>
    pub fn created_by(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.created_by = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The principal that started the incident.</p>
    pub fn set_created_by(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.created_by = input;
        self
    }
    /// <p>The service principal that assumed the role specified in <code>createdBy</code>. If no service principal assumed the role this will be left blank.</p>
    pub fn invoked_by(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.invoked_by = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The service principal that assumed the role specified in <code>createdBy</code>. If no service principal assumed the role this will be left blank.</p>
    pub fn set_invoked_by(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.invoked_by = input;
        self
    }
    /// <p>The resource that caused the incident to be created.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The resource that caused the incident to be created.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// <p>The service that started the incident. This can be manually created from Incident Manager, automatically created using an Amazon CloudWatch alarm, or Amazon EventBridge event.</p>
    pub fn source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The service that started the incident. This can be manually created from Incident Manager, automatically created using an Amazon CloudWatch alarm, or Amazon EventBridge event.</p>
    pub fn set_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source = input;
        self
    }
    /// Consumes the builder and constructs a [`IncidentRecordSource`](crate::types::IncidentRecordSource).
    pub fn build(self) -> crate::types::IncidentRecordSource {
        crate::types::IncidentRecordSource {
            created_by: self.created_by,
            invoked_by: self.invoked_by,
            resource_arn: self.resource_arn,
            source: self.source,
        }
    }
}
