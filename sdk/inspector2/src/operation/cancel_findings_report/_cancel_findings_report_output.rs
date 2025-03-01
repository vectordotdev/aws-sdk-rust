// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelFindingsReportOutput {
    /// <p>The ID of the canceled report.</p>
    #[doc(hidden)]
    pub report_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CancelFindingsReportOutput {
    /// <p>The ID of the canceled report.</p>
    pub fn report_id(&self) -> ::std::option::Option<&str> {
        self.report_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CancelFindingsReportOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CancelFindingsReportOutput {
    /// Creates a new builder-style object to manufacture [`CancelFindingsReportOutput`](crate::operation::cancel_findings_report::CancelFindingsReportOutput).
    pub fn builder(
    ) -> crate::operation::cancel_findings_report::builders::CancelFindingsReportOutputBuilder {
        crate::operation::cancel_findings_report::builders::CancelFindingsReportOutputBuilder::default()
    }
}

/// A builder for [`CancelFindingsReportOutput`](crate::operation::cancel_findings_report::CancelFindingsReportOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CancelFindingsReportOutputBuilder {
    pub(crate) report_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CancelFindingsReportOutputBuilder {
    /// <p>The ID of the canceled report.</p>
    pub fn report_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.report_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the canceled report.</p>
    pub fn set_report_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.report_id = input;
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
    /// Consumes the builder and constructs a [`CancelFindingsReportOutput`](crate::operation::cancel_findings_report::CancelFindingsReportOutput).
    pub fn build(self) -> crate::operation::cancel_findings_report::CancelFindingsReportOutput {
        crate::operation::cancel_findings_report::CancelFindingsReportOutput {
            report_id: self.report_id,
            _request_id: self._request_id,
        }
    }
}
