// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_permission_set::_create_permission_set_output::CreatePermissionSetOutputBuilder;

pub use crate::operation::create_permission_set::_create_permission_set_input::CreatePermissionSetInputBuilder;

/// Fluent builder constructing a request to `CreatePermissionSet`.
///
/// <p>Creates a permission set within a specified IAM Identity Center instance.</p> <note>
/// <p>To grant users and groups access to AWS account resources, use <code> <code>CreateAccountAssignment</code> </code>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreatePermissionSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_permission_set::builders::CreatePermissionSetInputBuilder,
}
impl CreatePermissionSetFluentBuilder {
    /// Creates a new `CreatePermissionSet`.
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
            crate::operation::create_permission_set::CreatePermissionSet,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_permission_set::CreatePermissionSetError,
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
        crate::operation::create_permission_set::CreatePermissionSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_permission_set::CreatePermissionSetError,
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
        crate::operation::create_permission_set::CreatePermissionSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_permission_set::CreatePermissionSetError,
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
            crate::operation::create_permission_set::CreatePermissionSet,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_permission_set::CreatePermissionSetError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the <code>PermissionSet</code>.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the <code>PermissionSet</code>.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The description of the <code>PermissionSet</code>.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the <code>PermissionSet</code>.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
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
    /// <p>The length of time that the application user sessions are valid in the ISO-8601 standard.</p>
    pub fn session_duration(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.session_duration(input.into());
        self
    }
    /// <p>The length of time that the application user sessions are valid in the ISO-8601 standard.</p>
    pub fn set_session_duration(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_session_duration(input);
        self
    }
    /// <p>Used to redirect users within the application during the federation authentication process.</p>
    pub fn relay_state(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.relay_state(input.into());
        self
    }
    /// <p>Used to redirect users within the application during the federation authentication process.</p>
    pub fn set_relay_state(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_relay_state(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to attach to the new <code>PermissionSet</code>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to attach to the new <code>PermissionSet</code>.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
