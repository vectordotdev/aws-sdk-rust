// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::reject_client_vpc_connection::_reject_client_vpc_connection_output::RejectClientVpcConnectionOutputBuilder;

pub use crate::operation::reject_client_vpc_connection::_reject_client_vpc_connection_input::RejectClientVpcConnectionInputBuilder;

/// Fluent builder constructing a request to `RejectClientVpcConnection`.
///
/// <p>Returns empty response.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RejectClientVpcConnectionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::reject_client_vpc_connection::builders::RejectClientVpcConnectionInputBuilder,
}
impl RejectClientVpcConnectionFluentBuilder {
    /// Creates a new `RejectClientVpcConnection`.
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
            crate::operation::reject_client_vpc_connection::RejectClientVpcConnection,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::reject_client_vpc_connection::RejectClientVpcConnectionError,
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
        crate::operation::reject_client_vpc_connection::RejectClientVpcConnectionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::reject_client_vpc_connection::RejectClientVpcConnectionError,
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
        crate::operation::reject_client_vpc_connection::RejectClientVpcConnectionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::reject_client_vpc_connection::RejectClientVpcConnectionError,
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
            crate::operation::reject_client_vpc_connection::RejectClientVpcConnection,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::reject_client_vpc_connection::RejectClientVpcConnectionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    pub fn cluster_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    pub fn set_cluster_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_arn(input);
        self
    }
    /// <p>The VPC connection ARN.</p>
    pub fn vpc_connection_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.vpc_connection_arn(input.into());
        self
    }
    /// <p>The VPC connection ARN.</p>
    pub fn set_vpc_connection_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_vpc_connection_arn(input);
        self
    }
}
