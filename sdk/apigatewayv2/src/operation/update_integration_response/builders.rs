// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_integration_response::_update_integration_response_output::UpdateIntegrationResponseOutputBuilder;

pub use crate::operation::update_integration_response::_update_integration_response_input::UpdateIntegrationResponseInputBuilder;

/// Fluent builder constructing a request to `UpdateIntegrationResponse`.
///
/// <p>Updates an IntegrationResponses.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateIntegrationResponseFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::update_integration_response::builders::UpdateIntegrationResponseInputBuilder,
}
impl UpdateIntegrationResponseFluentBuilder {
    /// Creates a new `UpdateIntegrationResponse`.
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
            crate::operation::update_integration_response::UpdateIntegrationResponse,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_integration_response::UpdateIntegrationResponseError,
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
        crate::operation::update_integration_response::UpdateIntegrationResponseOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_integration_response::UpdateIntegrationResponseError,
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
        crate::operation::update_integration_response::UpdateIntegrationResponseOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_integration_response::UpdateIntegrationResponseError,
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
            crate::operation::update_integration_response::UpdateIntegrationResponse,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_integration_response::UpdateIntegrationResponseError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The API identifier.</p>
    pub fn api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.api_id(input.into());
        self
    }
    /// <p>The API identifier.</p>
    pub fn set_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_api_id(input);
        self
    }
    /// <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p>
    /// <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p>
    /// <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p>
    /// <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    pub fn content_handling_strategy(
        mut self,
        input: crate::types::ContentHandlingStrategy,
    ) -> Self {
        self.inner = self.inner.content_handling_strategy(input);
        self
    }
    /// <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p>
    /// <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p>
    /// <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p>
    /// <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    pub fn set_content_handling_strategy(
        mut self,
        input: ::std::option::Option<crate::types::ContentHandlingStrategy>,
    ) -> Self {
        self.inner = self.inner.set_content_handling_strategy(input);
        self
    }
    /// <p>The integration ID.</p>
    pub fn integration_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.integration_id(input.into());
        self
    }
    /// <p>The integration ID.</p>
    pub fn set_integration_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_integration_id(input);
        self
    }
    /// <p>The integration response ID.</p>
    pub fn integration_response_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.integration_response_id(input.into());
        self
    }
    /// <p>The integration response ID.</p>
    pub fn set_integration_response_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_integration_response_id(input);
        self
    }
    /// <p>The integration response key.</p>
    pub fn integration_response_key(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.integration_response_key(input.into());
        self
    }
    /// <p>The integration response key.</p>
    pub fn set_integration_response_key(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_integration_response_key(input);
        self
    }
    /// Adds a key-value pair to `ResponseParameters`.
    ///
    /// To override the contents of this collection use [`set_response_parameters`](Self::set_response_parameters).
    ///
    /// <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.<replaceable>
    /// {name}
    /// </replaceable> , where name is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.<replaceable>
    /// {name}
    /// </replaceable> or integration.response.body.<replaceable>
    /// {JSON-expression}
    /// </replaceable> , where <replaceable>
    /// {name}
    /// </replaceable> is a valid and unique response header name and <replaceable>
    /// {JSON-expression}
    /// </replaceable> is a valid JSON expression without the $ prefix.</p>
    pub fn response_parameters(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.response_parameters(k.into(), v.into());
        self
    }
    /// <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.<replaceable>
    /// {name}
    /// </replaceable> , where name is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.<replaceable>
    /// {name}
    /// </replaceable> or integration.response.body.<replaceable>
    /// {JSON-expression}
    /// </replaceable> , where <replaceable>
    /// {name}
    /// </replaceable> is a valid and unique response header name and <replaceable>
    /// {JSON-expression}
    /// </replaceable> is a valid JSON expression without the $ prefix.</p>
    pub fn set_response_parameters(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_response_parameters(input);
        self
    }
    /// Adds a key-value pair to `ResponseTemplates`.
    ///
    /// To override the contents of this collection use [`set_response_templates`](Self::set_response_templates).
    ///
    /// <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    pub fn response_templates(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.response_templates(k.into(), v.into());
        self
    }
    /// <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    pub fn set_response_templates(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_response_templates(input);
        self
    }
    /// <p>The template selection expression for the integration response. Supported only for WebSocket APIs.</p>
    pub fn template_selection_expression(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.template_selection_expression(input.into());
        self
    }
    /// <p>The template selection expression for the integration response. Supported only for WebSocket APIs.</p>
    pub fn set_template_selection_expression(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_template_selection_expression(input);
        self
    }
}
