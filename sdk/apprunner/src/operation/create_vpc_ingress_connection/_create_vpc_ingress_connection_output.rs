// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateVpcIngressConnectionOutput {
    /// <p>A description of the App Runner VPC Ingress Connection resource that's created by this request. </p>
    #[doc(hidden)]
    pub vpc_ingress_connection: ::std::option::Option<crate::types::VpcIngressConnection>,
    _request_id: Option<String>,
}
impl CreateVpcIngressConnectionOutput {
    /// <p>A description of the App Runner VPC Ingress Connection resource that's created by this request. </p>
    pub fn vpc_ingress_connection(
        &self,
    ) -> ::std::option::Option<&crate::types::VpcIngressConnection> {
        self.vpc_ingress_connection.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for CreateVpcIngressConnectionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateVpcIngressConnectionOutput {
    /// Creates a new builder-style object to manufacture [`CreateVpcIngressConnectionOutput`](crate::operation::create_vpc_ingress_connection::CreateVpcIngressConnectionOutput).
    pub fn builder() -> crate::operation::create_vpc_ingress_connection::builders::CreateVpcIngressConnectionOutputBuilder{
        crate::operation::create_vpc_ingress_connection::builders::CreateVpcIngressConnectionOutputBuilder::default()
    }
}

/// A builder for [`CreateVpcIngressConnectionOutput`](crate::operation::create_vpc_ingress_connection::CreateVpcIngressConnectionOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateVpcIngressConnectionOutputBuilder {
    pub(crate) vpc_ingress_connection: ::std::option::Option<crate::types::VpcIngressConnection>,
    _request_id: Option<String>,
}
impl CreateVpcIngressConnectionOutputBuilder {
    /// <p>A description of the App Runner VPC Ingress Connection resource that's created by this request. </p>
    pub fn vpc_ingress_connection(mut self, input: crate::types::VpcIngressConnection) -> Self {
        self.vpc_ingress_connection = ::std::option::Option::Some(input);
        self
    }
    /// <p>A description of the App Runner VPC Ingress Connection resource that's created by this request. </p>
    pub fn set_vpc_ingress_connection(
        mut self,
        input: ::std::option::Option<crate::types::VpcIngressConnection>,
    ) -> Self {
        self.vpc_ingress_connection = input;
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
    /// Consumes the builder and constructs a [`CreateVpcIngressConnectionOutput`](crate::operation::create_vpc_ingress_connection::CreateVpcIngressConnectionOutput).
    pub fn build(
        self,
    ) -> crate::operation::create_vpc_ingress_connection::CreateVpcIngressConnectionOutput {
        crate::operation::create_vpc_ingress_connection::CreateVpcIngressConnectionOutput {
            vpc_ingress_connection: self.vpc_ingress_connection,
            _request_id: self._request_id,
        }
    }
}
