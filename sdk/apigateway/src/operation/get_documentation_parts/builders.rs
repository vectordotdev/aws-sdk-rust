// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_documentation_parts::_get_documentation_parts_output::GetDocumentationPartsOutputBuilder;

pub use crate::operation::get_documentation_parts::_get_documentation_parts_input::GetDocumentationPartsInputBuilder;

/// Fluent builder constructing a request to `GetDocumentationParts`.
///
/// <p>Gets documentation parts.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetDocumentationPartsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_documentation_parts::builders::GetDocumentationPartsInputBuilder,
}
impl GetDocumentationPartsFluentBuilder {
    /// Creates a new `GetDocumentationParts`.
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
            crate::operation::get_documentation_parts::GetDocumentationParts,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_documentation_parts::GetDocumentationPartsError,
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
        crate::operation::get_documentation_parts::GetDocumentationPartsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_documentation_parts::GetDocumentationPartsError,
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
        crate::operation::get_documentation_parts::GetDocumentationPartsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_documentation_parts::GetDocumentationPartsError,
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
            crate::operation::get_documentation_parts::GetDocumentationParts,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_documentation_parts::GetDocumentationPartsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn rest_api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rest_api_id(input.into());
        self
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn set_rest_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rest_api_id(input);
        self
    }
    /// <p>The type of API entities of the to-be-retrieved documentation parts. </p>
    pub fn r#type(mut self, input: crate::types::DocumentationPartType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of API entities of the to-be-retrieved documentation parts. </p>
    pub fn set_type(
        mut self,
        input: ::std::option::Option<crate::types::DocumentationPartType>,
    ) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The name of API entities of the to-be-retrieved documentation parts.</p>
    pub fn name_query(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name_query(input.into());
        self
    }
    /// <p>The name of API entities of the to-be-retrieved documentation parts.</p>
    pub fn set_name_query(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name_query(input);
        self
    }
    /// <p>The path of API entities of the to-be-retrieved documentation parts.</p>
    pub fn path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.path(input.into());
        self
    }
    /// <p>The path of API entities of the to-be-retrieved documentation parts.</p>
    pub fn set_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_path(input);
        self
    }
    /// <p>The current pagination position in the paged result set.</p>
    pub fn position(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.position(input.into());
        self
    }
    /// <p>The current pagination position in the paged result set.</p>
    pub fn set_position(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_position(input);
        self
    }
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The status of the API documentation parts to retrieve. Valid values are <code>DOCUMENTED</code> for retrieving DocumentationPart resources with content and <code>UNDOCUMENTED</code> for DocumentationPart resources without content.</p>
    pub fn location_status(mut self, input: crate::types::LocationStatusType) -> Self {
        self.inner = self.inner.location_status(input);
        self
    }
    /// <p>The status of the API documentation parts to retrieve. Valid values are <code>DOCUMENTED</code> for retrieving DocumentationPart resources with content and <code>UNDOCUMENTED</code> for DocumentationPart resources without content.</p>
    pub fn set_location_status(
        mut self,
        input: ::std::option::Option<crate::types::LocationStatusType>,
    ) -> Self {
        self.inner = self.inner.set_location_status(input);
        self
    }
}
