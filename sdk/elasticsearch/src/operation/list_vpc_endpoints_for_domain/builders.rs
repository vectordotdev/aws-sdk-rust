// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_vpc_endpoints_for_domain::_list_vpc_endpoints_for_domain_output::ListVpcEndpointsForDomainOutputBuilder;

pub use crate::operation::list_vpc_endpoints_for_domain::_list_vpc_endpoints_for_domain_input::ListVpcEndpointsForDomainInputBuilder;

/// Fluent builder constructing a request to `ListVpcEndpointsForDomain`.
///
/// <p>Retrieves all Amazon OpenSearch Service-managed VPC endpoints associated with a particular domain.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListVpcEndpointsForDomainFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::list_vpc_endpoints_for_domain::builders::ListVpcEndpointsForDomainInputBuilder,
}
impl ListVpcEndpointsForDomainFluentBuilder {
    /// Creates a new `ListVpcEndpointsForDomain`.
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
            crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomain,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError,
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
        crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError,
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
        crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError,
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
            crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomain,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Name of the ElasticSearch domain whose VPC endpoints are to be listed.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>Name of the ElasticSearch domain whose VPC endpoints are to be listed.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
