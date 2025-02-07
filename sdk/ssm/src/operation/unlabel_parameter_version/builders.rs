// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::unlabel_parameter_version::_unlabel_parameter_version_output::UnlabelParameterVersionOutputBuilder;

pub use crate::operation::unlabel_parameter_version::_unlabel_parameter_version_input::UnlabelParameterVersionInputBuilder;

/// Fluent builder constructing a request to `UnlabelParameterVersion`.
///
/// <p>Remove a label or labels from a parameter.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UnlabelParameterVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::unlabel_parameter_version::builders::UnlabelParameterVersionInputBuilder,
}
impl UnlabelParameterVersionFluentBuilder {
    /// Creates a new `UnlabelParameterVersion`.
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
            crate::operation::unlabel_parameter_version::UnlabelParameterVersion,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::unlabel_parameter_version::UnlabelParameterVersionError,
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
        crate::operation::unlabel_parameter_version::UnlabelParameterVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::unlabel_parameter_version::UnlabelParameterVersionError,
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
        crate::operation::unlabel_parameter_version::UnlabelParameterVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::unlabel_parameter_version::UnlabelParameterVersionError,
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
            crate::operation::unlabel_parameter_version::UnlabelParameterVersion,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::unlabel_parameter_version::UnlabelParameterVersionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the parameter from which you want to delete one or more labels.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the parameter from which you want to delete one or more labels.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The specific version of the parameter which you want to delete one or more labels from. If it isn't present, the call will fail.</p>
    pub fn parameter_version(mut self, input: i64) -> Self {
        self.inner = self.inner.parameter_version(input);
        self
    }
    /// <p>The specific version of the parameter which you want to delete one or more labels from. If it isn't present, the call will fail.</p>
    pub fn set_parameter_version(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_parameter_version(input);
        self
    }
    /// Appends an item to `Labels`.
    ///
    /// To override the contents of this collection use [`set_labels`](Self::set_labels).
    ///
    /// <p>One or more labels to delete from the specified parameter version.</p>
    pub fn labels(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.labels(input.into());
        self
    }
    /// <p>One or more labels to delete from the specified parameter version.</p>
    pub fn set_labels(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_labels(input);
        self
    }
}
