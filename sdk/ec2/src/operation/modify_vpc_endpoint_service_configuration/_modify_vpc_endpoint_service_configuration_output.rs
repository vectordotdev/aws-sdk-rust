// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyVpcEndpointServiceConfigurationOutput {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    #[doc(hidden)]
    pub r#return: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl ModifyVpcEndpointServiceConfigurationOutput {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub fn r#return(&self) -> ::std::option::Option<bool> {
        self.r#return
    }
}
impl ::aws_http::request_id::RequestId for ModifyVpcEndpointServiceConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyVpcEndpointServiceConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`ModifyVpcEndpointServiceConfigurationOutput`](crate::operation::modify_vpc_endpoint_service_configuration::ModifyVpcEndpointServiceConfigurationOutput).
    pub fn builder() -> crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationOutputBuilder{
        crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationOutputBuilder::default()
    }
}

/// A builder for [`ModifyVpcEndpointServiceConfigurationOutput`](crate::operation::modify_vpc_endpoint_service_configuration::ModifyVpcEndpointServiceConfigurationOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ModifyVpcEndpointServiceConfigurationOutputBuilder {
    pub(crate) r#return: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl ModifyVpcEndpointServiceConfigurationOutputBuilder {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub fn r#return(mut self, input: bool) -> Self {
        self.r#return = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub fn set_return(mut self, input: ::std::option::Option<bool>) -> Self {
        self.r#return = input;
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
    /// Consumes the builder and constructs a [`ModifyVpcEndpointServiceConfigurationOutput`](crate::operation::modify_vpc_endpoint_service_configuration::ModifyVpcEndpointServiceConfigurationOutput).
    pub fn build(self) -> crate::operation::modify_vpc_endpoint_service_configuration::ModifyVpcEndpointServiceConfigurationOutput{
        crate::operation::modify_vpc_endpoint_service_configuration::ModifyVpcEndpointServiceConfigurationOutput {
            r#return: self.r#return
            ,
            _request_id: self._request_id,
        }
    }
}
