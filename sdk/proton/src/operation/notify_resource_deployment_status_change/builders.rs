// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::notify_resource_deployment_status_change::_notify_resource_deployment_status_change_output::NotifyResourceDeploymentStatusChangeOutputBuilder;

pub use crate::operation::notify_resource_deployment_status_change::_notify_resource_deployment_status_change_input::NotifyResourceDeploymentStatusChangeInputBuilder;

/// Fluent builder constructing a request to `NotifyResourceDeploymentStatusChange`.
///
/// <p>Notify Proton of status changes to a provisioned resource when you use self-managed provisioning.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/ag-works-prov-methods.html#ag-works-prov-methods-self">Self-managed provisioning</a> in the <i>Proton User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct NotifyResourceDeploymentStatusChangeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeInputBuilder,
}
impl NotifyResourceDeploymentStatusChangeFluentBuilder {
    /// Creates a new `NotifyResourceDeploymentStatusChange`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::notify_resource_deployment_status_change::NotifyResourceDeploymentStatusChange, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::notify_resource_deployment_status_change::NotifyResourceDeploymentStatusChangeError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::notify_resource_deployment_status_change::NotifyResourceDeploymentStatusChangeOutput, ::aws_smithy_http::result::SdkError<crate::operation::notify_resource_deployment_status_change::NotifyResourceDeploymentStatusChangeError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::notify_resource_deployment_status_change::NotifyResourceDeploymentStatusChangeOutput, ::aws_smithy_http::result::SdkError<crate::operation::notify_resource_deployment_status_change::NotifyResourceDeploymentStatusChangeError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::notify_resource_deployment_status_change::NotifyResourceDeploymentStatusChange, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::notify_resource_deployment_status_change::NotifyResourceDeploymentStatusChangeError>
    >{
        self.customize_middleware().await
    }
    /// <p>The provisioned resource Amazon Resource Name (ARN).</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The provisioned resource Amazon Resource Name (ARN).</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>The status of your provisioned resource.</p>
    pub fn status(mut self, input: crate::types::ResourceDeploymentStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>The status of your provisioned resource.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::ResourceDeploymentStatus>,
    ) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// Appends an item to `outputs`.
    ///
    /// To override the contents of this collection use [`set_outputs`](Self::set_outputs).
    ///
    /// <p>The provisioned resource state change detail data that's returned by Proton.</p>
    pub fn outputs(mut self, input: crate::types::Output) -> Self {
        self.inner = self.inner.outputs(input);
        self
    }
    /// <p>The provisioned resource state change detail data that's returned by Proton.</p>
    pub fn set_outputs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Output>>,
    ) -> Self {
        self.inner = self.inner.set_outputs(input);
        self
    }
    /// <p>The deployment ID for your provisioned resource.</p>
    pub fn deployment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.deployment_id(input.into());
        self
    }
    /// <p>The deployment ID for your provisioned resource.</p>
    pub fn set_deployment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_deployment_id(input);
        self
    }
    /// <p>The deployment status message for your provisioned resource.</p>
    pub fn status_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.status_message(input.into());
        self
    }
    /// <p>The deployment status message for your provisioned resource.</p>
    pub fn set_status_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_status_message(input);
        self
    }
}
