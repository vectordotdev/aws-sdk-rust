// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_inference_schedulers::_list_inference_schedulers_output::ListInferenceSchedulersOutputBuilder;

pub use crate::operation::list_inference_schedulers::_list_inference_schedulers_input::ListInferenceSchedulersInputBuilder;

/// Fluent builder constructing a request to `ListInferenceSchedulers`.
///
/// <p>Retrieves a list of all inference schedulers currently available for your account. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListInferenceSchedulersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::list_inference_schedulers::builders::ListInferenceSchedulersInputBuilder,
}
impl ListInferenceSchedulersFluentBuilder {
    /// Creates a new `ListInferenceSchedulers`.
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
            crate::operation::list_inference_schedulers::ListInferenceSchedulers,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_inference_schedulers::ListInferenceSchedulersError,
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
        crate::operation::list_inference_schedulers::ListInferenceSchedulersOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_inference_schedulers::ListInferenceSchedulersError,
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
        crate::operation::list_inference_schedulers::ListInferenceSchedulersOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_inference_schedulers::ListInferenceSchedulersError,
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
            crate::operation::list_inference_schedulers::ListInferenceSchedulers,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_inference_schedulers::ListInferenceSchedulersError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_inference_schedulers::paginator::ListInferenceSchedulersPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_inference_schedulers::paginator::ListInferenceSchedulersPaginator
    {
        crate::operation::list_inference_schedulers::paginator::ListInferenceSchedulersPaginator::new(self.handle, self.inner)
    }
    /// <p> An opaque pagination token indicating where to continue the listing of inference schedulers. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p> An opaque pagination token indicating where to continue the listing of inference schedulers. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p> Specifies the maximum number of inference schedulers to list. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p> Specifies the maximum number of inference schedulers to list. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The beginning of the name of the inference schedulers to be listed. </p>
    pub fn inference_scheduler_name_begins_with(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self
            .inner
            .inference_scheduler_name_begins_with(input.into());
        self
    }
    /// <p>The beginning of the name of the inference schedulers to be listed. </p>
    pub fn set_inference_scheduler_name_begins_with(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_inference_scheduler_name_begins_with(input);
        self
    }
    /// <p>The name of the ML model used by the inference scheduler to be listed. </p>
    pub fn model_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.model_name(input.into());
        self
    }
    /// <p>The name of the ML model used by the inference scheduler to be listed. </p>
    pub fn set_model_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_model_name(input);
        self
    }
    /// <p>Specifies the current status of the inference schedulers to list.</p>
    pub fn status(mut self, input: crate::types::InferenceSchedulerStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>Specifies the current status of the inference schedulers to list.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::InferenceSchedulerStatus>,
    ) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
}
