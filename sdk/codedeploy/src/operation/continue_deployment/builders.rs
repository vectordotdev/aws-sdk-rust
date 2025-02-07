// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::continue_deployment::_continue_deployment_output::ContinueDeploymentOutputBuilder;

pub use crate::operation::continue_deployment::_continue_deployment_input::ContinueDeploymentInputBuilder;

/// Fluent builder constructing a request to `ContinueDeployment`.
///
/// <p>For a blue/green deployment, starts the process of rerouting traffic from instances in the original environment to instances in the replacement environment without waiting for a specified wait time to elapse. (Traffic rerouting, which is achieved by registering instances in the replacement environment with the load balancer, can start as soon as all instances have a status of Ready.) </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ContinueDeploymentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::continue_deployment::builders::ContinueDeploymentInputBuilder,
}
impl ContinueDeploymentFluentBuilder {
    /// Creates a new `ContinueDeployment`.
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
            crate::operation::continue_deployment::ContinueDeployment,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::continue_deployment::ContinueDeploymentError,
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
        crate::operation::continue_deployment::ContinueDeploymentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::continue_deployment::ContinueDeploymentError,
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
        crate::operation::continue_deployment::ContinueDeploymentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::continue_deployment::ContinueDeploymentError,
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
            crate::operation::continue_deployment::ContinueDeployment,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::continue_deployment::ContinueDeploymentError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p> The unique ID of a blue/green deployment for which you want to start rerouting traffic to the replacement environment. </p>
    pub fn deployment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.deployment_id(input.into());
        self
    }
    /// <p> The unique ID of a blue/green deployment for which you want to start rerouting traffic to the replacement environment. </p>
    pub fn set_deployment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_deployment_id(input);
        self
    }
    /// <p> The status of the deployment's waiting period. <code>READY_WAIT</code> indicates that the deployment is ready to start shifting traffic. <code>TERMINATION_WAIT</code> indicates that the traffic is shifted, but the original target is not terminated. </p>
    pub fn deployment_wait_type(mut self, input: crate::types::DeploymentWaitType) -> Self {
        self.inner = self.inner.deployment_wait_type(input);
        self
    }
    /// <p> The status of the deployment's waiting period. <code>READY_WAIT</code> indicates that the deployment is ready to start shifting traffic. <code>TERMINATION_WAIT</code> indicates that the traffic is shifted, but the original target is not terminated. </p>
    pub fn set_deployment_wait_type(
        mut self,
        input: ::std::option::Option<crate::types::DeploymentWaitType>,
    ) -> Self {
        self.inner = self.inner.set_deployment_wait_type(input);
        self
    }
}
