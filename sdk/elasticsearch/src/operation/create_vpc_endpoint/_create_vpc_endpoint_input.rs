// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for the parameters to the <code><code>CreateVpcEndpointRequest</code></code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateVpcEndpointInput {
    /// <p>The Amazon Resource Name (ARN) of the domain to grant access to.</p>
    #[doc(hidden)]
    pub domain_arn: ::std::option::Option<::std::string::String>,
    /// <p>Options to specify the subnets and security groups for the endpoint.</p>
    #[doc(hidden)]
    pub vpc_options: ::std::option::Option<crate::types::VpcOptions>,
    /// <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
}
impl CreateVpcEndpointInput {
    /// <p>The Amazon Resource Name (ARN) of the domain to grant access to.</p>
    pub fn domain_arn(&self) -> ::std::option::Option<&str> {
        self.domain_arn.as_deref()
    }
    /// <p>Options to specify the subnets and security groups for the endpoint.</p>
    pub fn vpc_options(&self) -> ::std::option::Option<&crate::types::VpcOptions> {
        self.vpc_options.as_ref()
    }
    /// <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
}
impl CreateVpcEndpointInput {
    /// Creates a new builder-style object to manufacture [`CreateVpcEndpointInput`](crate::operation::create_vpc_endpoint::CreateVpcEndpointInput).
    pub fn builder(
    ) -> crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointInputBuilder {
        crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointInputBuilder::default()
    }
}

/// A builder for [`CreateVpcEndpointInput`](crate::operation::create_vpc_endpoint::CreateVpcEndpointInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateVpcEndpointInputBuilder {
    pub(crate) domain_arn: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_options: ::std::option::Option<crate::types::VpcOptions>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
}
impl CreateVpcEndpointInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the domain to grant access to.</p>
    pub fn domain_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the domain to grant access to.</p>
    pub fn set_domain_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_arn = input;
        self
    }
    /// <p>Options to specify the subnets and security groups for the endpoint.</p>
    pub fn vpc_options(mut self, input: crate::types::VpcOptions) -> Self {
        self.vpc_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Options to specify the subnets and security groups for the endpoint.</p>
    pub fn set_vpc_options(
        mut self,
        input: ::std::option::Option<crate::types::VpcOptions>,
    ) -> Self {
        self.vpc_options = input;
        self
    }
    /// <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateVpcEndpointInput`](crate::operation::create_vpc_endpoint::CreateVpcEndpointInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_vpc_endpoint::CreateVpcEndpointInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_vpc_endpoint::CreateVpcEndpointInput {
                domain_arn: self.domain_arn,
                vpc_options: self.vpc_options,
                client_token: self.client_token,
            },
        )
    }
}
