// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_contact_flow_content::_update_contact_flow_content_output::UpdateContactFlowContentOutputBuilder;

pub use crate::operation::update_contact_flow_content::_update_contact_flow_content_input::UpdateContactFlowContentInputBuilder;

/// Fluent builder constructing a request to `UpdateContactFlowContent`.
///
/// <p>Updates the specified flow.</p>
/// <p>You can also create and update flows using the <a href="https://docs.aws.amazon.com/connect/latest/APIReference/flow-language.html">Amazon Connect Flow language</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateContactFlowContentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_contact_flow_content::builders::UpdateContactFlowContentInputBuilder,
}
impl UpdateContactFlowContentFluentBuilder {
    /// Creates a new `UpdateContactFlowContent`.
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
            crate::operation::update_contact_flow_content::UpdateContactFlowContent,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_contact_flow_content::UpdateContactFlowContentError,
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
        crate::operation::update_contact_flow_content::UpdateContactFlowContentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_contact_flow_content::UpdateContactFlowContentError,
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
        crate::operation::update_contact_flow_content::UpdateContactFlowContentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_contact_flow_content::UpdateContactFlowContentError,
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
            crate::operation::update_contact_flow_content::UpdateContactFlowContent,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_contact_flow_content::UpdateContactFlowContentError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The identifier of the Amazon Connect instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The identifier of the flow.</p>
    pub fn contact_flow_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.contact_flow_id(input.into());
        self
    }
    /// <p>The identifier of the flow.</p>
    pub fn set_contact_flow_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_contact_flow_id(input);
        self
    }
    /// <p>The JSON string that represents flow's content. For an example, see <a href="https://docs.aws.amazon.com/connect/latest/APIReference/flow-language-example.html">Example contact flow in Amazon Connect Flow language</a>. </p>
    pub fn content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.content(input.into());
        self
    }
    /// <p>The JSON string that represents flow's content. For an example, see <a href="https://docs.aws.amazon.com/connect/latest/APIReference/flow-language-example.html">Example contact flow in Amazon Connect Flow language</a>. </p>
    pub fn set_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_content(input);
        self
    }
}
