// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AttachElasticLoadBalancerInput {
    /// <p>The Elastic Load Balancing instance's name.</p>
    #[doc(hidden)]
    pub elastic_load_balancer_name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the layer to which the Elastic Load Balancing instance is to be attached.</p>
    #[doc(hidden)]
    pub layer_id: ::std::option::Option<::std::string::String>,
}
impl AttachElasticLoadBalancerInput {
    /// <p>The Elastic Load Balancing instance's name.</p>
    pub fn elastic_load_balancer_name(&self) -> ::std::option::Option<&str> {
        self.elastic_load_balancer_name.as_deref()
    }
    /// <p>The ID of the layer to which the Elastic Load Balancing instance is to be attached.</p>
    pub fn layer_id(&self) -> ::std::option::Option<&str> {
        self.layer_id.as_deref()
    }
}
impl AttachElasticLoadBalancerInput {
    /// Creates a new builder-style object to manufacture [`AttachElasticLoadBalancerInput`](crate::operation::attach_elastic_load_balancer::AttachElasticLoadBalancerInput).
    pub fn builder() -> crate::operation::attach_elastic_load_balancer::builders::AttachElasticLoadBalancerInputBuilder{
        crate::operation::attach_elastic_load_balancer::builders::AttachElasticLoadBalancerInputBuilder::default()
    }
}

/// A builder for [`AttachElasticLoadBalancerInput`](crate::operation::attach_elastic_load_balancer::AttachElasticLoadBalancerInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AttachElasticLoadBalancerInputBuilder {
    pub(crate) elastic_load_balancer_name: ::std::option::Option<::std::string::String>,
    pub(crate) layer_id: ::std::option::Option<::std::string::String>,
}
impl AttachElasticLoadBalancerInputBuilder {
    /// <p>The Elastic Load Balancing instance's name.</p>
    pub fn elastic_load_balancer_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.elastic_load_balancer_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Elastic Load Balancing instance's name.</p>
    pub fn set_elastic_load_balancer_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.elastic_load_balancer_name = input;
        self
    }
    /// <p>The ID of the layer to which the Elastic Load Balancing instance is to be attached.</p>
    pub fn layer_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.layer_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the layer to which the Elastic Load Balancing instance is to be attached.</p>
    pub fn set_layer_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.layer_id = input;
        self
    }
    /// Consumes the builder and constructs a [`AttachElasticLoadBalancerInput`](crate::operation::attach_elastic_load_balancer::AttachElasticLoadBalancerInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::attach_elastic_load_balancer::AttachElasticLoadBalancerInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::attach_elastic_load_balancer::AttachElasticLoadBalancerInput {
                elastic_load_balancer_name: self.elastic_load_balancer_name,
                layer_id: self.layer_id,
            },
        )
    }
}
