// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeregisterRdsDbInstance`](crate::operation::deregister_rds_db_instance::builders::DeregisterRdsDbInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`rds_db_instance_arn(impl ::std::convert::Into<String>)`](crate::operation::deregister_rds_db_instance::builders::DeregisterRdsDbInstanceFluentBuilder::rds_db_instance_arn) / [`set_rds_db_instance_arn(Option<String>)`](crate::operation::deregister_rds_db_instance::builders::DeregisterRdsDbInstanceFluentBuilder::set_rds_db_instance_arn): <p>The Amazon RDS instance's ARN.</p>
    /// - On success, responds with [`DeregisterRdsDbInstanceOutput`](crate::operation::deregister_rds_db_instance::DeregisterRdsDbInstanceOutput)
    /// - On failure, responds with [`SdkError<DeregisterRdsDbInstanceError>`](crate::operation::deregister_rds_db_instance::DeregisterRdsDbInstanceError)
    pub fn deregister_rds_db_instance(
        &self,
    ) -> crate::operation::deregister_rds_db_instance::builders::DeregisterRdsDbInstanceFluentBuilder
    {
        crate::operation::deregister_rds_db_instance::builders::DeregisterRdsDbInstanceFluentBuilder::new(self.handle.clone())
    }
}
