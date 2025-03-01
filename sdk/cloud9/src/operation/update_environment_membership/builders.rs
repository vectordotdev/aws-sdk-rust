// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_environment_membership::_update_environment_membership_output::UpdateEnvironmentMembershipOutputBuilder;

pub use crate::operation::update_environment_membership::_update_environment_membership_input::UpdateEnvironmentMembershipInputBuilder;

/// Fluent builder constructing a request to `UpdateEnvironmentMembership`.
///
/// <p>Changes the settings of an existing environment member for an Cloud9 development environment.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateEnvironmentMembershipFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_environment_membership::builders::UpdateEnvironmentMembershipInputBuilder,
}
impl UpdateEnvironmentMembershipFluentBuilder {
    /// Creates a new `UpdateEnvironmentMembership`.
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
            crate::operation::update_environment_membership::UpdateEnvironmentMembership,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_environment_membership::UpdateEnvironmentMembershipError,
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
        crate::operation::update_environment_membership::UpdateEnvironmentMembershipOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_environment_membership::UpdateEnvironmentMembershipError,
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
        crate::operation::update_environment_membership::UpdateEnvironmentMembershipOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_environment_membership::UpdateEnvironmentMembershipError,
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
            crate::operation::update_environment_membership::UpdateEnvironmentMembership,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_environment_membership::UpdateEnvironmentMembershipError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the environment for the environment member whose settings you want to change.</p>
    pub fn environment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.environment_id(input.into());
        self
    }
    /// <p>The ID of the environment for the environment member whose settings you want to change.</p>
    pub fn set_environment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_environment_id(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the environment member whose settings you want to change.</p>
    pub fn user_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the environment member whose settings you want to change.</p>
    pub fn set_user_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_arn(input);
        self
    }
    /// <p>The replacement type of environment member permissions you want to associate with this environment member. Available values include:</p>
    /// <ul>
    /// <li> <p> <code>read-only</code>: Has read-only access to the environment.</p> </li>
    /// <li> <p> <code>read-write</code>: Has read-write access to the environment.</p> </li>
    /// </ul>
    pub fn permissions(mut self, input: crate::types::MemberPermissions) -> Self {
        self.inner = self.inner.permissions(input);
        self
    }
    /// <p>The replacement type of environment member permissions you want to associate with this environment member. Available values include:</p>
    /// <ul>
    /// <li> <p> <code>read-only</code>: Has read-only access to the environment.</p> </li>
    /// <li> <p> <code>read-write</code>: Has read-write access to the environment.</p> </li>
    /// </ul>
    pub fn set_permissions(
        mut self,
        input: ::std::option::Option<crate::types::MemberPermissions>,
    ) -> Self {
        self.inner = self.inner.set_permissions(input);
        self
    }
}
