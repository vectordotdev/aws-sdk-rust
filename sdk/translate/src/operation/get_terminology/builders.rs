// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_terminology::_get_terminology_output::GetTerminologyOutputBuilder;

pub use crate::operation::get_terminology::_get_terminology_input::GetTerminologyInputBuilder;

/// Fluent builder constructing a request to `GetTerminology`.
///
/// <p>Retrieves a custom terminology.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetTerminologyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_terminology::builders::GetTerminologyInputBuilder,
}
impl GetTerminologyFluentBuilder {
    /// Creates a new `GetTerminology`.
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
            crate::operation::get_terminology::GetTerminology,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_terminology::GetTerminologyError>,
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
        crate::operation::get_terminology::GetTerminologyOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::get_terminology::GetTerminologyError>,
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
        crate::operation::get_terminology::GetTerminologyOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::get_terminology::GetTerminologyError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::get_terminology::GetTerminology,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_terminology::GetTerminologyError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the custom terminology being retrieved.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the custom terminology being retrieved.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The data format of the custom terminology being retrieved.</p>
    /// <p>If you don't specify this parameter, Amazon Translate returns a file with the same format as the file that was imported to create the terminology. </p>
    /// <p>If you specify this parameter when you retrieve a multi-directional terminology resource, you must specify the same format as the input file that was imported to create it. Otherwise, Amazon Translate throws an error.</p>
    pub fn terminology_data_format(mut self, input: crate::types::TerminologyDataFormat) -> Self {
        self.inner = self.inner.terminology_data_format(input);
        self
    }
    /// <p>The data format of the custom terminology being retrieved.</p>
    /// <p>If you don't specify this parameter, Amazon Translate returns a file with the same format as the file that was imported to create the terminology. </p>
    /// <p>If you specify this parameter when you retrieve a multi-directional terminology resource, you must specify the same format as the input file that was imported to create it. Otherwise, Amazon Translate throws an error.</p>
    pub fn set_terminology_data_format(
        mut self,
        input: ::std::option::Option<crate::types::TerminologyDataFormat>,
    ) -> Self {
        self.inner = self.inner.set_terminology_data_format(input);
        self
    }
}
