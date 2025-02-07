// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_threat_intel_set::_update_threat_intel_set_output::UpdateThreatIntelSetOutputBuilder;

pub use crate::operation::update_threat_intel_set::_update_threat_intel_set_input::UpdateThreatIntelSetInputBuilder;

/// Fluent builder constructing a request to `UpdateThreatIntelSet`.
///
/// <p>Updates the ThreatIntelSet specified by the ThreatIntelSet ID.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateThreatIntelSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_threat_intel_set::builders::UpdateThreatIntelSetInputBuilder,
}
impl UpdateThreatIntelSetFluentBuilder {
    /// Creates a new `UpdateThreatIntelSet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_threat_intel_set::UpdateThreatIntelSet,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_threat_intel_set::UpdateThreatIntelSetError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_threat_intel_set::UpdateThreatIntelSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_threat_intel_set::UpdateThreatIntelSetError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_threat_intel_set::UpdateThreatIntelSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_threat_intel_set::UpdateThreatIntelSetError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_threat_intel_set::UpdateThreatIntelSet,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_threat_intel_set::UpdateThreatIntelSetError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The detectorID that specifies the GuardDuty service whose ThreatIntelSet you want to update.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The detectorID that specifies the GuardDuty service whose ThreatIntelSet you want to update.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// <p>The unique ID that specifies the ThreatIntelSet that you want to update.</p>
    pub fn threat_intel_set_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.threat_intel_set_id(input.into());
        self
    }
    /// <p>The unique ID that specifies the ThreatIntelSet that you want to update.</p>
    pub fn set_threat_intel_set_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_threat_intel_set_id(input);
        self
    }
    /// <p>The unique ID that specifies the ThreatIntelSet that you want to update.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The unique ID that specifies the ThreatIntelSet that you want to update.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The updated URI of the file that contains the ThreateIntelSet.</p>
    pub fn location(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.location(input.into());
        self
    }
    /// <p>The updated URI of the file that contains the ThreateIntelSet.</p>
    pub fn set_location(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_location(input);
        self
    }
    /// <p>The updated Boolean value that specifies whether the ThreateIntelSet is active or not.</p>
    pub fn activate(mut self, input: bool) -> Self {
        self.inner = self.inner.activate(input);
        self
    }
    /// <p>The updated Boolean value that specifies whether the ThreateIntelSet is active or not.</p>
    pub fn set_activate(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_activate(input);
        self
    }
}
