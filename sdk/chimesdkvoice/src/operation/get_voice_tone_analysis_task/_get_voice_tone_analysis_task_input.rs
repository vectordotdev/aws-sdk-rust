// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetVoiceToneAnalysisTaskInput {
    /// <p>The Voice Connector ID.</p>
    #[doc(hidden)]
    pub voice_connector_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the voice tone anlysis task.</p>
    #[doc(hidden)]
    pub voice_tone_analysis_task_id: ::std::option::Option<::std::string::String>,
    /// <p>Specifies whether the voice being analyzed is the caller (originator) or the callee (responder).</p>
    #[doc(hidden)]
    pub is_caller: ::std::option::Option<bool>,
}
impl GetVoiceToneAnalysisTaskInput {
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(&self) -> ::std::option::Option<&str> {
        self.voice_connector_id.as_deref()
    }
    /// <p>The ID of the voice tone anlysis task.</p>
    pub fn voice_tone_analysis_task_id(&self) -> ::std::option::Option<&str> {
        self.voice_tone_analysis_task_id.as_deref()
    }
    /// <p>Specifies whether the voice being analyzed is the caller (originator) or the callee (responder).</p>
    pub fn is_caller(&self) -> ::std::option::Option<bool> {
        self.is_caller
    }
}
impl GetVoiceToneAnalysisTaskInput {
    /// Creates a new builder-style object to manufacture [`GetVoiceToneAnalysisTaskInput`](crate::operation::get_voice_tone_analysis_task::GetVoiceToneAnalysisTaskInput).
    pub fn builder() -> crate::operation::get_voice_tone_analysis_task::builders::GetVoiceToneAnalysisTaskInputBuilder{
        crate::operation::get_voice_tone_analysis_task::builders::GetVoiceToneAnalysisTaskInputBuilder::default()
    }
}

/// A builder for [`GetVoiceToneAnalysisTaskInput`](crate::operation::get_voice_tone_analysis_task::GetVoiceToneAnalysisTaskInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetVoiceToneAnalysisTaskInputBuilder {
    pub(crate) voice_connector_id: ::std::option::Option<::std::string::String>,
    pub(crate) voice_tone_analysis_task_id: ::std::option::Option<::std::string::String>,
    pub(crate) is_caller: ::std::option::Option<bool>,
}
impl GetVoiceToneAnalysisTaskInputBuilder {
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.voice_connector_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Voice Connector ID.</p>
    pub fn set_voice_connector_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.voice_connector_id = input;
        self
    }
    /// <p>The ID of the voice tone anlysis task.</p>
    pub fn voice_tone_analysis_task_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.voice_tone_analysis_task_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the voice tone anlysis task.</p>
    pub fn set_voice_tone_analysis_task_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.voice_tone_analysis_task_id = input;
        self
    }
    /// <p>Specifies whether the voice being analyzed is the caller (originator) or the callee (responder).</p>
    pub fn is_caller(mut self, input: bool) -> Self {
        self.is_caller = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether the voice being analyzed is the caller (originator) or the callee (responder).</p>
    pub fn set_is_caller(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_caller = input;
        self
    }
    /// Consumes the builder and constructs a [`GetVoiceToneAnalysisTaskInput`](crate::operation::get_voice_tone_analysis_task::GetVoiceToneAnalysisTaskInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_voice_tone_analysis_task::GetVoiceToneAnalysisTaskInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_voice_tone_analysis_task::GetVoiceToneAnalysisTaskInput {
                voice_connector_id: self.voice_connector_id,
                voice_tone_analysis_task_id: self.voice_tone_analysis_task_id,
                is_caller: self.is_caller,
            },
        )
    }
}
