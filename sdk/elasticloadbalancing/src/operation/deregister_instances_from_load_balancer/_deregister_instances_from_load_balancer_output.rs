// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the output of DeregisterInstancesFromLoadBalancer.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeregisterInstancesFromLoadBalancerOutput {
    /// <p>The remaining instances registered with the load balancer.</p>
    #[doc(hidden)]
    pub instances: ::std::option::Option<::std::vec::Vec<crate::types::Instance>>,
    _request_id: Option<String>,
}
impl DeregisterInstancesFromLoadBalancerOutput {
    /// <p>The remaining instances registered with the load balancer.</p>
    pub fn instances(&self) -> ::std::option::Option<&[crate::types::Instance]> {
        self.instances.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DeregisterInstancesFromLoadBalancerOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeregisterInstancesFromLoadBalancerOutput {
    /// Creates a new builder-style object to manufacture [`DeregisterInstancesFromLoadBalancerOutput`](crate::operation::deregister_instances_from_load_balancer::DeregisterInstancesFromLoadBalancerOutput).
    pub fn builder() -> crate::operation::deregister_instances_from_load_balancer::builders::DeregisterInstancesFromLoadBalancerOutputBuilder{
        crate::operation::deregister_instances_from_load_balancer::builders::DeregisterInstancesFromLoadBalancerOutputBuilder::default()
    }
}

/// A builder for [`DeregisterInstancesFromLoadBalancerOutput`](crate::operation::deregister_instances_from_load_balancer::DeregisterInstancesFromLoadBalancerOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeregisterInstancesFromLoadBalancerOutputBuilder {
    pub(crate) instances: ::std::option::Option<::std::vec::Vec<crate::types::Instance>>,
    _request_id: Option<String>,
}
impl DeregisterInstancesFromLoadBalancerOutputBuilder {
    /// Appends an item to `instances`.
    ///
    /// To override the contents of this collection use [`set_instances`](Self::set_instances).
    ///
    /// <p>The remaining instances registered with the load balancer.</p>
    pub fn instances(mut self, input: crate::types::Instance) -> Self {
        let mut v = self.instances.unwrap_or_default();
        v.push(input);
        self.instances = ::std::option::Option::Some(v);
        self
    }
    /// <p>The remaining instances registered with the load balancer.</p>
    pub fn set_instances(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Instance>>,
    ) -> Self {
        self.instances = input;
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
    /// Consumes the builder and constructs a [`DeregisterInstancesFromLoadBalancerOutput`](crate::operation::deregister_instances_from_load_balancer::DeregisterInstancesFromLoadBalancerOutput).
    pub fn build(self) -> crate::operation::deregister_instances_from_load_balancer::DeregisterInstancesFromLoadBalancerOutput{
        crate::operation::deregister_instances_from_load_balancer::DeregisterInstancesFromLoadBalancerOutput {
            instances: self.instances
            ,
            _request_id: self._request_id,
        }
    }
}
