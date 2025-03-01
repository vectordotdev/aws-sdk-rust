// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_type_registration::_describe_type_registration_output::DescribeTypeRegistrationOutputBuilder;

pub use crate::operation::describe_type_registration::_describe_type_registration_input::DescribeTypeRegistrationInputBuilder;

/// Fluent builder constructing a request to `DescribeTypeRegistration`.
///
/// <p>Returns information about an extension's registration, including its current status and type and version identifiers.</p>
/// <p>When you initiate a registration request using <code> <code>RegisterType</code> </code>, you can then use <code> <code>DescribeTypeRegistration</code> </code> to monitor the progress of that registration request.</p>
/// <p>Once the registration request has completed, use <code> <code>DescribeType</code> </code> to return detailed information about an extension.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeTypeRegistrationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_type_registration::builders::DescribeTypeRegistrationInputBuilder,
}
impl DescribeTypeRegistrationFluentBuilder {
    /// Creates a new `DescribeTypeRegistration`.
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
            crate::operation::describe_type_registration::DescribeTypeRegistration,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_type_registration::DescribeTypeRegistrationError,
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
        crate::operation::describe_type_registration::DescribeTypeRegistrationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_type_registration::DescribeTypeRegistrationError,
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
        crate::operation::describe_type_registration::DescribeTypeRegistrationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_type_registration::DescribeTypeRegistrationError,
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
            crate::operation::describe_type_registration::DescribeTypeRegistration,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_type_registration::DescribeTypeRegistrationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The identifier for this registration request.</p>
    /// <p>This registration token is generated by CloudFormation when you initiate a registration request using <code> <code>RegisterType</code> </code>.</p>
    pub fn registration_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.registration_token(input.into());
        self
    }
    /// <p>The identifier for this registration request.</p>
    /// <p>This registration token is generated by CloudFormation when you initiate a registration request using <code> <code>RegisterType</code> </code>.</p>
    pub fn set_registration_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_registration_token(input);
        self
    }
}
