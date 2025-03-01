// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_configurations::_list_configurations_output::ListConfigurationsOutputBuilder;

pub use crate::operation::list_configurations::_list_configurations_input::ListConfigurationsInputBuilder;

/// Fluent builder constructing a request to `ListConfigurations`.
///
/// <p>Retrieves a list of configuration items as specified by the value passed to the required parameter <code>configurationType</code>. Optional filtering may be applied to refine search results.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListConfigurationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_configurations::builders::ListConfigurationsInputBuilder,
}
impl ListConfigurationsFluentBuilder {
    /// Creates a new `ListConfigurations`.
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
            crate::operation::list_configurations::ListConfigurations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_configurations::ListConfigurationsError,
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
        crate::operation::list_configurations::ListConfigurationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_configurations::ListConfigurationsError,
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
        crate::operation::list_configurations::ListConfigurationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_configurations::ListConfigurationsError,
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
            crate::operation::list_configurations::ListConfigurations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_configurations::ListConfigurationsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A valid configuration identified by Application Discovery Service. </p>
    pub fn configuration_type(mut self, input: crate::types::ConfigurationItemType) -> Self {
        self.inner = self.inner.configuration_type(input);
        self
    }
    /// <p>A valid configuration identified by Application Discovery Service. </p>
    pub fn set_configuration_type(
        mut self,
        input: ::std::option::Option<crate::types::ConfigurationItemType>,
    ) -> Self {
        self.inner = self.inner.set_configuration_type(input);
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>You can filter the request using various logical operators and a <i>key</i>-<i>value</i> format. For example: </p>
    /// <p> <code>{"key": "serverType", "value": "webServer"}</code> </p>
    /// <p>For a complete list of filter options and guidance about using them with this action, see <a href="https://docs.aws.amazon.com/application-discovery/latest/userguide/discovery-api-queries.html#ListConfigurations">Using the ListConfigurations Action</a> in the <i>Amazon Web Services Application Discovery Service User Guide</i>.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>You can filter the request using various logical operators and a <i>key</i>-<i>value</i> format. For example: </p>
    /// <p> <code>{"key": "serverType", "value": "webServer"}</code> </p>
    /// <p>For a complete list of filter options and guidance about using them with this action, see <a href="https://docs.aws.amazon.com/application-discovery/latest/userguide/discovery-api-queries.html#ListConfigurations">Using the ListConfigurations Action</a> in the <i>Amazon Web Services Application Discovery Service User Guide</i>.</p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The total number of items to return. The maximum value is 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The total number of items to return. The maximum value is 100.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Token to retrieve the next set of results. For example, if a previous call to ListConfigurations returned 100 items, but you set <code>ListConfigurationsRequest$maxResults</code> to 10, you received a set of 10 results along with a token. Use that token in this query to get the next set of 10.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Token to retrieve the next set of results. For example, if a previous call to ListConfigurations returned 100 items, but you set <code>ListConfigurationsRequest$maxResults</code> to 10, you received a set of 10 results along with a token. Use that token in this query to get the next set of 10.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// Appends an item to `orderBy`.
    ///
    /// To override the contents of this collection use [`set_order_by`](Self::set_order_by).
    ///
    /// <p>Certain filter criteria return output that can be sorted in ascending or descending order. For a list of output characteristics for each filter, see <a href="https://docs.aws.amazon.com/application-discovery/latest/userguide/discovery-api-queries.html#ListConfigurations">Using the ListConfigurations Action</a> in the <i>Amazon Web Services Application Discovery Service User Guide</i>.</p>
    pub fn order_by(mut self, input: crate::types::OrderByElement) -> Self {
        self.inner = self.inner.order_by(input);
        self
    }
    /// <p>Certain filter criteria return output that can be sorted in ascending or descending order. For a list of output characteristics for each filter, see <a href="https://docs.aws.amazon.com/application-discovery/latest/userguide/discovery-api-queries.html#ListConfigurations">Using the ListConfigurations Action</a> in the <i>Amazon Web Services Application Discovery Service User Guide</i>.</p>
    pub fn set_order_by(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::OrderByElement>>,
    ) -> Self {
        self.inner = self.inner.set_order_by(input);
        self
    }
}
