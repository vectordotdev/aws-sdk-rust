// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListInstances`](crate::operation::list_instances::builders::ListInstancesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`service_id(impl ::std::convert::Into<String>)`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::service_id) / [`set_service_id(Option<String>)`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::set_service_id): <p>The ID of the service that you want to list instances for.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::set_next_token): <p>For the first <code>ListInstances</code> request, omit this value.</p>  <p>If more than <code>MaxResults</code> instances match the specified criteria, you can submit another <code>ListInstances</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p>
    ///   - [`max_results(i32)`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_instances::builders::ListInstancesFluentBuilder::set_max_results): <p>The maximum number of instances that you want Cloud Map to return in the response to a <code>ListInstances</code> request. If you don't specify a value for <code>MaxResults</code>, Cloud Map returns up to 100 instances.</p>
    /// - On success, responds with [`ListInstancesOutput`](crate::operation::list_instances::ListInstancesOutput) with field(s):
    ///   - [`instances(Option<Vec<InstanceSummary>>)`](crate::operation::list_instances::ListInstancesOutput::instances): <p>Summary information about the instances that are associated with the specified service.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_instances::ListInstancesOutput::next_token): <p>If more than <code>MaxResults</code> instances match the specified criteria, you can submit another <code>ListInstances</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p>
    /// - On failure, responds with [`SdkError<ListInstancesError>`](crate::operation::list_instances::ListInstancesError)
    pub fn list_instances(
        &self,
    ) -> crate::operation::list_instances::builders::ListInstancesFluentBuilder {
        crate::operation::list_instances::builders::ListInstancesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
