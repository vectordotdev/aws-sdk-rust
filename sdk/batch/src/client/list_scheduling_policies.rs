// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSchedulingPolicies`](crate::operation::list_scheduling_policies::builders::ListSchedulingPoliciesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_scheduling_policies::builders::ListSchedulingPoliciesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_scheduling_policies::builders::ListSchedulingPoliciesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_scheduling_policies::builders::ListSchedulingPoliciesFluentBuilder::set_max_results): <p>The maximum number of results that's returned by <code>ListSchedulingPolicies</code> in paginated output. When this parameter is used, <code>ListSchedulingPolicies</code> only returns <code>maxResults</code> results in a single page and a <code>nextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListSchedulingPolicies</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, <code>ListSchedulingPolicies</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_scheduling_policies::builders::ListSchedulingPoliciesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_scheduling_policies::builders::ListSchedulingPoliciesFluentBuilder::set_next_token): <p>The <code>nextToken</code> value that's returned from a previous paginated <code>ListSchedulingPolicies</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p> <note>   <p>Treat this token as an opaque identifier that's only used to retrieve the next items in a list and not for other programmatic purposes.</p>  </note>
    /// - On success, responds with [`ListSchedulingPoliciesOutput`](crate::operation::list_scheduling_policies::ListSchedulingPoliciesOutput) with field(s):
    ///   - [`scheduling_policies(Option<Vec<SchedulingPolicyListingDetail>>)`](crate::operation::list_scheduling_policies::ListSchedulingPoliciesOutput::scheduling_policies): <p>A list of scheduling policies that match the request.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_scheduling_policies::ListSchedulingPoliciesOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>ListSchedulingPolicies</code> request. When the results of a <code>ListSchedulingPolicies</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<ListSchedulingPoliciesError>`](crate::operation::list_scheduling_policies::ListSchedulingPoliciesError)
    pub fn list_scheduling_policies(
        &self,
    ) -> crate::operation::list_scheduling_policies::builders::ListSchedulingPoliciesFluentBuilder
    {
        crate::operation::list_scheduling_policies::builders::ListSchedulingPoliciesFluentBuilder::new(self.handle.clone())
    }
}
