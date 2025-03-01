// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::attach_instances_to_load_balancer::_attach_instances_to_load_balancer_output::AttachInstancesToLoadBalancerOutputBuilder;

pub use crate::operation::attach_instances_to_load_balancer::_attach_instances_to_load_balancer_input::AttachInstancesToLoadBalancerInputBuilder;

/// Fluent builder constructing a request to `AttachInstancesToLoadBalancer`.
///
/// <p>Attaches one or more Lightsail instances to a load balancer.</p>
/// <p>After some time, the instances are attached to the load balancer and the health check status is available.</p>
/// <p>The <code>attach instances to load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Developer Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AttachInstancesToLoadBalancerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::attach_instances_to_load_balancer::builders::AttachInstancesToLoadBalancerInputBuilder,
}
impl AttachInstancesToLoadBalancerFluentBuilder {
    /// Creates a new `AttachInstancesToLoadBalancer`.
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
            crate::operation::attach_instances_to_load_balancer::AttachInstancesToLoadBalancer,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::attach_instances_to_load_balancer::AttachInstancesToLoadBalancerError,
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
        crate::operation::attach_instances_to_load_balancer::AttachInstancesToLoadBalancerOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::attach_instances_to_load_balancer::AttachInstancesToLoadBalancerError,
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
        crate::operation::attach_instances_to_load_balancer::AttachInstancesToLoadBalancerOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::attach_instances_to_load_balancer::AttachInstancesToLoadBalancerError,
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
            crate::operation::attach_instances_to_load_balancer::AttachInstancesToLoadBalancer,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::attach_instances_to_load_balancer::AttachInstancesToLoadBalancerError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the load balancer.</p>
    pub fn load_balancer_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.load_balancer_name(input.into());
        self
    }
    /// <p>The name of the load balancer.</p>
    pub fn set_load_balancer_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_load_balancer_name(input);
        self
    }
    /// Appends an item to `instanceNames`.
    ///
    /// To override the contents of this collection use [`set_instance_names`](Self::set_instance_names).
    ///
    /// <p>An array of strings representing the instance name(s) you want to attach to your load balancer.</p>
    /// <p>An instance must be <code>running</code> before you can attach it to your load balancer.</p>
    /// <p>There are no additional limits on the number of instances you can attach to your load balancer, aside from the limit of Lightsail instances you can create in your account (20).</p>
    pub fn instance_names(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.instance_names(input.into());
        self
    }
    /// <p>An array of strings representing the instance name(s) you want to attach to your load balancer.</p>
    /// <p>An instance must be <code>running</code> before you can attach it to your load balancer.</p>
    /// <p>There are no additional limits on the number of instances you can attach to your load balancer, aside from the limit of Lightsail instances you can create in your account (20).</p>
    pub fn set_instance_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_instance_names(input);
        self
    }
}
