// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::attach_load_balancer_tls_certificate::_attach_load_balancer_tls_certificate_output::AttachLoadBalancerTlsCertificateOutputBuilder;

pub use crate::operation::attach_load_balancer_tls_certificate::_attach_load_balancer_tls_certificate_input::AttachLoadBalancerTlsCertificateInputBuilder;

/// Fluent builder constructing a request to `AttachLoadBalancerTlsCertificate`.
///
/// <p>Attaches a Transport Layer Security (TLS) certificate to your load balancer. TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p>
/// <p>Once you create and validate your certificate, you can attach it to your load balancer. You can also use this API to rotate the certificates on your account. Use the <code>AttachLoadBalancerTlsCertificate</code> action with the non-attached certificate, and it will replace the existing one and become the attached certificate.</p>
/// <p>The <code>AttachLoadBalancerTlsCertificate</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-controlling-access-using-tags">Amazon Lightsail Developer Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AttachLoadBalancerTlsCertificateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::attach_load_balancer_tls_certificate::builders::AttachLoadBalancerTlsCertificateInputBuilder,
}
impl AttachLoadBalancerTlsCertificateFluentBuilder {
    /// Creates a new `AttachLoadBalancerTlsCertificate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificate, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateOutput, ::aws_smithy_http::result::SdkError<crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateOutput, ::aws_smithy_http::result::SdkError<crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificate, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError>
    >{
        self.customize_middleware().await
    }
    /// <p>The name of the load balancer to which you want to associate the SSL/TLS certificate.</p>
    pub fn load_balancer_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.load_balancer_name(input.into());
        self
    }
    /// <p>The name of the load balancer to which you want to associate the SSL/TLS certificate.</p>
    pub fn set_load_balancer_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_load_balancer_name(input);
        self
    }
    /// <p>The name of your SSL/TLS certificate.</p>
    pub fn certificate_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.certificate_name(input.into());
        self
    }
    /// <p>The name of your SSL/TLS certificate.</p>
    pub fn set_certificate_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_certificate_name(input);
        self
    }
}
