// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteVoiceMessageSpendLimitOverrideOutput {
    /// <p>The current monthly limit, in US dollars.</p>
    #[doc(hidden)]
    pub monthly_limit: ::std::option::Option<i64>,
    _request_id: Option<String>,
}
impl DeleteVoiceMessageSpendLimitOverrideOutput {
    /// <p>The current monthly limit, in US dollars.</p>
    pub fn monthly_limit(&self) -> ::std::option::Option<i64> {
        self.monthly_limit
    }
}
impl ::aws_http::request_id::RequestId for DeleteVoiceMessageSpendLimitOverrideOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteVoiceMessageSpendLimitOverrideOutput {
    /// Creates a new builder-style object to manufacture [`DeleteVoiceMessageSpendLimitOverrideOutput`](crate::operation::delete_voice_message_spend_limit_override::DeleteVoiceMessageSpendLimitOverrideOutput).
    pub fn builder() -> crate::operation::delete_voice_message_spend_limit_override::builders::DeleteVoiceMessageSpendLimitOverrideOutputBuilder{
        crate::operation::delete_voice_message_spend_limit_override::builders::DeleteVoiceMessageSpendLimitOverrideOutputBuilder::default()
    }
}

/// A builder for [`DeleteVoiceMessageSpendLimitOverrideOutput`](crate::operation::delete_voice_message_spend_limit_override::DeleteVoiceMessageSpendLimitOverrideOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteVoiceMessageSpendLimitOverrideOutputBuilder {
    pub(crate) monthly_limit: ::std::option::Option<i64>,
    _request_id: Option<String>,
}
impl DeleteVoiceMessageSpendLimitOverrideOutputBuilder {
    /// <p>The current monthly limit, in US dollars.</p>
    pub fn monthly_limit(mut self, input: i64) -> Self {
        self.monthly_limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current monthly limit, in US dollars.</p>
    pub fn set_monthly_limit(mut self, input: ::std::option::Option<i64>) -> Self {
        self.monthly_limit = input;
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
    /// Consumes the builder and constructs a [`DeleteVoiceMessageSpendLimitOverrideOutput`](crate::operation::delete_voice_message_spend_limit_override::DeleteVoiceMessageSpendLimitOverrideOutput).
    pub fn build(self) -> crate::operation::delete_voice_message_spend_limit_override::DeleteVoiceMessageSpendLimitOverrideOutput{
        crate::operation::delete_voice_message_spend_limit_override::DeleteVoiceMessageSpendLimitOverrideOutput {
            monthly_limit: self.monthly_limit
            ,
            _request_id: self._request_id,
        }
    }
}
