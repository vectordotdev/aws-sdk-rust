// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_service_primary_task_set::_update_service_primary_task_set_output::UpdateServicePrimaryTaskSetOutputBuilder;

pub use crate::operation::update_service_primary_task_set::_update_service_primary_task_set_input::UpdateServicePrimaryTaskSetInputBuilder;

/// Fluent builder constructing a request to `UpdateServicePrimaryTaskSet`.
///
/// <p>Modifies which task set in a service is the primary task set. Any parameters that are updated on the primary task set in a service will transition to the service. This is used when a service uses the <code>EXTERNAL</code> deployment controller type. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS Deployment Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateServicePrimaryTaskSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_service_primary_task_set::builders::UpdateServicePrimaryTaskSetInputBuilder,
}
impl UpdateServicePrimaryTaskSetFluentBuilder {
    /// Creates a new `UpdateServicePrimaryTaskSet`.
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
            crate::operation::update_service_primary_task_set::UpdateServicePrimaryTaskSet,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_primary_task_set::UpdateServicePrimaryTaskSetError,
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
        crate::operation::update_service_primary_task_set::UpdateServicePrimaryTaskSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_primary_task_set::UpdateServicePrimaryTaskSetError,
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
        crate::operation::update_service_primary_task_set::UpdateServicePrimaryTaskSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_primary_task_set::UpdateServicePrimaryTaskSetError,
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
            crate::operation::update_service_primary_task_set::UpdateServicePrimaryTaskSet,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_primary_task_set::UpdateServicePrimaryTaskSetError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task set exists in.</p>
    pub fn cluster(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster(input.into());
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task set exists in.</p>
    pub fn set_cluster(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster(input);
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the service that the task set exists in.</p>
    pub fn service(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service(input.into());
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the service that the task set exists in.</p>
    pub fn set_service(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service(input);
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the task set to set as the primary task set in the deployment.</p>
    pub fn primary_task_set(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.primary_task_set(input.into());
        self
    }
    /// <p>The short name or full Amazon Resource Name (ARN) of the task set to set as the primary task set in the deployment.</p>
    pub fn set_primary_task_set(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_primary_task_set(input);
        self
    }
}
