// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociatePricingRules`](crate::operation::associate_pricing_rules::builders::AssociatePricingRulesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl ::std::convert::Into<String>)`](crate::operation::associate_pricing_rules::builders::AssociatePricingRulesFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::associate_pricing_rules::builders::AssociatePricingRulesFluentBuilder::set_arn): <p> The <code>PricingPlanArn</code> that the <code>PricingRuleArns</code> are associated with. </p>
    ///   - [`pricing_rule_arns(Vec<String>)`](crate::operation::associate_pricing_rules::builders::AssociatePricingRulesFluentBuilder::pricing_rule_arns) / [`set_pricing_rule_arns(Option<Vec<String>>)`](crate::operation::associate_pricing_rules::builders::AssociatePricingRulesFluentBuilder::set_pricing_rule_arns): <p> The <code>PricingRuleArns</code> that are associated with the Pricing Plan. </p>
    /// - On success, responds with [`AssociatePricingRulesOutput`](crate::operation::associate_pricing_rules::AssociatePricingRulesOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::associate_pricing_rules::AssociatePricingRulesOutput::arn): <p> The <code>PricingPlanArn</code> that the <code>PricingRuleArns</code> are associated with. </p>
    /// - On failure, responds with [`SdkError<AssociatePricingRulesError>`](crate::operation::associate_pricing_rules::AssociatePricingRulesError)
    pub fn associate_pricing_rules(
        &self,
    ) -> crate::operation::associate_pricing_rules::builders::AssociatePricingRulesFluentBuilder
    {
        crate::operation::associate_pricing_rules::builders::AssociatePricingRulesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
