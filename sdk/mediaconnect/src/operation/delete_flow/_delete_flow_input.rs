// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteFlowInput {
    /// The ARN of the flow that you want to delete.
    #[doc(hidden)]
    pub flow_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteFlowInput {
    /// The ARN of the flow that you want to delete.
    pub fn flow_arn(&self) -> ::std::option::Option<&str> {
        self.flow_arn.as_deref()
    }
}
impl DeleteFlowInput {
    /// Creates a new builder-style object to manufacture [`DeleteFlowInput`](crate::operation::delete_flow::DeleteFlowInput).
    pub fn builder() -> crate::operation::delete_flow::builders::DeleteFlowInputBuilder {
        crate::operation::delete_flow::builders::DeleteFlowInputBuilder::default()
    }
}

/// A builder for [`DeleteFlowInput`](crate::operation::delete_flow::DeleteFlowInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteFlowInputBuilder {
    pub(crate) flow_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteFlowInputBuilder {
    /// The ARN of the flow that you want to delete.
    pub fn flow_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.flow_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// The ARN of the flow that you want to delete.
    pub fn set_flow_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.flow_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteFlowInput`](crate::operation::delete_flow::DeleteFlowInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_flow::DeleteFlowInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_flow::DeleteFlowInput {
            flow_arn: self.flow_arn,
        })
    }
}
