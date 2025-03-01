// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The request structure for the delete branch request. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteBranchInput {
    /// <p> The unique ID for an Amplify app. </p>
    #[doc(hidden)]
    pub app_id: ::std::option::Option<::std::string::String>,
    /// <p> The name for the branch. </p>
    #[doc(hidden)]
    pub branch_name: ::std::option::Option<::std::string::String>,
}
impl DeleteBranchInput {
    /// <p> The unique ID for an Amplify app. </p>
    pub fn app_id(&self) -> ::std::option::Option<&str> {
        self.app_id.as_deref()
    }
    /// <p> The name for the branch. </p>
    pub fn branch_name(&self) -> ::std::option::Option<&str> {
        self.branch_name.as_deref()
    }
}
impl DeleteBranchInput {
    /// Creates a new builder-style object to manufacture [`DeleteBranchInput`](crate::operation::delete_branch::DeleteBranchInput).
    pub fn builder() -> crate::operation::delete_branch::builders::DeleteBranchInputBuilder {
        crate::operation::delete_branch::builders::DeleteBranchInputBuilder::default()
    }
}

/// A builder for [`DeleteBranchInput`](crate::operation::delete_branch::DeleteBranchInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteBranchInputBuilder {
    pub(crate) app_id: ::std::option::Option<::std::string::String>,
    pub(crate) branch_name: ::std::option::Option<::std::string::String>,
}
impl DeleteBranchInputBuilder {
    /// <p> The unique ID for an Amplify app. </p>
    pub fn app_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.app_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The unique ID for an Amplify app. </p>
    pub fn set_app_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.app_id = input;
        self
    }
    /// <p> The name for the branch. </p>
    pub fn branch_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.branch_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name for the branch. </p>
    pub fn set_branch_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.branch_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteBranchInput`](crate::operation::delete_branch::DeleteBranchInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_branch::DeleteBranchInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_branch::DeleteBranchInput {
            app_id: self.app_id,
            branch_name: self.branch_name,
        })
    }
}
