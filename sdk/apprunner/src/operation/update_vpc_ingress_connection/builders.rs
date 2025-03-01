// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_vpc_ingress_connection::_update_vpc_ingress_connection_output::UpdateVpcIngressConnectionOutputBuilder;

pub use crate::operation::update_vpc_ingress_connection::_update_vpc_ingress_connection_input::UpdateVpcIngressConnectionInputBuilder;

/// Fluent builder constructing a request to `UpdateVpcIngressConnection`.
///
/// <p>Update an existing App Runner VPC Ingress Connection resource. The VPC Ingress Connection must be in one of the following states to be updated:</p>
/// <ul>
/// <li> <p> AVAILABLE </p> </li>
/// <li> <p> FAILED_CREATION </p> </li>
/// <li> <p> FAILED_UPDATE </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateVpcIngressConnectionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_vpc_ingress_connection::builders::UpdateVpcIngressConnectionInputBuilder,
}
impl UpdateVpcIngressConnectionFluentBuilder {
    /// Creates a new `UpdateVpcIngressConnection`.
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
            crate::operation::update_vpc_ingress_connection::UpdateVpcIngressConnection,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_vpc_ingress_connection::UpdateVpcIngressConnectionError,
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
        crate::operation::update_vpc_ingress_connection::UpdateVpcIngressConnectionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_vpc_ingress_connection::UpdateVpcIngressConnectionError,
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
        crate::operation::update_vpc_ingress_connection::UpdateVpcIngressConnectionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_vpc_ingress_connection::UpdateVpcIngressConnectionError,
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
            crate::operation::update_vpc_ingress_connection::UpdateVpcIngressConnection,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_vpc_ingress_connection::UpdateVpcIngressConnectionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (Arn) for the App Runner VPC Ingress Connection resource that you want to update.</p>
    pub fn vpc_ingress_connection_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.vpc_ingress_connection_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (Arn) for the App Runner VPC Ingress Connection resource that you want to update.</p>
    pub fn set_vpc_ingress_connection_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_vpc_ingress_connection_arn(input);
        self
    }
    /// <p>Specifications for the customer’s Amazon VPC and the related Amazon Web Services PrivateLink VPC endpoint that are used to update the VPC Ingress Connection resource.</p>
    pub fn ingress_vpc_configuration(
        mut self,
        input: crate::types::IngressVpcConfiguration,
    ) -> Self {
        self.inner = self.inner.ingress_vpc_configuration(input);
        self
    }
    /// <p>Specifications for the customer’s Amazon VPC and the related Amazon Web Services PrivateLink VPC endpoint that are used to update the VPC Ingress Connection resource.</p>
    pub fn set_ingress_vpc_configuration(
        mut self,
        input: ::std::option::Option<crate::types::IngressVpcConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_ingress_vpc_configuration(input);
        self
    }
}
