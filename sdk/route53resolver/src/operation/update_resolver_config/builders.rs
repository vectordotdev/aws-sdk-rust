// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_resolver_config::_update_resolver_config_output::UpdateResolverConfigOutputBuilder;

pub use crate::operation::update_resolver_config::_update_resolver_config_input::UpdateResolverConfigInputBuilder;

/// Fluent builder constructing a request to `UpdateResolverConfig`.
///
/// <p>Updates the behavior configuration of Route&nbsp;53 Resolver behavior for a single VPC from Amazon Virtual Private Cloud.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateResolverConfigFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_resolver_config::builders::UpdateResolverConfigInputBuilder,
}
impl UpdateResolverConfigFluentBuilder {
    /// Creates a new `UpdateResolverConfig`.
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
            crate::operation::update_resolver_config::UpdateResolverConfig,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_resolver_config::UpdateResolverConfigError,
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
        crate::operation::update_resolver_config::UpdateResolverConfigOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_resolver_config::UpdateResolverConfigError,
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
        crate::operation::update_resolver_config::UpdateResolverConfigOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_resolver_config::UpdateResolverConfigError,
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
            crate::operation::update_resolver_config::UpdateResolverConfig,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_resolver_config::UpdateResolverConfigError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Resource ID of the Amazon VPC that you want to update the Resolver configuration for.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>Resource ID of the Amazon VPC that you want to update the Resolver configuration for.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>Indicates whether or not the Resolver will create autodefined rules for reverse DNS lookups. This is enabled by default. Disabling this option will also affect EC2-Classic instances using ClassicLink. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon EC2 guide</i>.</p> <important>
    /// <p>We are retiring EC2-Classic on August 15, 2022. We recommend that you migrate from EC2-Classic to a VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-migrate.html">Migrate from EC2-Classic to a VPC</a> in the <i>Amazon EC2 guide</i> and the blog <a href="http://aws.amazon.com/blogs/aws/ec2-classic-is-retiring-heres-how-to-prepare/">EC2-Classic Networking is Retiring – Here’s How to Prepare</a>.</p>
    /// </important> <note>
    /// <p>It can take some time for the status change to be completed.</p>
    /// </note>
    /// <p></p>
    pub fn autodefined_reverse_flag(mut self, input: crate::types::AutodefinedReverseFlag) -> Self {
        self.inner = self.inner.autodefined_reverse_flag(input);
        self
    }
    /// <p>Indicates whether or not the Resolver will create autodefined rules for reverse DNS lookups. This is enabled by default. Disabling this option will also affect EC2-Classic instances using ClassicLink. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-classiclink.html">ClassicLink</a> in the <i>Amazon EC2 guide</i>.</p> <important>
    /// <p>We are retiring EC2-Classic on August 15, 2022. We recommend that you migrate from EC2-Classic to a VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-migrate.html">Migrate from EC2-Classic to a VPC</a> in the <i>Amazon EC2 guide</i> and the blog <a href="http://aws.amazon.com/blogs/aws/ec2-classic-is-retiring-heres-how-to-prepare/">EC2-Classic Networking is Retiring – Here’s How to Prepare</a>.</p>
    /// </important> <note>
    /// <p>It can take some time for the status change to be completed.</p>
    /// </note>
    /// <p></p>
    pub fn set_autodefined_reverse_flag(
        mut self,
        input: ::std::option::Option<crate::types::AutodefinedReverseFlag>,
    ) -> Self {
        self.inner = self.inner.set_autodefined_reverse_flag(input);
        self
    }
}
