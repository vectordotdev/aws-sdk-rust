// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ValidatePolicy`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`locale(Locale)`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::locale) / [`set_locale(Option<Locale>)`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::set_locale): <p>The locale to use for localizing the findings.</p>
    ///   - [`max_results(i32)`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::set_max_results): <p>The maximum number of results to return in the response.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::set_next_token): <p>A token used for pagination of results returned.</p>
    ///   - [`policy_document(impl ::std::convert::Into<String>)`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::policy_document) / [`set_policy_document(Option<String>)`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::set_policy_document): <p>The JSON policy document to use as the content for the policy.</p>
    ///   - [`policy_type(PolicyType)`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::policy_type) / [`set_policy_type(Option<PolicyType>)`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::set_policy_type): <p>The type of policy to validate. Identity policies grant permissions to IAM principals. Identity policies include managed and inline policies for IAM roles, users, and groups. They also include service-control policies (SCPs) that are attached to an Amazon Web Services organization, organizational unit (OU), or an account.</p>  <p>Resource policies grant permissions on Amazon Web Services resources. Resource policies include trust policies for IAM roles and bucket policies for Amazon S3 buckets. You can provide a generic input such as identity policy or resource policy or a specific input such as managed policy or Amazon S3 bucket policy. </p>
    ///   - [`validate_policy_resource_type(ValidatePolicyResourceType)`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::validate_policy_resource_type) / [`set_validate_policy_resource_type(Option<ValidatePolicyResourceType>)`](crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::set_validate_policy_resource_type): <p>The type of resource to attach to your resource policy. Specify a value for the policy validation resource type only if the policy type is <code>RESOURCE_POLICY</code>. For example, to validate a resource policy to attach to an Amazon S3 bucket, you can choose <code>AWS::S3::Bucket</code> for the policy validation resource type.</p>  <p>For resource types not supported as valid values, IAM Access Analyzer runs policy checks that apply to all resource policies. For example, to validate a resource policy to attach to a KMS key, do not specify a value for the policy validation resource type and IAM Access Analyzer will run policy checks that apply to all resource policies.</p>
    /// - On success, responds with [`ValidatePolicyOutput`](crate::operation::validate_policy::ValidatePolicyOutput) with field(s):
    ///   - [`findings(Option<Vec<ValidatePolicyFinding>>)`](crate::operation::validate_policy::ValidatePolicyOutput::findings): <p>The list of findings in a policy returned by IAM Access Analyzer based on its suite of policy checks.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::validate_policy::ValidatePolicyOutput::next_token): <p>A token used for pagination of results returned.</p>
    /// - On failure, responds with [`SdkError<ValidatePolicyError>`](crate::operation::validate_policy::ValidatePolicyError)
    pub fn validate_policy(
        &self,
    ) -> crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder {
        crate::operation::validate_policy::builders::ValidatePolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
