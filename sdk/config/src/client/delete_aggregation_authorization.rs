// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAggregationAuthorization`](crate::operation::delete_aggregation_authorization::builders::DeleteAggregationAuthorizationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`authorized_account_id(impl ::std::convert::Into<String>)`](crate::operation::delete_aggregation_authorization::builders::DeleteAggregationAuthorizationFluentBuilder::authorized_account_id) / [`set_authorized_account_id(Option<String>)`](crate::operation::delete_aggregation_authorization::builders::DeleteAggregationAuthorizationFluentBuilder::set_authorized_account_id): <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    ///   - [`authorized_aws_region(impl ::std::convert::Into<String>)`](crate::operation::delete_aggregation_authorization::builders::DeleteAggregationAuthorizationFluentBuilder::authorized_aws_region) / [`set_authorized_aws_region(Option<String>)`](crate::operation::delete_aggregation_authorization::builders::DeleteAggregationAuthorizationFluentBuilder::set_authorized_aws_region): <p>The region authorized to collect aggregated data.</p>
    /// - On success, responds with [`DeleteAggregationAuthorizationOutput`](crate::operation::delete_aggregation_authorization::DeleteAggregationAuthorizationOutput)
    /// - On failure, responds with [`SdkError<DeleteAggregationAuthorizationError>`](crate::operation::delete_aggregation_authorization::DeleteAggregationAuthorizationError)
    pub fn delete_aggregation_authorization(&self) -> crate::operation::delete_aggregation_authorization::builders::DeleteAggregationAuthorizationFluentBuilder{
        crate::operation::delete_aggregation_authorization::builders::DeleteAggregationAuthorizationFluentBuilder::new(self.handle.clone())
    }
}
