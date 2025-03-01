// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetStreamingSessionBackupOutput {
    /// <p>Information about the streaming session backup.</p>
    #[doc(hidden)]
    pub streaming_session_backup: ::std::option::Option<crate::types::StreamingSessionBackup>,
    _request_id: Option<String>,
}
impl GetStreamingSessionBackupOutput {
    /// <p>Information about the streaming session backup.</p>
    pub fn streaming_session_backup(
        &self,
    ) -> ::std::option::Option<&crate::types::StreamingSessionBackup> {
        self.streaming_session_backup.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetStreamingSessionBackupOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetStreamingSessionBackupOutput {
    /// Creates a new builder-style object to manufacture [`GetStreamingSessionBackupOutput`](crate::operation::get_streaming_session_backup::GetStreamingSessionBackupOutput).
    pub fn builder() -> crate::operation::get_streaming_session_backup::builders::GetStreamingSessionBackupOutputBuilder{
        crate::operation::get_streaming_session_backup::builders::GetStreamingSessionBackupOutputBuilder::default()
    }
}

/// A builder for [`GetStreamingSessionBackupOutput`](crate::operation::get_streaming_session_backup::GetStreamingSessionBackupOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetStreamingSessionBackupOutputBuilder {
    pub(crate) streaming_session_backup:
        ::std::option::Option<crate::types::StreamingSessionBackup>,
    _request_id: Option<String>,
}
impl GetStreamingSessionBackupOutputBuilder {
    /// <p>Information about the streaming session backup.</p>
    pub fn streaming_session_backup(mut self, input: crate::types::StreamingSessionBackup) -> Self {
        self.streaming_session_backup = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the streaming session backup.</p>
    pub fn set_streaming_session_backup(
        mut self,
        input: ::std::option::Option<crate::types::StreamingSessionBackup>,
    ) -> Self {
        self.streaming_session_backup = input;
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
    /// Consumes the builder and constructs a [`GetStreamingSessionBackupOutput`](crate::operation::get_streaming_session_backup::GetStreamingSessionBackupOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_streaming_session_backup::GetStreamingSessionBackupOutput {
        crate::operation::get_streaming_session_backup::GetStreamingSessionBackupOutput {
            streaming_session_backup: self.streaming_session_backup,
            _request_id: self._request_id,
        }
    }
}
