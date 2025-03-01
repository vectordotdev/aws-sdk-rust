// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateRoutingProfileConcurrency`](crate::operation::update_routing_profile_concurrency::builders::UpdateRoutingProfileConcurrencyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::update_routing_profile_concurrency::builders::UpdateRoutingProfileConcurrencyFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::update_routing_profile_concurrency::builders::UpdateRoutingProfileConcurrencyFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`routing_profile_id(impl ::std::convert::Into<String>)`](crate::operation::update_routing_profile_concurrency::builders::UpdateRoutingProfileConcurrencyFluentBuilder::routing_profile_id) / [`set_routing_profile_id(Option<String>)`](crate::operation::update_routing_profile_concurrency::builders::UpdateRoutingProfileConcurrencyFluentBuilder::set_routing_profile_id): <p>The identifier of the routing profile.</p>
    ///   - [`media_concurrencies(Vec<MediaConcurrency>)`](crate::operation::update_routing_profile_concurrency::builders::UpdateRoutingProfileConcurrencyFluentBuilder::media_concurrencies) / [`set_media_concurrencies(Option<Vec<MediaConcurrency>>)`](crate::operation::update_routing_profile_concurrency::builders::UpdateRoutingProfileConcurrencyFluentBuilder::set_media_concurrencies): <p>The channels that agents can handle in the Contact Control Panel (CCP).</p>
    /// - On success, responds with [`UpdateRoutingProfileConcurrencyOutput`](crate::operation::update_routing_profile_concurrency::UpdateRoutingProfileConcurrencyOutput)
    /// - On failure, responds with [`SdkError<UpdateRoutingProfileConcurrencyError>`](crate::operation::update_routing_profile_concurrency::UpdateRoutingProfileConcurrencyError)
    pub fn update_routing_profile_concurrency(&self) -> crate::operation::update_routing_profile_concurrency::builders::UpdateRoutingProfileConcurrencyFluentBuilder{
        crate::operation::update_routing_profile_concurrency::builders::UpdateRoutingProfileConcurrencyFluentBuilder::new(self.handle.clone())
    }
}
