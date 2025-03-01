// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_pull_request_approval_rule_content::_update_pull_request_approval_rule_content_output::UpdatePullRequestApprovalRuleContentOutputBuilder;

pub use crate::operation::update_pull_request_approval_rule_content::_update_pull_request_approval_rule_content_input::UpdatePullRequestApprovalRuleContentInputBuilder;

/// Fluent builder constructing a request to `UpdatePullRequestApprovalRuleContent`.
///
/// <p>Updates the structure of an approval rule created specifically for a pull request. For example, you can change the number of required approvers and the approval pool for approvers. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdatePullRequestApprovalRuleContentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_pull_request_approval_rule_content::builders::UpdatePullRequestApprovalRuleContentInputBuilder,
}
impl UpdatePullRequestApprovalRuleContentFluentBuilder {
    /// Creates a new `UpdatePullRequestApprovalRuleContent`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::update_pull_request_approval_rule_content::UpdatePullRequestApprovalRuleContent, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::update_pull_request_approval_rule_content::UpdatePullRequestApprovalRuleContentError>
    >{
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::update_pull_request_approval_rule_content::UpdatePullRequestApprovalRuleContentOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_pull_request_approval_rule_content::UpdatePullRequestApprovalRuleContentError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
                        pub async fn send(self) -> ::std::result::Result<crate::operation::update_pull_request_approval_rule_content::UpdatePullRequestApprovalRuleContentOutput, ::aws_smithy_http::result::SdkError<crate::operation::update_pull_request_approval_rule_content::UpdatePullRequestApprovalRuleContentError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::update_pull_request_approval_rule_content::UpdatePullRequestApprovalRuleContent, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::update_pull_request_approval_rule_content::UpdatePullRequestApprovalRuleContentError>
    >{
        self.customize_middleware().await
    }
    /// <p>The system-generated ID of the pull request.</p>
    pub fn pull_request_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.pull_request_id(input.into());
        self
    }
    /// <p>The system-generated ID of the pull request.</p>
    pub fn set_pull_request_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_pull_request_id(input);
        self
    }
    /// <p>The name of the approval rule you want to update.</p>
    pub fn approval_rule_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.approval_rule_name(input.into());
        self
    }
    /// <p>The name of the approval rule you want to update.</p>
    pub fn set_approval_rule_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_approval_rule_name(input);
        self
    }
    /// <p>The SHA-256 hash signature for the content of the approval rule. You can retrieve this information by using <code>GetPullRequest</code>.</p>
    pub fn existing_rule_content_sha256(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.existing_rule_content_sha256(input.into());
        self
    }
    /// <p>The SHA-256 hash signature for the content of the approval rule. You can retrieve this information by using <code>GetPullRequest</code>.</p>
    pub fn set_existing_rule_content_sha256(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_existing_rule_content_sha256(input);
        self
    }
    /// <p>The updated content for the approval rule.</p> <note>
    /// <p>When you update the content of the approval rule, you can specify approvers in an approval pool in one of two ways:</p>
    /// <ul>
    /// <li> <p> <b>CodeCommitApprovers</b>: This option only requires an AWS account and a resource. It can be used for both IAM users and federated access users whose name matches the provided resource name. This is a very powerful option that offers a great deal of flexibility. For example, if you specify the AWS account <i>123456789012</i> and <i>Mary_Major</i>, all of the following are counted as approvals coming from that user:</p>
    /// <ul>
    /// <li> <p>An IAM user in the account (arn:aws:iam::<i>123456789012</i>:user/<i>Mary_Major</i>)</p> </li>
    /// <li> <p>A federated user identified in IAM as Mary_Major (arn:aws:sts::<i>123456789012</i>:federated-user/<i>Mary_Major</i>)</p> </li>
    /// </ul> <p>This option does not recognize an active session of someone assuming the role of CodeCommitReview with a role session name of <i>Mary_Major</i> (arn:aws:sts::<i>123456789012</i>:assumed-role/CodeCommitReview/<i>Mary_Major</i>) unless you include a wildcard (*Mary_Major).</p> </li>
    /// <li> <p> <b>Fully qualified ARN</b>: This option allows you to specify the fully qualified Amazon Resource Name (ARN) of the IAM user or role. </p> </li>
    /// </ul>
    /// <p>For more information about IAM ARNs, wildcards, and formats, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the <i>IAM User Guide</i>.</p>
    /// </note>
    pub fn new_rule_content(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.new_rule_content(input.into());
        self
    }
    /// <p>The updated content for the approval rule.</p> <note>
    /// <p>When you update the content of the approval rule, you can specify approvers in an approval pool in one of two ways:</p>
    /// <ul>
    /// <li> <p> <b>CodeCommitApprovers</b>: This option only requires an AWS account and a resource. It can be used for both IAM users and federated access users whose name matches the provided resource name. This is a very powerful option that offers a great deal of flexibility. For example, if you specify the AWS account <i>123456789012</i> and <i>Mary_Major</i>, all of the following are counted as approvals coming from that user:</p>
    /// <ul>
    /// <li> <p>An IAM user in the account (arn:aws:iam::<i>123456789012</i>:user/<i>Mary_Major</i>)</p> </li>
    /// <li> <p>A federated user identified in IAM as Mary_Major (arn:aws:sts::<i>123456789012</i>:federated-user/<i>Mary_Major</i>)</p> </li>
    /// </ul> <p>This option does not recognize an active session of someone assuming the role of CodeCommitReview with a role session name of <i>Mary_Major</i> (arn:aws:sts::<i>123456789012</i>:assumed-role/CodeCommitReview/<i>Mary_Major</i>) unless you include a wildcard (*Mary_Major).</p> </li>
    /// <li> <p> <b>Fully qualified ARN</b>: This option allows you to specify the fully qualified Amazon Resource Name (ARN) of the IAM user or role. </p> </li>
    /// </ul>
    /// <p>For more information about IAM ARNs, wildcards, and formats, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the <i>IAM User Guide</i>.</p>
    /// </note>
    pub fn set_new_rule_content(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_new_rule_content(input);
        self
    }
}
