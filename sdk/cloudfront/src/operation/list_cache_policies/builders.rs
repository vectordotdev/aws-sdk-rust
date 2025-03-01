// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_cache_policies::_list_cache_policies_output::ListCachePoliciesOutputBuilder;

pub use crate::operation::list_cache_policies::_list_cache_policies_input::ListCachePoliciesInputBuilder;

/// Fluent builder constructing a request to `ListCachePolicies`.
///
/// <p>Gets a list of cache policies.</p>
/// <p>You can optionally apply a filter to return only the managed policies created by Amazon Web Services, or only the custom policies created in your Amazon Web Services account.</p>
/// <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListCachePoliciesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_cache_policies::builders::ListCachePoliciesInputBuilder,
}
impl ListCachePoliciesFluentBuilder {
    /// Creates a new `ListCachePolicies`.
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
            crate::operation::list_cache_policies::ListCachePolicies,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_cache_policies::ListCachePoliciesError,
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
        crate::operation::list_cache_policies::ListCachePoliciesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_cache_policies::ListCachePoliciesError,
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
        crate::operation::list_cache_policies::ListCachePoliciesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_cache_policies::ListCachePoliciesError,
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
            crate::operation::list_cache_policies::ListCachePolicies,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_cache_policies::ListCachePoliciesError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A filter to return only the specified kinds of cache policies. Valid values are:</p>
    /// <ul>
    /// <li> <p> <code>managed</code> – Returns only the managed policies created by Amazon Web Services.</p> </li>
    /// <li> <p> <code>custom</code> – Returns only the custom policies created in your Amazon Web Services account.</p> </li>
    /// </ul>
    pub fn r#type(mut self, input: crate::types::CachePolicyType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>A filter to return only the specified kinds of cache policies. Valid values are:</p>
    /// <ul>
    /// <li> <p> <code>managed</code> – Returns only the managed policies created by Amazon Web Services.</p> </li>
    /// <li> <p> <code>custom</code> – Returns only the custom policies created in your Amazon Web Services account.</p> </li>
    /// </ul>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::CachePolicyType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>Use this field when paginating results to indicate where to begin in your list of cache policies. The response includes cache policies in the list that occur after the marker. To get the next page of the list, set this field's value to the value of <code>NextMarker</code> from the current page's response.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>Use this field when paginating results to indicate where to begin in your list of cache policies. The response includes cache policies in the list that occur after the marker. To get the next page of the list, set this field's value to the value of <code>NextMarker</code> from the current page's response.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>The maximum number of cache policies that you want in the response.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>The maximum number of cache policies that you want in the response.</p>
    pub fn set_max_items(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
}
