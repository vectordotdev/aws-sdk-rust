// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_answer::_get_answer_output::GetAnswerOutputBuilder;

pub use crate::operation::get_answer::_get_answer_input::GetAnswerInputBuilder;

/// Fluent builder constructing a request to `GetAnswer`.
///
/// <p>Get the answer to a specific question in a workload review.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetAnswerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_answer::builders::GetAnswerInputBuilder,
}
impl GetAnswerFluentBuilder {
    /// Creates a new `GetAnswer`.
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
            crate::operation::get_answer::GetAnswer,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_answer::GetAnswerError>,
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
        crate::operation::get_answer::GetAnswerOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::get_answer::GetAnswerError>,
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
        crate::operation::get_answer::GetAnswerOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::get_answer::GetAnswerError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::get_answer::GetAnswer,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_answer::GetAnswerError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID assigned to the workload. This ID is unique within an Amazon Web Services Region.</p>
    pub fn workload_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workload_id(input.into());
        self
    }
    /// <p>The ID assigned to the workload. This ID is unique within an Amazon Web Services Region.</p>
    pub fn set_workload_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workload_id(input);
        self
    }
    /// <p>The alias of the lens.</p>
    /// <p>For Amazon Web Services official lenses, this is either the lens alias, such as <code>serverless</code>, or the lens ARN, such as <code>arn:aws:wellarchitected:us-east-1::lens/serverless</code>. Note that some operations (such as ExportLens and CreateLensShare) are not permitted on Amazon Web Services official lenses.</p>
    /// <p>For custom lenses, this is the lens ARN, such as <code>arn:aws:wellarchitected:us-west-2:123456789012:lens/0123456789abcdef01234567890abcdef</code>. </p>
    /// <p>Each lens is identified by its <code>LensSummary$LensAlias</code>.</p>
    pub fn lens_alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.lens_alias(input.into());
        self
    }
    /// <p>The alias of the lens.</p>
    /// <p>For Amazon Web Services official lenses, this is either the lens alias, such as <code>serverless</code>, or the lens ARN, such as <code>arn:aws:wellarchitected:us-east-1::lens/serverless</code>. Note that some operations (such as ExportLens and CreateLensShare) are not permitted on Amazon Web Services official lenses.</p>
    /// <p>For custom lenses, this is the lens ARN, such as <code>arn:aws:wellarchitected:us-west-2:123456789012:lens/0123456789abcdef01234567890abcdef</code>. </p>
    /// <p>Each lens is identified by its <code>LensSummary$LensAlias</code>.</p>
    pub fn set_lens_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_lens_alias(input);
        self
    }
    /// <p>The ID of the question.</p>
    pub fn question_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.question_id(input.into());
        self
    }
    /// <p>The ID of the question.</p>
    pub fn set_question_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_question_id(input);
        self
    }
    /// <p>The milestone number.</p>
    /// <p>A workload can have a maximum of 100 milestones.</p>
    pub fn milestone_number(mut self, input: i32) -> Self {
        self.inner = self.inner.milestone_number(input);
        self
    }
    /// <p>The milestone number.</p>
    /// <p>A workload can have a maximum of 100 milestones.</p>
    pub fn set_milestone_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_milestone_number(input);
        self
    }
}
