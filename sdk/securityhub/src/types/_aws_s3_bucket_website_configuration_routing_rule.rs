// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A rule for redirecting requests to the website.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsS3BucketWebsiteConfigurationRoutingRule {
    /// <p>Provides the condition that must be met in order to apply the routing rule.</p>
    #[doc(hidden)]
    pub condition:
        ::std::option::Option<crate::types::AwsS3BucketWebsiteConfigurationRoutingRuleCondition>,
    /// <p>Provides the rules to redirect the request if the condition in <code>Condition</code> is met.</p>
    #[doc(hidden)]
    pub redirect:
        ::std::option::Option<crate::types::AwsS3BucketWebsiteConfigurationRoutingRuleRedirect>,
}
impl AwsS3BucketWebsiteConfigurationRoutingRule {
    /// <p>Provides the condition that must be met in order to apply the routing rule.</p>
    pub fn condition(
        &self,
    ) -> ::std::option::Option<&crate::types::AwsS3BucketWebsiteConfigurationRoutingRuleCondition>
    {
        self.condition.as_ref()
    }
    /// <p>Provides the rules to redirect the request if the condition in <code>Condition</code> is met.</p>
    pub fn redirect(
        &self,
    ) -> ::std::option::Option<&crate::types::AwsS3BucketWebsiteConfigurationRoutingRuleRedirect>
    {
        self.redirect.as_ref()
    }
}
impl AwsS3BucketWebsiteConfigurationRoutingRule {
    /// Creates a new builder-style object to manufacture [`AwsS3BucketWebsiteConfigurationRoutingRule`](crate::types::AwsS3BucketWebsiteConfigurationRoutingRule).
    pub fn builder() -> crate::types::builders::AwsS3BucketWebsiteConfigurationRoutingRuleBuilder {
        crate::types::builders::AwsS3BucketWebsiteConfigurationRoutingRuleBuilder::default()
    }
}

/// A builder for [`AwsS3BucketWebsiteConfigurationRoutingRule`](crate::types::AwsS3BucketWebsiteConfigurationRoutingRule).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsS3BucketWebsiteConfigurationRoutingRuleBuilder {
    pub(crate) condition:
        ::std::option::Option<crate::types::AwsS3BucketWebsiteConfigurationRoutingRuleCondition>,
    pub(crate) redirect:
        ::std::option::Option<crate::types::AwsS3BucketWebsiteConfigurationRoutingRuleRedirect>,
}
impl AwsS3BucketWebsiteConfigurationRoutingRuleBuilder {
    /// <p>Provides the condition that must be met in order to apply the routing rule.</p>
    pub fn condition(
        mut self,
        input: crate::types::AwsS3BucketWebsiteConfigurationRoutingRuleCondition,
    ) -> Self {
        self.condition = ::std::option::Option::Some(input);
        self
    }
    /// <p>Provides the condition that must be met in order to apply the routing rule.</p>
    pub fn set_condition(
        mut self,
        input: ::std::option::Option<
            crate::types::AwsS3BucketWebsiteConfigurationRoutingRuleCondition,
        >,
    ) -> Self {
        self.condition = input;
        self
    }
    /// <p>Provides the rules to redirect the request if the condition in <code>Condition</code> is met.</p>
    pub fn redirect(
        mut self,
        input: crate::types::AwsS3BucketWebsiteConfigurationRoutingRuleRedirect,
    ) -> Self {
        self.redirect = ::std::option::Option::Some(input);
        self
    }
    /// <p>Provides the rules to redirect the request if the condition in <code>Condition</code> is met.</p>
    pub fn set_redirect(
        mut self,
        input: ::std::option::Option<
            crate::types::AwsS3BucketWebsiteConfigurationRoutingRuleRedirect,
        >,
    ) -> Self {
        self.redirect = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsS3BucketWebsiteConfigurationRoutingRule`](crate::types::AwsS3BucketWebsiteConfigurationRoutingRule).
    pub fn build(self) -> crate::types::AwsS3BucketWebsiteConfigurationRoutingRule {
        crate::types::AwsS3BucketWebsiteConfigurationRoutingRule {
            condition: self.condition,
            redirect: self.redirect,
        }
    }
}
