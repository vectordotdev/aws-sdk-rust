// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPullRequestApprovalStatesOutput {
    /// <p>Information about users who have approved the pull request.</p>
    #[doc(hidden)]
    pub approvals: ::std::option::Option<::std::vec::Vec<crate::types::Approval>>,
    _request_id: Option<String>,
}
impl GetPullRequestApprovalStatesOutput {
    /// <p>Information about users who have approved the pull request.</p>
    pub fn approvals(&self) -> ::std::option::Option<&[crate::types::Approval]> {
        self.approvals.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetPullRequestApprovalStatesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetPullRequestApprovalStatesOutput {
    /// Creates a new builder-style object to manufacture [`GetPullRequestApprovalStatesOutput`](crate::operation::get_pull_request_approval_states::GetPullRequestApprovalStatesOutput).
    pub fn builder() -> crate::operation::get_pull_request_approval_states::builders::GetPullRequestApprovalStatesOutputBuilder{
        crate::operation::get_pull_request_approval_states::builders::GetPullRequestApprovalStatesOutputBuilder::default()
    }
}

/// A builder for [`GetPullRequestApprovalStatesOutput`](crate::operation::get_pull_request_approval_states::GetPullRequestApprovalStatesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetPullRequestApprovalStatesOutputBuilder {
    pub(crate) approvals: ::std::option::Option<::std::vec::Vec<crate::types::Approval>>,
    _request_id: Option<String>,
}
impl GetPullRequestApprovalStatesOutputBuilder {
    /// Appends an item to `approvals`.
    ///
    /// To override the contents of this collection use [`set_approvals`](Self::set_approvals).
    ///
    /// <p>Information about users who have approved the pull request.</p>
    pub fn approvals(mut self, input: crate::types::Approval) -> Self {
        let mut v = self.approvals.unwrap_or_default();
        v.push(input);
        self.approvals = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about users who have approved the pull request.</p>
    pub fn set_approvals(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Approval>>,
    ) -> Self {
        self.approvals = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetPullRequestApprovalStatesOutput`](crate::operation::get_pull_request_approval_states::GetPullRequestApprovalStatesOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_pull_request_approval_states::GetPullRequestApprovalStatesOutput
    {
        crate::operation::get_pull_request_approval_states::GetPullRequestApprovalStatesOutput {
            approvals: self.approvals,
            _request_id: self._request_id,
        }
    }
}
