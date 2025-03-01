// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteFirewallPolicy`](crate::operation::delete_firewall_policy::builders::DeleteFirewallPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`firewall_policy_name(impl ::std::convert::Into<String>)`](crate::operation::delete_firewall_policy::builders::DeleteFirewallPolicyFluentBuilder::firewall_policy_name) / [`set_firewall_policy_name(Option<String>)`](crate::operation::delete_firewall_policy::builders::DeleteFirewallPolicyFluentBuilder::set_firewall_policy_name): <p>The descriptive name of the firewall policy. You can't change the name of a firewall policy after you create it.</p>  <p>You must specify the ARN or the name, and you can specify both. </p>
    ///   - [`firewall_policy_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_firewall_policy::builders::DeleteFirewallPolicyFluentBuilder::firewall_policy_arn) / [`set_firewall_policy_arn(Option<String>)`](crate::operation::delete_firewall_policy::builders::DeleteFirewallPolicyFluentBuilder::set_firewall_policy_arn): <p>The Amazon Resource Name (ARN) of the firewall policy.</p>  <p>You must specify the ARN or the name, and you can specify both. </p>
    /// - On success, responds with [`DeleteFirewallPolicyOutput`](crate::operation::delete_firewall_policy::DeleteFirewallPolicyOutput) with field(s):
    ///   - [`firewall_policy_response(Option<FirewallPolicyResponse>)`](crate::operation::delete_firewall_policy::DeleteFirewallPolicyOutput::firewall_policy_response): <p>The object containing the definition of the <code>FirewallPolicyResponse</code> that you asked to delete. </p>
    /// - On failure, responds with [`SdkError<DeleteFirewallPolicyError>`](crate::operation::delete_firewall_policy::DeleteFirewallPolicyError)
    pub fn delete_firewall_policy(
        &self,
    ) -> crate::operation::delete_firewall_policy::builders::DeleteFirewallPolicyFluentBuilder {
        crate::operation::delete_firewall_policy::builders::DeleteFirewallPolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
