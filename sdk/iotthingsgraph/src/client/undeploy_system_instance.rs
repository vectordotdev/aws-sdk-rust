// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UndeploySystemInstance`](crate::operation::undeploy_system_instance::builders::UndeploySystemInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::undeploy_system_instance::builders::UndeploySystemInstanceFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::undeploy_system_instance::builders::UndeploySystemInstanceFluentBuilder::set_id): <p>The ID of the system instance to remove from its target.</p>
    /// - On success, responds with [`UndeploySystemInstanceOutput`](crate::operation::undeploy_system_instance::UndeploySystemInstanceOutput) with field(s):
    ///   - [`summary(Option<SystemInstanceSummary>)`](crate::operation::undeploy_system_instance::UndeploySystemInstanceOutput::summary): <p>An object that contains summary information about the system instance that was removed from its target.</p>
    /// - On failure, responds with [`SdkError<UndeploySystemInstanceError>`](crate::operation::undeploy_system_instance::UndeploySystemInstanceError)
    #[deprecated(note = "since: 2022-08-30")]
    pub fn undeploy_system_instance(
        &self,
    ) -> crate::operation::undeploy_system_instance::builders::UndeploySystemInstanceFluentBuilder
    {
        crate::operation::undeploy_system_instance::builders::UndeploySystemInstanceFluentBuilder::new(self.handle.clone())
    }
}
