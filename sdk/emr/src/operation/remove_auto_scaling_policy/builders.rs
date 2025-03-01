// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_auto_scaling_policy::_remove_auto_scaling_policy_output::RemoveAutoScalingPolicyOutputBuilder;

pub use crate::operation::remove_auto_scaling_policy::_remove_auto_scaling_policy_input::RemoveAutoScalingPolicyInputBuilder;

/// Fluent builder constructing a request to `RemoveAutoScalingPolicy`.
///
/// <p>Removes an automatic scaling policy from a specified instance group within an Amazon EMR cluster.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RemoveAutoScalingPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::remove_auto_scaling_policy::builders::RemoveAutoScalingPolicyInputBuilder,
}
impl RemoveAutoScalingPolicyFluentBuilder {
    /// Creates a new `RemoveAutoScalingPolicy`.
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
            crate::operation::remove_auto_scaling_policy::RemoveAutoScalingPolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_auto_scaling_policy::RemoveAutoScalingPolicyError,
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
        crate::operation::remove_auto_scaling_policy::RemoveAutoScalingPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_auto_scaling_policy::RemoveAutoScalingPolicyError,
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
        crate::operation::remove_auto_scaling_policy::RemoveAutoScalingPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_auto_scaling_policy::RemoveAutoScalingPolicyError,
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
            crate::operation::remove_auto_scaling_policy::RemoveAutoScalingPolicy,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_auto_scaling_policy::RemoveAutoScalingPolicyError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy is applied is within this cluster.</p>
    pub fn cluster_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_id(input.into());
        self
    }
    /// <p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy is applied is within this cluster.</p>
    pub fn set_cluster_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_id(input);
        self
    }
    /// <p>Specifies the ID of the instance group to which the scaling policy is applied.</p>
    pub fn instance_group_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.instance_group_id(input.into());
        self
    }
    /// <p>Specifies the ID of the instance group to which the scaling policy is applied.</p>
    pub fn set_instance_group_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_instance_group_id(input);
        self
    }
}
