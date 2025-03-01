// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateAlias`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`alias_id(impl ::std::convert::Into<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::alias_id) / [`set_alias_id(Option<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_alias_id): <p>A unique identifier for the alias that you want to update. You can use either the alias ID or ARN value.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_name): <p>A descriptive label that is associated with an alias. Alias names do not need to be unique.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_description): <p>A human-readable description of the alias.</p>
    ///   - [`routing_strategy(RoutingStrategy)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::routing_strategy) / [`set_routing_strategy(Option<RoutingStrategy>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_routing_strategy): <p>The routing configuration, including routing type and fleet target, for the alias.</p>
    /// - On success, responds with [`UpdateAliasOutput`](crate::operation::update_alias::UpdateAliasOutput) with field(s):
    ///   - [`alias(Option<Alias>)`](crate::operation::update_alias::UpdateAliasOutput::alias): <p>The updated alias resource.</p>
    /// - On failure, responds with [`SdkError<UpdateAliasError>`](crate::operation::update_alias::UpdateAliasError)
    pub fn update_alias(
        &self,
    ) -> crate::operation::update_alias::builders::UpdateAliasFluentBuilder {
        crate::operation::update_alias::builders::UpdateAliasFluentBuilder::new(self.handle.clone())
    }
}
