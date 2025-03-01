// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_aggregation_authorization::_put_aggregation_authorization_output::PutAggregationAuthorizationOutputBuilder;

pub use crate::operation::put_aggregation_authorization::_put_aggregation_authorization_input::PutAggregationAuthorizationInputBuilder;

/// Fluent builder constructing a request to `PutAggregationAuthorization`.
///
/// <p>Authorizes the aggregator account and region to collect data from the source account and region. </p> <note>
/// <p> <code>PutAggregationAuthorization</code> is an idempotent API. Subsequent requests won’t create a duplicate resource if one was already created. If a following request has different <code>tags</code> values, Config will ignore these differences and treat it as an idempotent request of the previous. In this case, <code>tags</code> will not be updated, even if they are different.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutAggregationAuthorizationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::put_aggregation_authorization::builders::PutAggregationAuthorizationInputBuilder,
}
impl PutAggregationAuthorizationFluentBuilder {
    /// Creates a new `PutAggregationAuthorization`.
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
            crate::operation::put_aggregation_authorization::PutAggregationAuthorization,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_aggregation_authorization::PutAggregationAuthorizationError,
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
        crate::operation::put_aggregation_authorization::PutAggregationAuthorizationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_aggregation_authorization::PutAggregationAuthorizationError,
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
        crate::operation::put_aggregation_authorization::PutAggregationAuthorizationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_aggregation_authorization::PutAggregationAuthorizationError,
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
            crate::operation::put_aggregation_authorization::PutAggregationAuthorization,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_aggregation_authorization::PutAggregationAuthorizationError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    pub fn authorized_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.authorized_account_id(input.into());
        self
    }
    /// <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    pub fn set_authorized_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_authorized_account_id(input);
        self
    }
    /// <p>The region authorized to collect aggregated data.</p>
    pub fn authorized_aws_region(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.authorized_aws_region(input.into());
        self
    }
    /// <p>The region authorized to collect aggregated data.</p>
    pub fn set_authorized_aws_region(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_authorized_aws_region(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>An array of tag object.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>An array of tag object.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
