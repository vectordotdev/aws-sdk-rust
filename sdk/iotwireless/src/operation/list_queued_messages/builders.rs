// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_queued_messages::_list_queued_messages_output::ListQueuedMessagesOutputBuilder;

pub use crate::operation::list_queued_messages::_list_queued_messages_input::ListQueuedMessagesInputBuilder;

/// Fluent builder constructing a request to `ListQueuedMessages`.
///
/// <p>List queued messages in the downlink queue.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListQueuedMessagesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_queued_messages::builders::ListQueuedMessagesInputBuilder,
}
impl ListQueuedMessagesFluentBuilder {
    /// Creates a new `ListQueuedMessages`.
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
            crate::operation::list_queued_messages::ListQueuedMessages,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_queued_messages::ListQueuedMessagesError,
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
        crate::operation::list_queued_messages::ListQueuedMessagesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_queued_messages::ListQueuedMessagesError,
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
        crate::operation::list_queued_messages::ListQueuedMessagesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_queued_messages::ListQueuedMessagesError,
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
            crate::operation::list_queued_messages::ListQueuedMessages,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_queued_messages::ListQueuedMessagesError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_queued_messages::paginator::ListQueuedMessagesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_queued_messages::paginator::ListQueuedMessagesPaginator {
        crate::operation::list_queued_messages::paginator::ListQueuedMessagesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The ID of a given wireless device which the downlink message packets are being sent.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of a given wireless device which the downlink message packets are being sent.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The wireless device type, whic can be either Sidewalk or LoRaWAN.</p>
    pub fn wireless_device_type(mut self, input: crate::types::WirelessDeviceType) -> Self {
        self.inner = self.inner.wireless_device_type(input);
        self
    }
    /// <p>The wireless device type, whic can be either Sidewalk or LoRaWAN.</p>
    pub fn set_wireless_device_type(
        mut self,
        input: ::std::option::Option<crate::types::WirelessDeviceType>,
    ) -> Self {
        self.inner = self.inner.set_wireless_device_type(input);
        self
    }
}
