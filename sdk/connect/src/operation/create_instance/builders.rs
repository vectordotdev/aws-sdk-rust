// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_instance::_create_instance_output::CreateInstanceOutputBuilder;

pub use crate::operation::create_instance::_create_instance_input::CreateInstanceInputBuilder;

/// Fluent builder constructing a request to `CreateInstance`.
///
/// <p>This API is in preview release for Amazon Connect and is subject to change.</p>
/// <p>Initiates an Amazon Connect instance with all the supported channels enabled. It does not attach any storage, such as Amazon Simple Storage Service (Amazon S3) or Amazon Kinesis. It also does not allow for any configurations on features, such as Contact Lens for Amazon Connect. </p>
/// <p>Amazon Connect enforces a limit on the total number of instances that you can create or delete in 30 days. If you exceed this limit, you will get an error message indicating there has been an excessive number of attempts at creating or deleting instances. You must wait 30 days before you can restart creating and deleting instances in your account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateInstanceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_instance::builders::CreateInstanceInputBuilder,
}
impl CreateInstanceFluentBuilder {
    /// Creates a new `CreateInstance`.
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
            crate::operation::create_instance::CreateInstance,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_instance::CreateInstanceError>,
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
        crate::operation::create_instance::CreateInstanceOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_instance::CreateInstanceError>,
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
        crate::operation::create_instance::CreateInstanceOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_instance::CreateInstanceError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_instance::CreateInstance,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_instance::CreateInstanceError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The idempotency token.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The idempotency token.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The type of identity management for your Amazon Connect users.</p>
    pub fn identity_management_type(mut self, input: crate::types::DirectoryType) -> Self {
        self.inner = self.inner.identity_management_type(input);
        self
    }
    /// <p>The type of identity management for your Amazon Connect users.</p>
    pub fn set_identity_management_type(
        mut self,
        input: ::std::option::Option<crate::types::DirectoryType>,
    ) -> Self {
        self.inner = self.inner.set_identity_management_type(input);
        self
    }
    /// <p>The name for your instance.</p>
    pub fn instance_alias(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.instance_alias(input.into());
        self
    }
    /// <p>The name for your instance.</p>
    pub fn set_instance_alias(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_instance_alias(input);
        self
    }
    /// <p>The identifier for the directory.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The identifier for the directory.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>Your contact center handles incoming contacts.</p>
    pub fn inbound_calls_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.inbound_calls_enabled(input);
        self
    }
    /// <p>Your contact center handles incoming contacts.</p>
    pub fn set_inbound_calls_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_inbound_calls_enabled(input);
        self
    }
    /// <p>Your contact center allows outbound calls.</p>
    pub fn outbound_calls_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.outbound_calls_enabled(input);
        self
    }
    /// <p>Your contact center allows outbound calls.</p>
    pub fn set_outbound_calls_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_outbound_calls_enabled(input);
        self
    }
}
