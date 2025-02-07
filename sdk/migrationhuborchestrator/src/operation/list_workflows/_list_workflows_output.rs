// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListWorkflowsOutput {
    /// <p>The pagination token.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The summary of the migration workflow.</p>
    #[doc(hidden)]
    pub migration_workflow_summary:
        ::std::option::Option<::std::vec::Vec<crate::types::MigrationWorkflowSummary>>,
    _request_id: Option<String>,
}
impl ListWorkflowsOutput {
    /// <p>The pagination token.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The summary of the migration workflow.</p>
    pub fn migration_workflow_summary(
        &self,
    ) -> ::std::option::Option<&[crate::types::MigrationWorkflowSummary]> {
        self.migration_workflow_summary.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListWorkflowsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListWorkflowsOutput {
    /// Creates a new builder-style object to manufacture [`ListWorkflowsOutput`](crate::operation::list_workflows::ListWorkflowsOutput).
    pub fn builder() -> crate::operation::list_workflows::builders::ListWorkflowsOutputBuilder {
        crate::operation::list_workflows::builders::ListWorkflowsOutputBuilder::default()
    }
}

/// A builder for [`ListWorkflowsOutput`](crate::operation::list_workflows::ListWorkflowsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListWorkflowsOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) migration_workflow_summary:
        ::std::option::Option<::std::vec::Vec<crate::types::MigrationWorkflowSummary>>,
    _request_id: Option<String>,
}
impl ListWorkflowsOutputBuilder {
    /// <p>The pagination token.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `migration_workflow_summary`.
    ///
    /// To override the contents of this collection use [`set_migration_workflow_summary`](Self::set_migration_workflow_summary).
    ///
    /// <p>The summary of the migration workflow.</p>
    pub fn migration_workflow_summary(
        mut self,
        input: crate::types::MigrationWorkflowSummary,
    ) -> Self {
        let mut v = self.migration_workflow_summary.unwrap_or_default();
        v.push(input);
        self.migration_workflow_summary = ::std::option::Option::Some(v);
        self
    }
    /// <p>The summary of the migration workflow.</p>
    pub fn set_migration_workflow_summary(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MigrationWorkflowSummary>>,
    ) -> Self {
        self.migration_workflow_summary = input;
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
    /// Consumes the builder and constructs a [`ListWorkflowsOutput`](crate::operation::list_workflows::ListWorkflowsOutput).
    pub fn build(self) -> crate::operation::list_workflows::ListWorkflowsOutput {
        crate::operation::list_workflows::ListWorkflowsOutput {
            next_token: self.next_token,
            migration_workflow_summary: self.migration_workflow_summary,
            _request_id: self._request_id,
        }
    }
}
