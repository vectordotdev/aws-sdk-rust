// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_stored_queries::_list_stored_queries_output::ListStoredQueriesOutputBuilder;

pub use crate::operation::list_stored_queries::_list_stored_queries_input::ListStoredQueriesInputBuilder;

/// Fluent builder constructing a request to `ListStoredQueries`.
///
/// <p>Lists the stored queries for a single Amazon Web Services account and a single Amazon Web Services Region. The default is 100. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListStoredQueriesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_stored_queries::builders::ListStoredQueriesInputBuilder,
}
impl ListStoredQueriesFluentBuilder {
    /// Creates a new `ListStoredQueries`.
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
            crate::operation::list_stored_queries::ListStoredQueries,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_stored_queries::ListStoredQueriesError,
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
        crate::operation::list_stored_queries::ListStoredQueriesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_stored_queries::ListStoredQueriesError,
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
        crate::operation::list_stored_queries::ListStoredQueriesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_stored_queries::ListStoredQueriesError,
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
            crate::operation::list_stored_queries::ListStoredQueries,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_stored_queries::ListStoredQueriesError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_stored_queries::paginator::ListStoredQueriesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_stored_queries::paginator::ListStoredQueriesPaginator {
        crate::operation::list_stored_queries::paginator::ListStoredQueriesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The nextToken string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The nextToken string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to be returned with a single call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to be returned with a single call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
