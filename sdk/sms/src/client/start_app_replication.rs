// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartAppReplication`](crate::operation::start_app_replication::builders::StartAppReplicationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl ::std::convert::Into<String>)`](crate::operation::start_app_replication::builders::StartAppReplicationFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::start_app_replication::builders::StartAppReplicationFluentBuilder::set_app_id): <p>The ID of the application.</p>
    /// - On success, responds with [`StartAppReplicationOutput`](crate::operation::start_app_replication::StartAppReplicationOutput)
    /// - On failure, responds with [`SdkError<StartAppReplicationError>`](crate::operation::start_app_replication::StartAppReplicationError)
    pub fn start_app_replication(
        &self,
    ) -> crate::operation::start_app_replication::builders::StartAppReplicationFluentBuilder {
        crate::operation::start_app_replication::builders::StartAppReplicationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
