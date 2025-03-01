// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_transit_gateway_route_table_attachment::_create_transit_gateway_route_table_attachment_output::CreateTransitGatewayRouteTableAttachmentOutputBuilder;

pub use crate::operation::create_transit_gateway_route_table_attachment::_create_transit_gateway_route_table_attachment_input::CreateTransitGatewayRouteTableAttachmentInputBuilder;

/// Fluent builder constructing a request to `CreateTransitGatewayRouteTableAttachment`.
///
/// <p>Creates a transit gateway route table attachment.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateTransitGatewayRouteTableAttachmentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_transit_gateway_route_table_attachment::builders::CreateTransitGatewayRouteTableAttachmentInputBuilder,
}
impl CreateTransitGatewayRouteTableAttachmentFluentBuilder {
    /// Creates a new `CreateTransitGatewayRouteTableAttachment`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::create_transit_gateway_route_table_attachment::CreateTransitGatewayRouteTableAttachment, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::create_transit_gateway_route_table_attachment::CreateTransitGatewayRouteTableAttachmentError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::create_transit_gateway_route_table_attachment::CreateTransitGatewayRouteTableAttachmentOutput, ::aws_smithy_http::result::SdkError<crate::operation::create_transit_gateway_route_table_attachment::CreateTransitGatewayRouteTableAttachmentError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::create_transit_gateway_route_table_attachment::CreateTransitGatewayRouteTableAttachmentOutput, ::aws_smithy_http::result::SdkError<crate::operation::create_transit_gateway_route_table_attachment::CreateTransitGatewayRouteTableAttachmentError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::create_transit_gateway_route_table_attachment::CreateTransitGatewayRouteTableAttachment, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::create_transit_gateway_route_table_attachment::CreateTransitGatewayRouteTableAttachmentError>
    >{
        self.customize_middleware().await
    }
    /// <p>The ID of the peer for the </p>
    pub fn peering_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.peering_id(input.into());
        self
    }
    /// <p>The ID of the peer for the </p>
    pub fn set_peering_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_peering_id(input);
        self
    }
    /// <p>The ARN of the transit gateway route table for the attachment request. For example, <code>"TransitGatewayRouteTableArn": "arn:aws:ec2:us-west-2:123456789012:transit-gateway-route-table/tgw-rtb-9876543210123456"</code>.</p>
    pub fn transit_gateway_route_table_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.transit_gateway_route_table_arn(input.into());
        self
    }
    /// <p>The ARN of the transit gateway route table for the attachment request. For example, <code>"TransitGatewayRouteTableArn": "arn:aws:ec2:us-west-2:123456789012:transit-gateway-route-table/tgw-rtb-9876543210123456"</code>.</p>
    pub fn set_transit_gateway_route_table_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_transit_gateway_route_table_arn(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of key-value tags associated with the request.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The list of key-value tags associated with the request.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The client token associated with the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The client token associated with the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
