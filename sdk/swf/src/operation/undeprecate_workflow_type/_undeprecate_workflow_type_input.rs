// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UndeprecateWorkflowTypeInput {
    /// <p>The name of the domain of the deprecated workflow type.</p>
    #[doc(hidden)]
    pub domain: ::std::option::Option<::std::string::String>,
    /// <p>The name of the domain of the deprecated workflow type.</p>
    #[doc(hidden)]
    pub workflow_type: ::std::option::Option<crate::types::WorkflowType>,
}
impl UndeprecateWorkflowTypeInput {
    /// <p>The name of the domain of the deprecated workflow type.</p>
    pub fn domain(&self) -> ::std::option::Option<&str> {
        self.domain.as_deref()
    }
    /// <p>The name of the domain of the deprecated workflow type.</p>
    pub fn workflow_type(&self) -> ::std::option::Option<&crate::types::WorkflowType> {
        self.workflow_type.as_ref()
    }
}
impl UndeprecateWorkflowTypeInput {
    /// Creates a new builder-style object to manufacture [`UndeprecateWorkflowTypeInput`](crate::operation::undeprecate_workflow_type::UndeprecateWorkflowTypeInput).
    pub fn builder(
    ) -> crate::operation::undeprecate_workflow_type::builders::UndeprecateWorkflowTypeInputBuilder
    {
        crate::operation::undeprecate_workflow_type::builders::UndeprecateWorkflowTypeInputBuilder::default()
    }
}

/// A builder for [`UndeprecateWorkflowTypeInput`](crate::operation::undeprecate_workflow_type::UndeprecateWorkflowTypeInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UndeprecateWorkflowTypeInputBuilder {
    pub(crate) domain: ::std::option::Option<::std::string::String>,
    pub(crate) workflow_type: ::std::option::Option<crate::types::WorkflowType>,
}
impl UndeprecateWorkflowTypeInputBuilder {
    /// <p>The name of the domain of the deprecated workflow type.</p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the domain of the deprecated workflow type.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain = input;
        self
    }
    /// <p>The name of the domain of the deprecated workflow type.</p>
    pub fn workflow_type(mut self, input: crate::types::WorkflowType) -> Self {
        self.workflow_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the domain of the deprecated workflow type.</p>
    pub fn set_workflow_type(
        mut self,
        input: ::std::option::Option<crate::types::WorkflowType>,
    ) -> Self {
        self.workflow_type = input;
        self
    }
    /// Consumes the builder and constructs a [`UndeprecateWorkflowTypeInput`](crate::operation::undeprecate_workflow_type::UndeprecateWorkflowTypeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::undeprecate_workflow_type::UndeprecateWorkflowTypeInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::undeprecate_workflow_type::UndeprecateWorkflowTypeInput {
                domain: self.domain,
                workflow_type: self.workflow_type,
            },
        )
    }
}
