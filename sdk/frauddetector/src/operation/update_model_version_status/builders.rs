// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_model_version_status::_update_model_version_status_output::UpdateModelVersionStatusOutputBuilder;

pub use crate::operation::update_model_version_status::_update_model_version_status_input::UpdateModelVersionStatusInputBuilder;

/// Fluent builder constructing a request to `UpdateModelVersionStatus`.
///
/// <p>Updates the status of a model version.</p>
/// <p>You can perform the following status updates:</p>
/// <ol>
/// <li> <p>Change the <code>TRAINING_IN_PROGRESS</code> status to <code>TRAINING_CANCELLED</code>.</p> </li>
/// <li> <p>Change the <code>TRAINING_COMPLETE</code> status to <code>ACTIVE</code>.</p> </li>
/// <li> <p>Change <code>ACTIVE</code> to <code>INACTIVE</code>.</p> </li>
/// </ol>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateModelVersionStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_model_version_status::builders::UpdateModelVersionStatusInputBuilder,
}
impl UpdateModelVersionStatusFluentBuilder {
    /// Creates a new `UpdateModelVersionStatus`.
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
            crate::operation::update_model_version_status::UpdateModelVersionStatus,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_model_version_status::UpdateModelVersionStatusError,
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
        crate::operation::update_model_version_status::UpdateModelVersionStatusOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_model_version_status::UpdateModelVersionStatusError,
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
        crate::operation::update_model_version_status::UpdateModelVersionStatusOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_model_version_status::UpdateModelVersionStatusError,
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
            crate::operation::update_model_version_status::UpdateModelVersionStatus,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_model_version_status::UpdateModelVersionStatusError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The model ID of the model version to update.</p>
    pub fn model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.model_id(input.into());
        self
    }
    /// <p>The model ID of the model version to update.</p>
    pub fn set_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_model_id(input);
        self
    }
    /// <p>The model type.</p>
    pub fn model_type(mut self, input: crate::types::ModelTypeEnum) -> Self {
        self.inner = self.inner.model_type(input);
        self
    }
    /// <p>The model type.</p>
    pub fn set_model_type(
        mut self,
        input: ::std::option::Option<crate::types::ModelTypeEnum>,
    ) -> Self {
        self.inner = self.inner.set_model_type(input);
        self
    }
    /// <p>The model version number.</p>
    pub fn model_version_number(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.model_version_number(input.into());
        self
    }
    /// <p>The model version number.</p>
    pub fn set_model_version_number(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_model_version_number(input);
        self
    }
    /// <p>The model version status.</p>
    pub fn status(mut self, input: crate::types::ModelVersionStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>The model version status.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::ModelVersionStatus>,
    ) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
}
