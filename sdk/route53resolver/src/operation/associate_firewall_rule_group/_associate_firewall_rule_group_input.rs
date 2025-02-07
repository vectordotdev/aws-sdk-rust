// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateFirewallRuleGroupInput {
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp. </p>
    #[doc(hidden)]
    pub creator_request_id: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier of the firewall rule group. </p>
    #[doc(hidden)]
    pub firewall_rule_group_id: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier of the VPC that you want to associate with the rule group. </p>
    #[doc(hidden)]
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>The setting that determines the processing order of the rule group among the rule groups that you associate with the specified VPC. DNS Firewall filters VPC traffic starting from the rule group with the lowest numeric priority setting. </p>
    /// <p>You must specify a unique priority for each rule group that you associate with a single VPC. To make it easier to insert rule groups later, leave space between the numbers, for example, use 101, 200, and so on. You can change the priority setting for a rule group association after you create it.</p>
    /// <p>The allowed values for <code>Priority</code> are between 100 and 9900.</p>
    #[doc(hidden)]
    pub priority: ::std::option::Option<i32>,
    /// <p>A name that lets you identify the association, to manage and use it.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>If enabled, this setting disallows modification or removal of the association, to help prevent against accidentally altering DNS firewall protections. When you create the association, the default setting is <code>DISABLED</code>. </p>
    #[doc(hidden)]
    pub mutation_protection: ::std::option::Option<crate::types::MutationProtectionStatus>,
    /// <p>A list of the tag keys and values that you want to associate with the rule group association. </p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl AssociateFirewallRuleGroupInput {
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp. </p>
    pub fn creator_request_id(&self) -> ::std::option::Option<&str> {
        self.creator_request_id.as_deref()
    }
    /// <p>The unique identifier of the firewall rule group. </p>
    pub fn firewall_rule_group_id(&self) -> ::std::option::Option<&str> {
        self.firewall_rule_group_id.as_deref()
    }
    /// <p>The unique identifier of the VPC that you want to associate with the rule group. </p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>The setting that determines the processing order of the rule group among the rule groups that you associate with the specified VPC. DNS Firewall filters VPC traffic starting from the rule group with the lowest numeric priority setting. </p>
    /// <p>You must specify a unique priority for each rule group that you associate with a single VPC. To make it easier to insert rule groups later, leave space between the numbers, for example, use 101, 200, and so on. You can change the priority setting for a rule group association after you create it.</p>
    /// <p>The allowed values for <code>Priority</code> are between 100 and 9900.</p>
    pub fn priority(&self) -> ::std::option::Option<i32> {
        self.priority
    }
    /// <p>A name that lets you identify the association, to manage and use it.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>If enabled, this setting disallows modification or removal of the association, to help prevent against accidentally altering DNS firewall protections. When you create the association, the default setting is <code>DISABLED</code>. </p>
    pub fn mutation_protection(
        &self,
    ) -> ::std::option::Option<&crate::types::MutationProtectionStatus> {
        self.mutation_protection.as_ref()
    }
    /// <p>A list of the tag keys and values that you want to associate with the rule group association. </p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl AssociateFirewallRuleGroupInput {
    /// Creates a new builder-style object to manufacture [`AssociateFirewallRuleGroupInput`](crate::operation::associate_firewall_rule_group::AssociateFirewallRuleGroupInput).
    pub fn builder() -> crate::operation::associate_firewall_rule_group::builders::AssociateFirewallRuleGroupInputBuilder{
        crate::operation::associate_firewall_rule_group::builders::AssociateFirewallRuleGroupInputBuilder::default()
    }
}

/// A builder for [`AssociateFirewallRuleGroupInput`](crate::operation::associate_firewall_rule_group::AssociateFirewallRuleGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AssociateFirewallRuleGroupInputBuilder {
    pub(crate) creator_request_id: ::std::option::Option<::std::string::String>,
    pub(crate) firewall_rule_group_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) priority: ::std::option::Option<i32>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) mutation_protection: ::std::option::Option<crate::types::MutationProtectionStatus>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl AssociateFirewallRuleGroupInputBuilder {
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp. </p>
    pub fn creator_request_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.creator_request_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp. </p>
    pub fn set_creator_request_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.creator_request_id = input;
        self
    }
    /// <p>The unique identifier of the firewall rule group. </p>
    pub fn firewall_rule_group_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.firewall_rule_group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the firewall rule group. </p>
    pub fn set_firewall_rule_group_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.firewall_rule_group_id = input;
        self
    }
    /// <p>The unique identifier of the VPC that you want to associate with the rule group. </p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the VPC that you want to associate with the rule group. </p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// <p>The setting that determines the processing order of the rule group among the rule groups that you associate with the specified VPC. DNS Firewall filters VPC traffic starting from the rule group with the lowest numeric priority setting. </p>
    /// <p>You must specify a unique priority for each rule group that you associate with a single VPC. To make it easier to insert rule groups later, leave space between the numbers, for example, use 101, 200, and so on. You can change the priority setting for a rule group association after you create it.</p>
    /// <p>The allowed values for <code>Priority</code> are between 100 and 9900.</p>
    pub fn priority(mut self, input: i32) -> Self {
        self.priority = ::std::option::Option::Some(input);
        self
    }
    /// <p>The setting that determines the processing order of the rule group among the rule groups that you associate with the specified VPC. DNS Firewall filters VPC traffic starting from the rule group with the lowest numeric priority setting. </p>
    /// <p>You must specify a unique priority for each rule group that you associate with a single VPC. To make it easier to insert rule groups later, leave space between the numbers, for example, use 101, 200, and so on. You can change the priority setting for a rule group association after you create it.</p>
    /// <p>The allowed values for <code>Priority</code> are between 100 and 9900.</p>
    pub fn set_priority(mut self, input: ::std::option::Option<i32>) -> Self {
        self.priority = input;
        self
    }
    /// <p>A name that lets you identify the association, to manage and use it.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A name that lets you identify the association, to manage and use it.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>If enabled, this setting disallows modification or removal of the association, to help prevent against accidentally altering DNS firewall protections. When you create the association, the default setting is <code>DISABLED</code>. </p>
    pub fn mutation_protection(mut self, input: crate::types::MutationProtectionStatus) -> Self {
        self.mutation_protection = ::std::option::Option::Some(input);
        self
    }
    /// <p>If enabled, this setting disallows modification or removal of the association, to help prevent against accidentally altering DNS firewall protections. When you create the association, the default setting is <code>DISABLED</code>. </p>
    pub fn set_mutation_protection(
        mut self,
        input: ::std::option::Option<crate::types::MutationProtectionStatus>,
    ) -> Self {
        self.mutation_protection = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of the tag keys and values that you want to associate with the rule group association. </p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the tag keys and values that you want to associate with the rule group association. </p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociateFirewallRuleGroupInput`](crate::operation::associate_firewall_rule_group::AssociateFirewallRuleGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::associate_firewall_rule_group::AssociateFirewallRuleGroupInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::associate_firewall_rule_group::AssociateFirewallRuleGroupInput {
                creator_request_id: self.creator_request_id,
                firewall_rule_group_id: self.firewall_rule_group_id,
                vpc_id: self.vpc_id,
                priority: self.priority,
                name: self.name,
                mutation_protection: self.mutation_protection,
                tags: self.tags,
            },
        )
    }
}
