// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AddFlowSourcesOutput {
    /// The ARN of the flow that these sources were added to.
    #[doc(hidden)]
    pub flow_arn: ::std::option::Option<::std::string::String>,
    /// The details of the newly added sources.
    #[doc(hidden)]
    pub sources: ::std::option::Option<::std::vec::Vec<crate::types::Source>>,
    _request_id: Option<String>,
}
impl AddFlowSourcesOutput {
    /// The ARN of the flow that these sources were added to.
    pub fn flow_arn(&self) -> ::std::option::Option<&str> {
        self.flow_arn.as_deref()
    }
    /// The details of the newly added sources.
    pub fn sources(&self) -> ::std::option::Option<&[crate::types::Source]> {
        self.sources.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for AddFlowSourcesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AddFlowSourcesOutput {
    /// Creates a new builder-style object to manufacture [`AddFlowSourcesOutput`](crate::operation::add_flow_sources::AddFlowSourcesOutput).
    pub fn builder() -> crate::operation::add_flow_sources::builders::AddFlowSourcesOutputBuilder {
        crate::operation::add_flow_sources::builders::AddFlowSourcesOutputBuilder::default()
    }
}

/// A builder for [`AddFlowSourcesOutput`](crate::operation::add_flow_sources::AddFlowSourcesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AddFlowSourcesOutputBuilder {
    pub(crate) flow_arn: ::std::option::Option<::std::string::String>,
    pub(crate) sources: ::std::option::Option<::std::vec::Vec<crate::types::Source>>,
    _request_id: Option<String>,
}
impl AddFlowSourcesOutputBuilder {
    /// The ARN of the flow that these sources were added to.
    pub fn flow_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.flow_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// The ARN of the flow that these sources were added to.
    pub fn set_flow_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.flow_arn = input;
        self
    }
    /// Appends an item to `sources`.
    ///
    /// To override the contents of this collection use [`set_sources`](Self::set_sources).
    ///
    /// The details of the newly added sources.
    pub fn sources(mut self, input: crate::types::Source) -> Self {
        let mut v = self.sources.unwrap_or_default();
        v.push(input);
        self.sources = ::std::option::Option::Some(v);
        self
    }
    /// The details of the newly added sources.
    pub fn set_sources(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Source>>,
    ) -> Self {
        self.sources = input;
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
    /// Consumes the builder and constructs a [`AddFlowSourcesOutput`](crate::operation::add_flow_sources::AddFlowSourcesOutput).
    pub fn build(self) -> crate::operation::add_flow_sources::AddFlowSourcesOutput {
        crate::operation::add_flow_sources::AddFlowSourcesOutput {
            flow_arn: self.flow_arn,
            sources: self.sources,
            _request_id: self._request_id,
        }
    }
}
