// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_kinesis_streaming_destination::_describe_kinesis_streaming_destination_output::DescribeKinesisStreamingDestinationOutputBuilder;

pub use crate::operation::describe_kinesis_streaming_destination::_describe_kinesis_streaming_destination_input::DescribeKinesisStreamingDestinationInputBuilder;

/// Fluent builder constructing a request to `DescribeKinesisStreamingDestination`.
///
/// <p>Returns information about the status of Kinesis streaming.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeKinesisStreamingDestinationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_kinesis_streaming_destination::builders::DescribeKinesisStreamingDestinationInputBuilder,
}
impl DescribeKinesisStreamingDestinationFluentBuilder {
    /// Creates a new `DescribeKinesisStreamingDestination`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestination, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationOutput, ::aws_smithy_http::result::SdkError<crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationOutput, ::aws_smithy_http::result::SdkError<crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestination, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::describe_kinesis_streaming_destination::DescribeKinesisStreamingDestinationError>
    >{
        self.customize_middleware().await
    }
    /// <p>The name of the table being described.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table being described.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
}
