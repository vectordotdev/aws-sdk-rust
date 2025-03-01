// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_customer_gateway::_associate_customer_gateway_output::AssociateCustomerGatewayOutputBuilder;

pub use crate::operation::associate_customer_gateway::_associate_customer_gateway_input::AssociateCustomerGatewayInputBuilder;

/// Fluent builder constructing a request to `AssociateCustomerGateway`.
///
/// <p>Associates a customer gateway with a device and optionally, with a link. If you specify a link, it must be associated with the specified device. </p>
/// <p>You can only associate customer gateways that are connected to a VPN attachment on a transit gateway or core network registered in your global network. When you register a transit gateway or core network, customer gateways that are connected to the transit gateway are automatically included in the global network. To list customer gateways that are connected to a transit gateway, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeVpnConnections.html">DescribeVpnConnections</a> EC2 API and filter by <code>transit-gateway-id</code>.</p>
/// <p>You cannot associate a customer gateway with more than one device and link. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateCustomerGatewayFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::associate_customer_gateway::builders::AssociateCustomerGatewayInputBuilder,
}
impl AssociateCustomerGatewayFluentBuilder {
    /// Creates a new `AssociateCustomerGateway`.
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
            crate::operation::associate_customer_gateway::AssociateCustomerGateway,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_customer_gateway::AssociateCustomerGatewayError,
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
        crate::operation::associate_customer_gateway::AssociateCustomerGatewayOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_customer_gateway::AssociateCustomerGatewayError,
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
        crate::operation::associate_customer_gateway::AssociateCustomerGatewayOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_customer_gateway::AssociateCustomerGatewayError,
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
            crate::operation::associate_customer_gateway::AssociateCustomerGateway,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::associate_customer_gateway::AssociateCustomerGatewayError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (ARN) of the customer gateway.</p>
    pub fn customer_gateway_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.customer_gateway_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the customer gateway.</p>
    pub fn set_customer_gateway_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_customer_gateway_arn(input);
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn global_network_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.global_network_id(input.into());
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn set_global_network_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_global_network_id(input);
        self
    }
    /// <p>The ID of the device.</p>
    pub fn device_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.device_id(input.into());
        self
    }
    /// <p>The ID of the device.</p>
    pub fn set_device_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_device_id(input);
        self
    }
    /// <p>The ID of the link.</p>
    pub fn link_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.link_id(input.into());
        self
    }
    /// <p>The ID of the link.</p>
    pub fn set_link_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_link_id(input);
        self
    }
}
