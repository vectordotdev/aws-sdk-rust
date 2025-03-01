// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetResourcePolicies`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arns(Vec<String>)`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::resource_arns) / [`set_resource_arns(Option<Vec<String>>)`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::set_resource_arns): <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> of the resources whose policies you want to retrieve.</p>
    ///   - [`principal(impl ::std::convert::Into<String>)`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::principal) / [`set_principal(Option<String>)`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::set_principal): <p>Specifies the principal.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::set_next_token): <p>Specifies that you want to receive the next page of results. Valid only if you received a <code>NextToken</code> response in the previous request. If you did, it indicates that more output is available. Set this parameter to the value provided by the previous call's <code>NextToken</code> response to request the next page of results.</p>
    ///   - [`max_results(i32)`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::set_max_results): <p>Specifies the total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the number you specify, the <code>NextToken</code> response element is returned with a value (not null). Include the specified value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that the service might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    /// - On success, responds with [`GetResourcePoliciesOutput`](crate::operation::get_resource_policies::GetResourcePoliciesOutput) with field(s):
    ///   - [`policies(Option<Vec<String>>)`](crate::operation::get_resource_policies::GetResourcePoliciesOutput::policies): <p>An array of resource policy documents in JSON format.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_resource_policies::GetResourcePoliciesOutput::next_token): <p>If present, this value indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>. This indicates that this is the last page of results.</p>
    /// - On failure, responds with [`SdkError<GetResourcePoliciesError>`](crate::operation::get_resource_policies::GetResourcePoliciesError)
    pub fn get_resource_policies(
        &self,
    ) -> crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder {
        crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
