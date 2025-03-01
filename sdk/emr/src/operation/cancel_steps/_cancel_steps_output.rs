// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The output for the <code>CancelSteps</code> operation. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelStepsOutput {
    /// <p>A list of <code>CancelStepsInfo</code>, which shows the status of specified cancel requests for each <code>StepID</code> specified.</p>
    #[doc(hidden)]
    pub cancel_steps_info_list:
        ::std::option::Option<::std::vec::Vec<crate::types::CancelStepsInfo>>,
    _request_id: Option<String>,
}
impl CancelStepsOutput {
    /// <p>A list of <code>CancelStepsInfo</code>, which shows the status of specified cancel requests for each <code>StepID</code> specified.</p>
    pub fn cancel_steps_info_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::CancelStepsInfo]> {
        self.cancel_steps_info_list.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CancelStepsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CancelStepsOutput {
    /// Creates a new builder-style object to manufacture [`CancelStepsOutput`](crate::operation::cancel_steps::CancelStepsOutput).
    pub fn builder() -> crate::operation::cancel_steps::builders::CancelStepsOutputBuilder {
        crate::operation::cancel_steps::builders::CancelStepsOutputBuilder::default()
    }
}

/// A builder for [`CancelStepsOutput`](crate::operation::cancel_steps::CancelStepsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CancelStepsOutputBuilder {
    pub(crate) cancel_steps_info_list:
        ::std::option::Option<::std::vec::Vec<crate::types::CancelStepsInfo>>,
    _request_id: Option<String>,
}
impl CancelStepsOutputBuilder {
    /// Appends an item to `cancel_steps_info_list`.
    ///
    /// To override the contents of this collection use [`set_cancel_steps_info_list`](Self::set_cancel_steps_info_list).
    ///
    /// <p>A list of <code>CancelStepsInfo</code>, which shows the status of specified cancel requests for each <code>StepID</code> specified.</p>
    pub fn cancel_steps_info_list(mut self, input: crate::types::CancelStepsInfo) -> Self {
        let mut v = self.cancel_steps_info_list.unwrap_or_default();
        v.push(input);
        self.cancel_steps_info_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>CancelStepsInfo</code>, which shows the status of specified cancel requests for each <code>StepID</code> specified.</p>
    pub fn set_cancel_steps_info_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CancelStepsInfo>>,
    ) -> Self {
        self.cancel_steps_info_list = input;
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
    /// Consumes the builder and constructs a [`CancelStepsOutput`](crate::operation::cancel_steps::CancelStepsOutput).
    pub fn build(self) -> crate::operation::cancel_steps::CancelStepsOutput {
        crate::operation::cancel_steps::CancelStepsOutput {
            cancel_steps_info_list: self.cancel_steps_info_list,
            _request_id: self._request_id,
        }
    }
}
