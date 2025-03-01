// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteTarget`](crate::operation::delete_target::builders::DeleteTargetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`target_address(impl ::std::convert::Into<String>)`](crate::operation::delete_target::builders::DeleteTargetFluentBuilder::target_address) / [`set_target_address(Option<String>)`](crate::operation::delete_target::builders::DeleteTargetFluentBuilder::set_target_address): <p>The Amazon Resource Name (ARN) of the Chatbot topic or Chatbot client to delete.</p>
    ///   - [`force_unsubscribe_all(bool)`](crate::operation::delete_target::builders::DeleteTargetFluentBuilder::force_unsubscribe_all) / [`set_force_unsubscribe_all(bool)`](crate::operation::delete_target::builders::DeleteTargetFluentBuilder::set_force_unsubscribe_all): <p>A Boolean value that can be used to delete all associations with this Chatbot topic. The default value is FALSE. If set to TRUE, all associations between that target and every notification rule in your Amazon Web Services account are deleted.</p>
    /// - On success, responds with [`DeleteTargetOutput`](crate::operation::delete_target::DeleteTargetOutput)
    /// - On failure, responds with [`SdkError<DeleteTargetError>`](crate::operation::delete_target::DeleteTargetError)
    pub fn delete_target(
        &self,
    ) -> crate::operation::delete_target::builders::DeleteTargetFluentBuilder {
        crate::operation::delete_target::builders::DeleteTargetFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
