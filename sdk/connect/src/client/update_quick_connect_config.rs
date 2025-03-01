// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateQuickConnectConfig`](crate::operation::update_quick_connect_config::builders::UpdateQuickConnectConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::update_quick_connect_config::builders::UpdateQuickConnectConfigFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::update_quick_connect_config::builders::UpdateQuickConnectConfigFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`quick_connect_id(impl ::std::convert::Into<String>)`](crate::operation::update_quick_connect_config::builders::UpdateQuickConnectConfigFluentBuilder::quick_connect_id) / [`set_quick_connect_id(Option<String>)`](crate::operation::update_quick_connect_config::builders::UpdateQuickConnectConfigFluentBuilder::set_quick_connect_id): <p>The identifier for the quick connect.</p>
    ///   - [`quick_connect_config(QuickConnectConfig)`](crate::operation::update_quick_connect_config::builders::UpdateQuickConnectConfigFluentBuilder::quick_connect_config) / [`set_quick_connect_config(Option<QuickConnectConfig>)`](crate::operation::update_quick_connect_config::builders::UpdateQuickConnectConfigFluentBuilder::set_quick_connect_config): <p>Information about the configuration settings for the quick connect.</p>
    /// - On success, responds with [`UpdateQuickConnectConfigOutput`](crate::operation::update_quick_connect_config::UpdateQuickConnectConfigOutput)
    /// - On failure, responds with [`SdkError<UpdateQuickConnectConfigError>`](crate::operation::update_quick_connect_config::UpdateQuickConnectConfigError)
    pub fn update_quick_connect_config(&self) -> crate::operation::update_quick_connect_config::builders::UpdateQuickConnectConfigFluentBuilder{
        crate::operation::update_quick_connect_config::builders::UpdateQuickConnectConfigFluentBuilder::new(self.handle.clone())
    }
}
