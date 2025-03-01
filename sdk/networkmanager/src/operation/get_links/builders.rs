// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_links::_get_links_output::GetLinksOutputBuilder;

pub use crate::operation::get_links::_get_links_input::GetLinksInputBuilder;

/// Fluent builder constructing a request to `GetLinks`.
///
/// <p>Gets information about one or more links in a specified global network.</p>
/// <p>If you specify the site ID, you cannot specify the type or provider in the same request. You can specify the type and provider in the same request.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetLinksFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_links::builders::GetLinksInputBuilder,
}
impl GetLinksFluentBuilder {
    /// Creates a new `GetLinks`.
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
            crate::operation::get_links::GetLinks,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_links::GetLinksError>,
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
        crate::operation::get_links::GetLinksOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::get_links::GetLinksError>,
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
        crate::operation::get_links::GetLinksOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::get_links::GetLinksError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::get_links::GetLinks,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_links::GetLinksError>,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::get_links::paginator::GetLinksPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::get_links::paginator::GetLinksPaginator {
        crate::operation::get_links::paginator::GetLinksPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of the global network.</p>
    pub fn global_network_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.global_network_id(input.into());
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn set_global_network_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_global_network_id(input);
        self
    }
    /// Appends an item to `LinkIds`.
    ///
    /// To override the contents of this collection use [`set_link_ids`](Self::set_link_ids).
    ///
    /// <p>One or more link IDs. The maximum is 10.</p>
    pub fn link_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.link_ids(input.into());
        self
    }
    /// <p>One or more link IDs. The maximum is 10.</p>
    pub fn set_link_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_link_ids(input);
        self
    }
    /// <p>The ID of the site.</p>
    pub fn site_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.site_id(input.into());
        self
    }
    /// <p>The ID of the site.</p>
    pub fn set_site_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_site_id(input);
        self
    }
    /// <p>The link type.</p>
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.r#type(input.into());
        self
    }
    /// <p>The link type.</p>
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The link provider.</p>
    pub fn provider(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.provider(input.into());
        self
    }
    /// <p>The link provider.</p>
    pub fn set_provider(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_provider(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
