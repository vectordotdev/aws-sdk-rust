// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_transit_gateway_route::_create_transit_gateway_route_output::CreateTransitGatewayRouteOutputBuilder;

pub use crate::operation::create_transit_gateway_route::_create_transit_gateway_route_input::CreateTransitGatewayRouteInputBuilder;

/// Fluent builder constructing a request to `CreateTransitGatewayRoute`.
///
/// <p>Creates a static route for the specified transit gateway route table.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateTransitGatewayRouteFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteInputBuilder,
}
impl CreateTransitGatewayRouteFluentBuilder {
    /// Creates a new `CreateTransitGatewayRoute`.
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
            crate::operation::create_transit_gateway_route::CreateTransitGatewayRoute,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteError,
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
        crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteError,
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
        crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteError,
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
            crate::operation::create_transit_gateway_route::CreateTransitGatewayRoute,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The CIDR range used for destination matches. Routing decisions are based on the most specific match.</p>
    pub fn destination_cidr_block(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.destination_cidr_block(input.into());
        self
    }
    /// <p>The CIDR range used for destination matches. Routing decisions are based on the most specific match.</p>
    pub fn set_destination_cidr_block(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_destination_cidr_block(input);
        self
    }
    /// <p>The ID of the transit gateway route table.</p>
    pub fn transit_gateway_route_table_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.transit_gateway_route_table_id(input.into());
        self
    }
    /// <p>The ID of the transit gateway route table.</p>
    pub fn set_transit_gateway_route_table_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_transit_gateway_route_table_id(input);
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn transit_gateway_attachment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.transit_gateway_attachment_id(input.into());
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn set_transit_gateway_attachment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_transit_gateway_attachment_id(input);
        self
    }
    /// <p>Indicates whether to drop traffic that matches this route.</p>
    pub fn blackhole(mut self, input: bool) -> Self {
        self.inner = self.inner.blackhole(input);
        self
    }
    /// <p>Indicates whether to drop traffic that matches this route.</p>
    pub fn set_blackhole(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_blackhole(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
}
