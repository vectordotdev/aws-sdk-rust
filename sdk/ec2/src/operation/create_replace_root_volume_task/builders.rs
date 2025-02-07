// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_replace_root_volume_task::_create_replace_root_volume_task_output::CreateReplaceRootVolumeTaskOutputBuilder;

pub use crate::operation::create_replace_root_volume_task::_create_replace_root_volume_task_input::CreateReplaceRootVolumeTaskInputBuilder;

/// Fluent builder constructing a request to `CreateReplaceRootVolumeTask`.
///
/// <p>Replaces the EBS-backed root volume for a <code>running</code> instance with a new volume that is restored to the original root volume's launch state, that is restored to a specific snapshot taken from the original root volume, or that is restored from an AMI that has the same key characteristics as that of the instance.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/replace-root.html">Replace a root volume</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateReplaceRootVolumeTaskFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_replace_root_volume_task::builders::CreateReplaceRootVolumeTaskInputBuilder,
}
impl CreateReplaceRootVolumeTaskFluentBuilder {
    /// Creates a new `CreateReplaceRootVolumeTask`.
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
            crate::operation::create_replace_root_volume_task::CreateReplaceRootVolumeTask,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_replace_root_volume_task::CreateReplaceRootVolumeTaskError,
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
        crate::operation::create_replace_root_volume_task::CreateReplaceRootVolumeTaskOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_replace_root_volume_task::CreateReplaceRootVolumeTaskError,
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
        crate::operation::create_replace_root_volume_task::CreateReplaceRootVolumeTaskOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_replace_root_volume_task::CreateReplaceRootVolumeTaskError,
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
            crate::operation::create_replace_root_volume_task::CreateReplaceRootVolumeTask,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_replace_root_volume_task::CreateReplaceRootVolumeTaskError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the instance for which to replace the root volume.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The ID of the instance for which to replace the root volume.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The ID of the snapshot from which to restore the replacement root volume. The specified snapshot must be a snapshot that you previously created from the original root volume.</p>
    /// <p>If you want to restore the replacement root volume to the initial launch state, or if you want to restore the replacement root volume from an AMI, omit this parameter.</p>
    pub fn snapshot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.snapshot_id(input.into());
        self
    }
    /// <p>The ID of the snapshot from which to restore the replacement root volume. The specified snapshot must be a snapshot that you previously created from the original root volume.</p>
    /// <p>If you want to restore the replacement root volume to the initial launch state, or if you want to restore the replacement root volume from an AMI, omit this parameter.</p>
    pub fn set_snapshot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_id(input);
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. If you do not specify a client token, a randomly generated token is used for the request to ensure idempotency. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. If you do not specify a client token, a randomly generated token is used for the request to ensure idempotency. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the root volume replacement task.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the root volume replacement task.</p>
    pub fn set_tag_specifications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The ID of the AMI to use to restore the root volume. The specified AMI must have the same product code, billing information, architecture type, and virtualization type as that of the instance.</p>
    /// <p>If you want to restore the replacement volume from a specific snapshot, or if you want to restore it to its launch state, omit this parameter.</p>
    pub fn image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.image_id(input.into());
        self
    }
    /// <p>The ID of the AMI to use to restore the root volume. The specified AMI must have the same product code, billing information, architecture type, and virtualization type as that of the instance.</p>
    /// <p>If you want to restore the replacement volume from a specific snapshot, or if you want to restore it to its launch state, omit this parameter.</p>
    pub fn set_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_image_id(input);
        self
    }
    /// <p>Indicates whether to automatically delete the original root volume after the root volume replacement task completes. To delete the original root volume, specify <code>true</code>. If you choose to keep the original root volume after the replacement task completes, you must manually delete it when you no longer need it.</p>
    pub fn delete_replaced_root_volume(mut self, input: bool) -> Self {
        self.inner = self.inner.delete_replaced_root_volume(input);
        self
    }
    /// <p>Indicates whether to automatically delete the original root volume after the root volume replacement task completes. To delete the original root volume, specify <code>true</code>. If you choose to keep the original root volume after the replacement task completes, you must manually delete it when you no longer need it.</p>
    pub fn set_delete_replaced_root_volume(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_delete_replaced_root_volume(input);
        self
    }
}
