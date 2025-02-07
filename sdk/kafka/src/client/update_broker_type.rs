// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateBrokerType`](crate::operation::update_broker_type::builders::UpdateBrokerTypeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_arn(impl ::std::convert::Into<String>)`](crate::operation::update_broker_type::builders::UpdateBrokerTypeFluentBuilder::cluster_arn) / [`set_cluster_arn(Option<String>)`](crate::operation::update_broker_type::builders::UpdateBrokerTypeFluentBuilder::set_cluster_arn): <p>The Amazon Resource Name (ARN) that uniquely identifies the cluster.</p>
    ///   - [`current_version(impl ::std::convert::Into<String>)`](crate::operation::update_broker_type::builders::UpdateBrokerTypeFluentBuilder::current_version) / [`set_current_version(Option<String>)`](crate::operation::update_broker_type::builders::UpdateBrokerTypeFluentBuilder::set_current_version): <p>The cluster version that you want to change. After this operation completes successfully, the cluster will have a new version.</p>
    ///   - [`target_instance_type(impl ::std::convert::Into<String>)`](crate::operation::update_broker_type::builders::UpdateBrokerTypeFluentBuilder::target_instance_type) / [`set_target_instance_type(Option<String>)`](crate::operation::update_broker_type::builders::UpdateBrokerTypeFluentBuilder::set_target_instance_type): <p>The Amazon MSK broker type that you want all of the brokers in this cluster to be.</p>
    /// - On success, responds with [`UpdateBrokerTypeOutput`](crate::operation::update_broker_type::UpdateBrokerTypeOutput) with field(s):
    ///   - [`cluster_arn(Option<String>)`](crate::operation::update_broker_type::UpdateBrokerTypeOutput::cluster_arn): <p>The Amazon Resource Name (ARN) of the cluster.</p>
    ///   - [`cluster_operation_arn(Option<String>)`](crate::operation::update_broker_type::UpdateBrokerTypeOutput::cluster_operation_arn): <p>The Amazon Resource Name (ARN) of the cluster operation.</p>
    /// - On failure, responds with [`SdkError<UpdateBrokerTypeError>`](crate::operation::update_broker_type::UpdateBrokerTypeError)
    pub fn update_broker_type(
        &self,
    ) -> crate::operation::update_broker_type::builders::UpdateBrokerTypeFluentBuilder {
        crate::operation::update_broker_type::builders::UpdateBrokerTypeFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
