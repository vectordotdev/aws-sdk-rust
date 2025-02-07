// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deprovision_public_ipv4_pool_cidr::_deprovision_public_ipv4_pool_cidr_output::DeprovisionPublicIpv4PoolCidrOutputBuilder;

pub use crate::operation::deprovision_public_ipv4_pool_cidr::_deprovision_public_ipv4_pool_cidr_input::DeprovisionPublicIpv4PoolCidrInputBuilder;

/// Fluent builder constructing a request to `DeprovisionPublicIpv4PoolCidr`.
///
/// <p>Deprovision a CIDR from a public IPv4 pool.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeprovisionPublicIpv4PoolCidrFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::deprovision_public_ipv4_pool_cidr::builders::DeprovisionPublicIpv4PoolCidrInputBuilder,
}
impl DeprovisionPublicIpv4PoolCidrFluentBuilder {
    /// Creates a new `DeprovisionPublicIpv4PoolCidr`.
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
            crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidr,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrError,
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
        crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrError,
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
        crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrError,
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
            crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidr,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>The ID of the pool that you want to deprovision the CIDR from.</p>
    pub fn pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pool_id(input.into());
        self
    }
    /// <p>The ID of the pool that you want to deprovision the CIDR from.</p>
    pub fn set_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_pool_id(input);
        self
    }
    /// <p>The CIDR you want to deprovision from the pool. Enter the CIDR you want to deprovision with a netmask of <code>/32</code>. You must rerun this command for each IP address in the CIDR range. If your CIDR is a <code>/24</code>, you will have to run this command to deprovision each of the 256 IP addresses in the <code>/24</code> CIDR.</p>
    pub fn cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cidr(input.into());
        self
    }
    /// <p>The CIDR you want to deprovision from the pool. Enter the CIDR you want to deprovision with a netmask of <code>/32</code>. You must rerun this command for each IP address in the CIDR range. If your CIDR is a <code>/24</code>, you will have to run this command to deprovision each of the 256 IP addresses in the <code>/24</code> CIDR.</p>
    pub fn set_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cidr(input);
        self
    }
}
