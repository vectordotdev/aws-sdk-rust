// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StopModelOutput {
    /// <p>The status of the model.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::ModelHostingStatus>,
    _request_id: Option<String>,
}
impl StopModelOutput {
    /// <p>The status of the model.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ModelHostingStatus> {
        self.status.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for StopModelOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StopModelOutput {
    /// Creates a new builder-style object to manufacture [`StopModelOutput`](crate::operation::stop_model::StopModelOutput).
    pub fn builder() -> crate::operation::stop_model::builders::StopModelOutputBuilder {
        crate::operation::stop_model::builders::StopModelOutputBuilder::default()
    }
}

/// A builder for [`StopModelOutput`](crate::operation::stop_model::StopModelOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StopModelOutputBuilder {
    pub(crate) status: ::std::option::Option<crate::types::ModelHostingStatus>,
    _request_id: Option<String>,
}
impl StopModelOutputBuilder {
    /// <p>The status of the model.</p>
    pub fn status(mut self, input: crate::types::ModelHostingStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the model.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::ModelHostingStatus>,
    ) -> Self {
        self.status = input;
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
    /// Consumes the builder and constructs a [`StopModelOutput`](crate::operation::stop_model::StopModelOutput).
    pub fn build(self) -> crate::operation::stop_model::StopModelOutput {
        crate::operation::stop_model::StopModelOutput {
            status: self.status,
            _request_id: self._request_id,
        }
    }
}
