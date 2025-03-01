// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetTopicRule`](crate::operation::get_topic_rule::builders::GetTopicRuleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`rule_name(impl ::std::convert::Into<String>)`](crate::operation::get_topic_rule::builders::GetTopicRuleFluentBuilder::rule_name) / [`set_rule_name(Option<String>)`](crate::operation::get_topic_rule::builders::GetTopicRuleFluentBuilder::set_rule_name): <p>The name of the rule.</p>
    /// - On success, responds with [`GetTopicRuleOutput`](crate::operation::get_topic_rule::GetTopicRuleOutput) with field(s):
    ///   - [`rule_arn(Option<String>)`](crate::operation::get_topic_rule::GetTopicRuleOutput::rule_arn): <p>The rule ARN.</p>
    ///   - [`rule(Option<TopicRule>)`](crate::operation::get_topic_rule::GetTopicRuleOutput::rule): <p>The rule.</p>
    /// - On failure, responds with [`SdkError<GetTopicRuleError>`](crate::operation::get_topic_rule::GetTopicRuleError)
    pub fn get_topic_rule(
        &self,
    ) -> crate::operation::get_topic_rule::builders::GetTopicRuleFluentBuilder {
        crate::operation::get_topic_rule::builders::GetTopicRuleFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
