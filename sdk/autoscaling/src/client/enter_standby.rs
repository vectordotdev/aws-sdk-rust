// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnterStandby`](crate::operation::enter_standby::builders::EnterStandbyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_ids(Vec<String>)`](crate::operation::enter_standby::builders::EnterStandbyFluentBuilder::instance_ids) / [`set_instance_ids(Option<Vec<String>>)`](crate::operation::enter_standby::builders::EnterStandbyFluentBuilder::set_instance_ids): <p>The IDs of the instances. You can specify up to 20 instances.</p>
    ///   - [`auto_scaling_group_name(impl ::std::convert::Into<String>)`](crate::operation::enter_standby::builders::EnterStandbyFluentBuilder::auto_scaling_group_name) / [`set_auto_scaling_group_name(Option<String>)`](crate::operation::enter_standby::builders::EnterStandbyFluentBuilder::set_auto_scaling_group_name): <p>The name of the Auto Scaling group.</p>
    ///   - [`should_decrement_desired_capacity(bool)`](crate::operation::enter_standby::builders::EnterStandbyFluentBuilder::should_decrement_desired_capacity) / [`set_should_decrement_desired_capacity(Option<bool>)`](crate::operation::enter_standby::builders::EnterStandbyFluentBuilder::set_should_decrement_desired_capacity): <p>Indicates whether to decrement the desired capacity of the Auto Scaling group by the number of instances moved to <code>Standby</code> mode.</p>
    /// - On success, responds with [`EnterStandbyOutput`](crate::operation::enter_standby::EnterStandbyOutput) with field(s):
    ///   - [`activities(Option<Vec<Activity>>)`](crate::operation::enter_standby::EnterStandbyOutput::activities): <p>The activities related to moving instances into <code>Standby</code> mode.</p>
    /// - On failure, responds with [`SdkError<EnterStandbyError>`](crate::operation::enter_standby::EnterStandbyError)
    pub fn enter_standby(
        &self,
    ) -> crate::operation::enter_standby::builders::EnterStandbyFluentBuilder {
        crate::operation::enter_standby::builders::EnterStandbyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
