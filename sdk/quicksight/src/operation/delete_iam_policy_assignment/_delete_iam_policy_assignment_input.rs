// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteIamPolicyAssignmentInput {
    /// <p>The Amazon Web Services account ID where you want to delete the IAM policy assignment.</p>
    #[doc(hidden)]
    pub aws_account_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the assignment. </p>
    #[doc(hidden)]
    pub assignment_name: ::std::option::Option<::std::string::String>,
    /// <p>The namespace that contains the assignment.</p>
    #[doc(hidden)]
    pub namespace: ::std::option::Option<::std::string::String>,
}
impl DeleteIamPolicyAssignmentInput {
    /// <p>The Amazon Web Services account ID where you want to delete the IAM policy assignment.</p>
    pub fn aws_account_id(&self) -> ::std::option::Option<&str> {
        self.aws_account_id.as_deref()
    }
    /// <p>The name of the assignment. </p>
    pub fn assignment_name(&self) -> ::std::option::Option<&str> {
        self.assignment_name.as_deref()
    }
    /// <p>The namespace that contains the assignment.</p>
    pub fn namespace(&self) -> ::std::option::Option<&str> {
        self.namespace.as_deref()
    }
}
impl DeleteIamPolicyAssignmentInput {
    /// Creates a new builder-style object to manufacture [`DeleteIamPolicyAssignmentInput`](crate::operation::delete_iam_policy_assignment::DeleteIamPolicyAssignmentInput).
    pub fn builder() -> crate::operation::delete_iam_policy_assignment::builders::DeleteIamPolicyAssignmentInputBuilder{
        crate::operation::delete_iam_policy_assignment::builders::DeleteIamPolicyAssignmentInputBuilder::default()
    }
}

/// A builder for [`DeleteIamPolicyAssignmentInput`](crate::operation::delete_iam_policy_assignment::DeleteIamPolicyAssignmentInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteIamPolicyAssignmentInputBuilder {
    pub(crate) aws_account_id: ::std::option::Option<::std::string::String>,
    pub(crate) assignment_name: ::std::option::Option<::std::string::String>,
    pub(crate) namespace: ::std::option::Option<::std::string::String>,
}
impl DeleteIamPolicyAssignmentInputBuilder {
    /// <p>The Amazon Web Services account ID where you want to delete the IAM policy assignment.</p>
    pub fn aws_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.aws_account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID where you want to delete the IAM policy assignment.</p>
    pub fn set_aws_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.aws_account_id = input;
        self
    }
    /// <p>The name of the assignment. </p>
    pub fn assignment_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.assignment_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the assignment. </p>
    pub fn set_assignment_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.assignment_name = input;
        self
    }
    /// <p>The namespace that contains the assignment.</p>
    pub fn namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.namespace = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The namespace that contains the assignment.</p>
    pub fn set_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.namespace = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteIamPolicyAssignmentInput`](crate::operation::delete_iam_policy_assignment::DeleteIamPolicyAssignmentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_iam_policy_assignment::DeleteIamPolicyAssignmentInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_iam_policy_assignment::DeleteIamPolicyAssignmentInput {
                aws_account_id: self.aws_account_id,
                assignment_name: self.assignment_name,
                namespace: self.namespace,
            },
        )
    }
}
