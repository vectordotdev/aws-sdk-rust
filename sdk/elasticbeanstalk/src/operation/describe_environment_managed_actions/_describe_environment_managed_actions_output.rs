// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The result message containing a list of managed actions.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeEnvironmentManagedActionsOutput {
    /// <p>A list of upcoming and in-progress managed actions.</p>
    #[doc(hidden)]
    pub managed_actions: ::std::option::Option<::std::vec::Vec<crate::types::ManagedAction>>,
    _request_id: Option<String>,
}
impl DescribeEnvironmentManagedActionsOutput {
    /// <p>A list of upcoming and in-progress managed actions.</p>
    pub fn managed_actions(&self) -> ::std::option::Option<&[crate::types::ManagedAction]> {
        self.managed_actions.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeEnvironmentManagedActionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeEnvironmentManagedActionsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeEnvironmentManagedActionsOutput`](crate::operation::describe_environment_managed_actions::DescribeEnvironmentManagedActionsOutput).
    pub fn builder() -> crate::operation::describe_environment_managed_actions::builders::DescribeEnvironmentManagedActionsOutputBuilder{
        crate::operation::describe_environment_managed_actions::builders::DescribeEnvironmentManagedActionsOutputBuilder::default()
    }
}

/// A builder for [`DescribeEnvironmentManagedActionsOutput`](crate::operation::describe_environment_managed_actions::DescribeEnvironmentManagedActionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeEnvironmentManagedActionsOutputBuilder {
    pub(crate) managed_actions: ::std::option::Option<::std::vec::Vec<crate::types::ManagedAction>>,
    _request_id: Option<String>,
}
impl DescribeEnvironmentManagedActionsOutputBuilder {
    /// Appends an item to `managed_actions`.
    ///
    /// To override the contents of this collection use [`set_managed_actions`](Self::set_managed_actions).
    ///
    /// <p>A list of upcoming and in-progress managed actions.</p>
    pub fn managed_actions(mut self, input: crate::types::ManagedAction) -> Self {
        let mut v = self.managed_actions.unwrap_or_default();
        v.push(input);
        self.managed_actions = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of upcoming and in-progress managed actions.</p>
    pub fn set_managed_actions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ManagedAction>>,
    ) -> Self {
        self.managed_actions = input;
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
    /// Consumes the builder and constructs a [`DescribeEnvironmentManagedActionsOutput`](crate::operation::describe_environment_managed_actions::DescribeEnvironmentManagedActionsOutput).
    pub fn build(self) -> crate::operation::describe_environment_managed_actions::DescribeEnvironmentManagedActionsOutput{
        crate::operation::describe_environment_managed_actions::DescribeEnvironmentManagedActionsOutput {
            managed_actions: self.managed_actions
            ,
            _request_id: self._request_id,
        }
    }
}
