// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::attach_managed_policy_to_permission_set::_attach_managed_policy_to_permission_set_output::AttachManagedPolicyToPermissionSetOutputBuilder;

pub use crate::operation::attach_managed_policy_to_permission_set::_attach_managed_policy_to_permission_set_input::AttachManagedPolicyToPermissionSetInputBuilder;

/// Fluent builder constructing a request to `AttachManagedPolicyToPermissionSet`.
///
/// <p>Attaches an AWS managed policy ARN to a permission set.</p> <note>
/// <p>If the permission set is already referenced by one or more account assignments, you will need to call <code> <code>ProvisionPermissionSet</code> </code> after this operation. Calling <code>ProvisionPermissionSet</code> applies the corresponding IAM policy updates to all assigned accounts.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AttachManagedPolicyToPermissionSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::attach_managed_policy_to_permission_set::builders::AttachManagedPolicyToPermissionSetInputBuilder,
}
impl AttachManagedPolicyToPermissionSetFluentBuilder {
    /// Creates a new `AttachManagedPolicyToPermissionSet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::attach_managed_policy_to_permission_set::AttachManagedPolicyToPermissionSet, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::attach_managed_policy_to_permission_set::AttachManagedPolicyToPermissionSetError>
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::attach_managed_policy_to_permission_set::AttachManagedPolicyToPermissionSetOutput, ::aws_smithy_http::result::SdkError<crate::operation::attach_managed_policy_to_permission_set::AttachManagedPolicyToPermissionSetError>>
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::attach_managed_policy_to_permission_set::AttachManagedPolicyToPermissionSetOutput, ::aws_smithy_http::result::SdkError<crate::operation::attach_managed_policy_to_permission_set::AttachManagedPolicyToPermissionSetError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::attach_managed_policy_to_permission_set::AttachManagedPolicyToPermissionSet, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::attach_managed_policy_to_permission_set::AttachManagedPolicyToPermissionSetError>
    >{
        self.customize_middleware().await
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    pub fn instance_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_arn(input.into());
        self
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    pub fn set_instance_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_arn(input);
        self
    }
    /// <p>The ARN of the <code>PermissionSet</code> that the managed policy should be attached to.</p>
    pub fn permission_set_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.permission_set_arn(input.into());
        self
    }
    /// <p>The ARN of the <code>PermissionSet</code> that the managed policy should be attached to.</p>
    pub fn set_permission_set_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_permission_set_arn(input);
        self
    }
    /// <p>The AWS managed policy ARN to be attached to a permission set.</p>
    pub fn managed_policy_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.managed_policy_arn(input.into());
        self
    }
    /// <p>The AWS managed policy ARN to be attached to a permission set.</p>
    pub fn set_managed_policy_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_managed_policy_arn(input);
        self
    }
}
