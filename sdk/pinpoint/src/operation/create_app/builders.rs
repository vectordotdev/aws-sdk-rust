// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_app::_create_app_output::CreateAppOutputBuilder;

pub use crate::operation::create_app::_create_app_input::CreateAppInputBuilder;

/// Fluent builder constructing a request to `CreateApp`.
///
/// <p>Creates an application.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAppFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_app::builders::CreateAppInputBuilder,
}
impl CreateAppFluentBuilder {
    /// Creates a new `CreateApp`.
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
            crate::operation::create_app::CreateApp,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_app::CreateAppError>,
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
        crate::operation::create_app::CreateAppOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_app::CreateAppError>,
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
        crate::operation::create_app::CreateAppOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_app::CreateAppError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_app::CreateApp,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_app::CreateAppError>,
    > {
        self.customize_middleware().await
    }
    /// <p>Specifies the display name of an application and the tags to associate with the application.</p>
    pub fn create_application_request(
        mut self,
        input: crate::types::CreateApplicationRequest,
    ) -> Self {
        self.inner = self.inner.create_application_request(input);
        self
    }
    /// <p>Specifies the display name of an application and the tags to associate with the application.</p>
    pub fn set_create_application_request(
        mut self,
        input: ::std::option::Option<crate::types::CreateApplicationRequest>,
    ) -> Self {
        self.inner = self.inner.set_create_application_request(input);
        self
    }
}
