// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A load balancer attribute.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsElbv2LoadBalancerAttribute {
    /// <p>The name of the load balancer attribute.</p>
    #[doc(hidden)]
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>The value of the load balancer attribute.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl AwsElbv2LoadBalancerAttribute {
    /// <p>The name of the load balancer attribute.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>The value of the load balancer attribute.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl AwsElbv2LoadBalancerAttribute {
    /// Creates a new builder-style object to manufacture [`AwsElbv2LoadBalancerAttribute`](crate::types::AwsElbv2LoadBalancerAttribute).
    pub fn builder() -> crate::types::builders::AwsElbv2LoadBalancerAttributeBuilder {
        crate::types::builders::AwsElbv2LoadBalancerAttributeBuilder::default()
    }
}

/// A builder for [`AwsElbv2LoadBalancerAttribute`](crate::types::AwsElbv2LoadBalancerAttribute).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsElbv2LoadBalancerAttributeBuilder {
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl AwsElbv2LoadBalancerAttributeBuilder {
    /// <p>The name of the load balancer attribute.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the load balancer attribute.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>The value of the load balancer attribute.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value of the load balancer attribute.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsElbv2LoadBalancerAttribute`](crate::types::AwsElbv2LoadBalancerAttribute).
    pub fn build(self) -> crate::types::AwsElbv2LoadBalancerAttribute {
        crate::types::AwsElbv2LoadBalancerAttribute {
            key: self.key,
            value: self.value,
        }
    }
}
