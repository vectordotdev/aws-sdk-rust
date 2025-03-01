// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_ip_set::_create_ip_set_output::CreateIpSetOutputBuilder;

pub use crate::operation::create_ip_set::_create_ip_set_input::CreateIpSetInputBuilder;

/// Fluent builder constructing a request to `CreateIPSet`.
///
/// <p>Creates a new IPSet, which is called a trusted IP list in the console user interface. An IPSet is a list of IP addresses that are trusted for secure communication with Amazon Web Services infrastructure and applications. GuardDuty doesn't generate findings for IP addresses that are included in IPSets. Only users from the administrator account can use this operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateIPSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_ip_set::builders::CreateIpSetInputBuilder,
}
impl CreateIPSetFluentBuilder {
    /// Creates a new `CreateIPSet`.
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
            crate::operation::create_ip_set::CreateIPSet,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_ip_set::CreateIPSetError>,
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
        crate::operation::create_ip_set::CreateIpSetOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_ip_set::CreateIPSetError>,
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
        crate::operation::create_ip_set::CreateIpSetOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_ip_set::CreateIPSetError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_ip_set::CreateIPSet,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_ip_set::CreateIPSetError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique ID of the detector of the GuardDuty account that you want to create an IPSet for.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The unique ID of the detector of the GuardDuty account that you want to create an IPSet for.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// <p>The user-friendly name to identify the IPSet.</p>
    /// <p> Allowed characters are alphanumeric, whitespace, dash (-), and underscores (_).</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The user-friendly name to identify the IPSet.</p>
    /// <p> Allowed characters are alphanumeric, whitespace, dash (-), and underscores (_).</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The format of the file that contains the IPSet.</p>
    pub fn format(mut self, input: crate::types::IpSetFormat) -> Self {
        self.inner = self.inner.format(input);
        self
    }
    /// <p>The format of the file that contains the IPSet.</p>
    pub fn set_format(mut self, input: ::std::option::Option<crate::types::IpSetFormat>) -> Self {
        self.inner = self.inner.set_format(input);
        self
    }
    /// <p>The URI of the file that contains the IPSet. </p>
    pub fn location(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.location(input.into());
        self
    }
    /// <p>The URI of the file that contains the IPSet. </p>
    pub fn set_location(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_location(input);
        self
    }
    /// <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded IPSet.</p>
    pub fn activate(mut self, input: bool) -> Self {
        self.inner = self.inner.activate(input);
        self
    }
    /// <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded IPSet.</p>
    pub fn set_activate(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_activate(input);
        self
    }
    /// <p>The idempotency token for the create request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The idempotency token for the create request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to be added to a new IP set resource.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags to be added to a new IP set resource.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
