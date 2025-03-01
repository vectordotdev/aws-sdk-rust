// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateApplicationInstanceOutput {
    /// <p>The application instance's ID.</p>
    #[doc(hidden)]
    pub application_instance_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateApplicationInstanceOutput {
    /// <p>The application instance's ID.</p>
    pub fn application_instance_id(&self) -> ::std::option::Option<&str> {
        self.application_instance_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateApplicationInstanceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateApplicationInstanceOutput {
    /// Creates a new builder-style object to manufacture [`CreateApplicationInstanceOutput`](crate::operation::create_application_instance::CreateApplicationInstanceOutput).
    pub fn builder() -> crate::operation::create_application_instance::builders::CreateApplicationInstanceOutputBuilder{
        crate::operation::create_application_instance::builders::CreateApplicationInstanceOutputBuilder::default()
    }
}

/// A builder for [`CreateApplicationInstanceOutput`](crate::operation::create_application_instance::CreateApplicationInstanceOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateApplicationInstanceOutputBuilder {
    pub(crate) application_instance_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateApplicationInstanceOutputBuilder {
    /// <p>The application instance's ID.</p>
    pub fn application_instance_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The application instance's ID.</p>
    pub fn set_application_instance_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_instance_id = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateApplicationInstanceOutput`](crate::operation::create_application_instance::CreateApplicationInstanceOutput).
    pub fn build(
        self,
    ) -> crate::operation::create_application_instance::CreateApplicationInstanceOutput {
        crate::operation::create_application_instance::CreateApplicationInstanceOutput {
            application_instance_id: self.application_instance_id,
            _request_id: self._request_id,
        }
    }
}
