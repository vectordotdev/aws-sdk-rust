// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListServicePipelineOutputsInput {
    /// <p>The name of the service whose pipeline's outputs you want.</p>
    #[doc(hidden)]
    pub service_name: ::std::option::Option<::std::string::String>,
    /// <p>A token that indicates the location of the next output in the array of outputs, after the list of outputs that was previously requested.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListServicePipelineOutputsInput {
    /// <p>The name of the service whose pipeline's outputs you want.</p>
    pub fn service_name(&self) -> ::std::option::Option<&str> {
        self.service_name.as_deref()
    }
    /// <p>A token that indicates the location of the next output in the array of outputs, after the list of outputs that was previously requested.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListServicePipelineOutputsInput {
    /// Creates a new builder-style object to manufacture [`ListServicePipelineOutputsInput`](crate::operation::list_service_pipeline_outputs::ListServicePipelineOutputsInput).
    pub fn builder() -> crate::operation::list_service_pipeline_outputs::builders::ListServicePipelineOutputsInputBuilder{
        crate::operation::list_service_pipeline_outputs::builders::ListServicePipelineOutputsInputBuilder::default()
    }
}

/// A builder for [`ListServicePipelineOutputsInput`](crate::operation::list_service_pipeline_outputs::ListServicePipelineOutputsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListServicePipelineOutputsInputBuilder {
    pub(crate) service_name: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListServicePipelineOutputsInputBuilder {
    /// <p>The name of the service whose pipeline's outputs you want.</p>
    pub fn service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service whose pipeline's outputs you want.</p>
    pub fn set_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_name = input;
        self
    }
    /// <p>A token that indicates the location of the next output in the array of outputs, after the list of outputs that was previously requested.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token that indicates the location of the next output in the array of outputs, after the list of outputs that was previously requested.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListServicePipelineOutputsInput`](crate::operation::list_service_pipeline_outputs::ListServicePipelineOutputsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_service_pipeline_outputs::ListServicePipelineOutputsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_service_pipeline_outputs::ListServicePipelineOutputsInput {
                service_name: self.service_name,
                next_token: self.next_token,
            },
        )
    }
}
