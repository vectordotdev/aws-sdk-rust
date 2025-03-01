// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_search_suggestions::_get_search_suggestions_output::GetSearchSuggestionsOutputBuilder;

pub use crate::operation::get_search_suggestions::_get_search_suggestions_input::GetSearchSuggestionsInputBuilder;

/// Fluent builder constructing a request to `GetSearchSuggestions`.
///
/// <p>An auto-complete API for the search functionality in the SageMaker console. It returns suggestions of possible matches for the property name to use in <code>Search</code> queries. Provides suggestions for <code>HyperParameters</code>, <code>Tags</code>, and <code>Metrics</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetSearchSuggestionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_search_suggestions::builders::GetSearchSuggestionsInputBuilder,
}
impl GetSearchSuggestionsFluentBuilder {
    /// Creates a new `GetSearchSuggestions`.
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
            crate::operation::get_search_suggestions::GetSearchSuggestions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_search_suggestions::GetSearchSuggestionsError,
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
        crate::operation::get_search_suggestions::GetSearchSuggestionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_search_suggestions::GetSearchSuggestionsError,
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
        crate::operation::get_search_suggestions::GetSearchSuggestionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_search_suggestions::GetSearchSuggestionsError,
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
            crate::operation::get_search_suggestions::GetSearchSuggestions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_search_suggestions::GetSearchSuggestionsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the SageMaker resource to search for.</p>
    pub fn resource(mut self, input: crate::types::ResourceType) -> Self {
        self.inner = self.inner.resource(input);
        self
    }
    /// <p>The name of the SageMaker resource to search for.</p>
    pub fn set_resource(
        mut self,
        input: ::std::option::Option<crate::types::ResourceType>,
    ) -> Self {
        self.inner = self.inner.set_resource(input);
        self
    }
    /// <p>Limits the property names that are included in the response.</p>
    pub fn suggestion_query(mut self, input: crate::types::SuggestionQuery) -> Self {
        self.inner = self.inner.suggestion_query(input);
        self
    }
    /// <p>Limits the property names that are included in the response.</p>
    pub fn set_suggestion_query(
        mut self,
        input: ::std::option::Option<crate::types::SuggestionQuery>,
    ) -> Self {
        self.inner = self.inner.set_suggestion_query(input);
        self
    }
}
