// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_queued_savings_plan::_delete_queued_savings_plan_output::DeleteQueuedSavingsPlanOutputBuilder;

pub use crate::operation::delete_queued_savings_plan::_delete_queued_savings_plan_input::DeleteQueuedSavingsPlanInputBuilder;

/// Fluent builder constructing a request to `DeleteQueuedSavingsPlan`.
///
/// <p>Deletes the queued purchase for the specified Savings Plan.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteQueuedSavingsPlanFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::delete_queued_savings_plan::builders::DeleteQueuedSavingsPlanInputBuilder,
}
impl DeleteQueuedSavingsPlanFluentBuilder {
    /// Creates a new `DeleteQueuedSavingsPlan`.
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
            crate::operation::delete_queued_savings_plan::DeleteQueuedSavingsPlan,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_queued_savings_plan::DeleteQueuedSavingsPlanError,
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
        crate::operation::delete_queued_savings_plan::DeleteQueuedSavingsPlanOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_queued_savings_plan::DeleteQueuedSavingsPlanError,
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
        crate::operation::delete_queued_savings_plan::DeleteQueuedSavingsPlanOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_queued_savings_plan::DeleteQueuedSavingsPlanError,
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
            crate::operation::delete_queued_savings_plan::DeleteQueuedSavingsPlan,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_queued_savings_plan::DeleteQueuedSavingsPlanError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the Savings Plan.</p>
    pub fn savings_plan_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.savings_plan_id(input.into());
        self
    }
    /// <p>The ID of the Savings Plan.</p>
    pub fn set_savings_plan_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_savings_plan_id(input);
        self
    }
}
