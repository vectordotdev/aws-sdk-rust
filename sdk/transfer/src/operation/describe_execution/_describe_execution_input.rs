// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeExecutionInput {
    /// <p>A unique identifier for the execution of a workflow.</p>
    #[doc(hidden)]
    pub execution_id: ::std::option::Option<::std::string::String>,
    /// <p>A unique identifier for the workflow.</p>
    #[doc(hidden)]
    pub workflow_id: ::std::option::Option<::std::string::String>,
}
impl DescribeExecutionInput {
    /// <p>A unique identifier for the execution of a workflow.</p>
    pub fn execution_id(&self) -> ::std::option::Option<&str> {
        self.execution_id.as_deref()
    }
    /// <p>A unique identifier for the workflow.</p>
    pub fn workflow_id(&self) -> ::std::option::Option<&str> {
        self.workflow_id.as_deref()
    }
}
impl DescribeExecutionInput {
    /// Creates a new builder-style object to manufacture [`DescribeExecutionInput`](crate::operation::describe_execution::DescribeExecutionInput).
    pub fn builder() -> crate::operation::describe_execution::builders::DescribeExecutionInputBuilder
    {
        crate::operation::describe_execution::builders::DescribeExecutionInputBuilder::default()
    }
}

/// A builder for [`DescribeExecutionInput`](crate::operation::describe_execution::DescribeExecutionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeExecutionInputBuilder {
    pub(crate) execution_id: ::std::option::Option<::std::string::String>,
    pub(crate) workflow_id: ::std::option::Option<::std::string::String>,
}
impl DescribeExecutionInputBuilder {
    /// <p>A unique identifier for the execution of a workflow.</p>
    pub fn execution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.execution_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the execution of a workflow.</p>
    pub fn set_execution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.execution_id = input;
        self
    }
    /// <p>A unique identifier for the workflow.</p>
    pub fn workflow_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workflow_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the workflow.</p>
    pub fn set_workflow_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workflow_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeExecutionInput`](crate::operation::describe_execution::DescribeExecutionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_execution::DescribeExecutionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_execution::DescribeExecutionInput {
                execution_id: self.execution_id,
                workflow_id: self.workflow_id,
            },
        )
    }
}
