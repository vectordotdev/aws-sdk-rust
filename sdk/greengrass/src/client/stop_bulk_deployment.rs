// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopBulkDeployment`](crate::operation::stop_bulk_deployment::builders::StopBulkDeploymentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bulk_deployment_id(impl ::std::convert::Into<String>)`](crate::operation::stop_bulk_deployment::builders::StopBulkDeploymentFluentBuilder::bulk_deployment_id) / [`set_bulk_deployment_id(Option<String>)`](crate::operation::stop_bulk_deployment::builders::StopBulkDeploymentFluentBuilder::set_bulk_deployment_id): The ID of the bulk deployment.
    /// - On success, responds with [`StopBulkDeploymentOutput`](crate::operation::stop_bulk_deployment::StopBulkDeploymentOutput)
    /// - On failure, responds with [`SdkError<StopBulkDeploymentError>`](crate::operation::stop_bulk_deployment::StopBulkDeploymentError)
    pub fn stop_bulk_deployment(
        &self,
    ) -> crate::operation::stop_bulk_deployment::builders::StopBulkDeploymentFluentBuilder {
        crate::operation::stop_bulk_deployment::builders::StopBulkDeploymentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
