// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetResourcePolicies`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::set_next_token): <p>A continuation token, if this is a continuation request.</p>
    ///   - [`max_results(i32)`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::set_max_results): <p>The maximum size of a list to return.</p>
    /// - On success, responds with [`GetResourcePoliciesOutput`](crate::operation::get_resource_policies::GetResourcePoliciesOutput) with field(s):
    ///   - [`get_resource_policies_response_list(Option<Vec<GluePolicy>>)`](crate::operation::get_resource_policies::GetResourcePoliciesOutput::get_resource_policies_response_list): <p>A list of the individual resource policies and the account-level resource policy.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_resource_policies::GetResourcePoliciesOutput::next_token): <p>A continuation token, if the returned list does not contain the last resource policy available.</p>
    /// - On failure, responds with [`SdkError<GetResourcePoliciesError>`](crate::operation::get_resource_policies::GetResourcePoliciesError)
    pub fn get_resource_policies(
        &self,
    ) -> crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder {
        crate::operation::get_resource_policies::builders::GetResourcePoliciesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
