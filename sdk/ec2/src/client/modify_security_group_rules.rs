// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifySecurityGroupRules`](crate::operation::modify_security_group_rules::builders::ModifySecurityGroupRulesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`group_id(impl ::std::convert::Into<String>)`](crate::operation::modify_security_group_rules::builders::ModifySecurityGroupRulesFluentBuilder::group_id) / [`set_group_id(Option<String>)`](crate::operation::modify_security_group_rules::builders::ModifySecurityGroupRulesFluentBuilder::set_group_id): <p>The ID of the security group.</p>
    ///   - [`security_group_rules(Vec<SecurityGroupRuleUpdate>)`](crate::operation::modify_security_group_rules::builders::ModifySecurityGroupRulesFluentBuilder::security_group_rules) / [`set_security_group_rules(Option<Vec<SecurityGroupRuleUpdate>>)`](crate::operation::modify_security_group_rules::builders::ModifySecurityGroupRulesFluentBuilder::set_security_group_rules): <p>Information about the security group properties to update.</p>
    ///   - [`dry_run(bool)`](crate::operation::modify_security_group_rules::builders::ModifySecurityGroupRulesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_security_group_rules::builders::ModifySecurityGroupRulesFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`ModifySecurityGroupRulesOutput`](crate::operation::modify_security_group_rules::ModifySecurityGroupRulesOutput) with field(s):
    ///   - [`r#return(Option<bool>)`](crate::operation::modify_security_group_rules::ModifySecurityGroupRulesOutput::return): <p>Returns <code>true</code> if the request succeeds; otherwise, returns an error.</p>
    /// - On failure, responds with [`SdkError<ModifySecurityGroupRulesError>`](crate::operation::modify_security_group_rules::ModifySecurityGroupRulesError)
    pub fn modify_security_group_rules(&self) -> crate::operation::modify_security_group_rules::builders::ModifySecurityGroupRulesFluentBuilder{
        crate::operation::modify_security_group_rules::builders::ModifySecurityGroupRulesFluentBuilder::new(self.handle.clone())
    }
}
