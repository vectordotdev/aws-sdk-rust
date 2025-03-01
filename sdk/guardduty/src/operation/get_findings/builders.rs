// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_findings::_get_findings_output::GetFindingsOutputBuilder;

pub use crate::operation::get_findings::_get_findings_input::GetFindingsInputBuilder;

/// Fluent builder constructing a request to `GetFindings`.
///
/// <p>Describes Amazon GuardDuty findings specified by finding IDs.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetFindingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_findings::builders::GetFindingsInputBuilder,
}
impl GetFindingsFluentBuilder {
    /// Creates a new `GetFindings`.
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
            crate::operation::get_findings::GetFindings,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_findings::GetFindingsError>,
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
        crate::operation::get_findings::GetFindingsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::get_findings::GetFindingsError>,
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
        crate::operation::get_findings::GetFindingsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::get_findings::GetFindingsError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::get_findings::GetFindings,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_findings::GetFindingsError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings you want to retrieve.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings you want to retrieve.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// Appends an item to `FindingIds`.
    ///
    /// To override the contents of this collection use [`set_finding_ids`](Self::set_finding_ids).
    ///
    /// <p>The IDs of the findings that you want to retrieve.</p>
    pub fn finding_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.finding_ids(input.into());
        self
    }
    /// <p>The IDs of the findings that you want to retrieve.</p>
    pub fn set_finding_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_finding_ids(input);
        self
    }
    /// <p>Represents the criteria used for sorting findings.</p>
    pub fn sort_criteria(mut self, input: crate::types::SortCriteria) -> Self {
        self.inner = self.inner.sort_criteria(input);
        self
    }
    /// <p>Represents the criteria used for sorting findings.</p>
    pub fn set_sort_criteria(
        mut self,
        input: ::std::option::Option<crate::types::SortCriteria>,
    ) -> Self {
        self.inner = self.inner.set_sort_criteria(input);
        self
    }
}
