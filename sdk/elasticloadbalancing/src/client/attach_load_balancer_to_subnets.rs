// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AttachLoadBalancerToSubnets`](crate::operation::attach_load_balancer_to_subnets::builders::AttachLoadBalancerToSubnetsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`load_balancer_name(impl ::std::convert::Into<String>)`](crate::operation::attach_load_balancer_to_subnets::builders::AttachLoadBalancerToSubnetsFluentBuilder::load_balancer_name) / [`set_load_balancer_name(Option<String>)`](crate::operation::attach_load_balancer_to_subnets::builders::AttachLoadBalancerToSubnetsFluentBuilder::set_load_balancer_name): <p>The name of the load balancer.</p>
    ///   - [`subnets(Vec<String>)`](crate::operation::attach_load_balancer_to_subnets::builders::AttachLoadBalancerToSubnetsFluentBuilder::subnets) / [`set_subnets(Option<Vec<String>>)`](crate::operation::attach_load_balancer_to_subnets::builders::AttachLoadBalancerToSubnetsFluentBuilder::set_subnets): <p>The IDs of the subnets to add. You can add only one subnet per Availability Zone.</p>
    /// - On success, responds with [`AttachLoadBalancerToSubnetsOutput`](crate::operation::attach_load_balancer_to_subnets::AttachLoadBalancerToSubnetsOutput) with field(s):
    ///   - [`subnets(Option<Vec<String>>)`](crate::operation::attach_load_balancer_to_subnets::AttachLoadBalancerToSubnetsOutput::subnets): <p>The IDs of the subnets attached to the load balancer.</p>
    /// - On failure, responds with [`SdkError<AttachLoadBalancerToSubnetsError>`](crate::operation::attach_load_balancer_to_subnets::AttachLoadBalancerToSubnetsError)
    pub fn attach_load_balancer_to_subnets(&self) -> crate::operation::attach_load_balancer_to_subnets::builders::AttachLoadBalancerToSubnetsFluentBuilder{
        crate::operation::attach_load_balancer_to_subnets::builders::AttachLoadBalancerToSubnetsFluentBuilder::new(self.handle.clone())
    }
}
