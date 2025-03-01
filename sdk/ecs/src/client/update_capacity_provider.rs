// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateCapacityProvider`](crate::operation::update_capacity_provider::builders::UpdateCapacityProviderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_capacity_provider::builders::UpdateCapacityProviderFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_capacity_provider::builders::UpdateCapacityProviderFluentBuilder::set_name): <p>The name of the capacity provider to update.</p>
    ///   - [`auto_scaling_group_provider(AutoScalingGroupProviderUpdate)`](crate::operation::update_capacity_provider::builders::UpdateCapacityProviderFluentBuilder::auto_scaling_group_provider) / [`set_auto_scaling_group_provider(Option<AutoScalingGroupProviderUpdate>)`](crate::operation::update_capacity_provider::builders::UpdateCapacityProviderFluentBuilder::set_auto_scaling_group_provider): <p>An object that represent the parameters to update for the Auto Scaling group capacity provider.</p>
    /// - On success, responds with [`UpdateCapacityProviderOutput`](crate::operation::update_capacity_provider::UpdateCapacityProviderOutput) with field(s):
    ///   - [`capacity_provider(Option<CapacityProvider>)`](crate::operation::update_capacity_provider::UpdateCapacityProviderOutput::capacity_provider): <p>Details about the capacity provider.</p>
    /// - On failure, responds with [`SdkError<UpdateCapacityProviderError>`](crate::operation::update_capacity_provider::UpdateCapacityProviderError)
    pub fn update_capacity_provider(
        &self,
    ) -> crate::operation::update_capacity_provider::builders::UpdateCapacityProviderFluentBuilder
    {
        crate::operation::update_capacity_provider::builders::UpdateCapacityProviderFluentBuilder::new(self.handle.clone())
    }
}
