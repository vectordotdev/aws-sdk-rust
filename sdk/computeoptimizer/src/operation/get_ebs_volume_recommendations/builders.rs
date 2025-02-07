// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_ebs_volume_recommendations::_get_ebs_volume_recommendations_output::GetEbsVolumeRecommendationsOutputBuilder;

pub use crate::operation::get_ebs_volume_recommendations::_get_ebs_volume_recommendations_input::GetEbsVolumeRecommendationsInputBuilder;

/// Fluent builder constructing a request to `GetEBSVolumeRecommendations`.
///
/// <p>Returns Amazon Elastic Block Store (Amazon EBS) volume recommendations.</p>
/// <p>Compute Optimizer generates recommendations for Amazon EBS volumes that meet a specific set of requirements. For more information, see the <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/requirements.html">Supported resources and requirements</a> in the <i>Compute Optimizer User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetEBSVolumeRecommendationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::get_ebs_volume_recommendations::builders::GetEbsVolumeRecommendationsInputBuilder,
}
impl GetEBSVolumeRecommendationsFluentBuilder {
    /// Creates a new `GetEBSVolumeRecommendations`.
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
            crate::operation::get_ebs_volume_recommendations::GetEBSVolumeRecommendations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_ebs_volume_recommendations::GetEBSVolumeRecommendationsError,
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
        crate::operation::get_ebs_volume_recommendations::GetEbsVolumeRecommendationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_ebs_volume_recommendations::GetEBSVolumeRecommendationsError,
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
        crate::operation::get_ebs_volume_recommendations::GetEbsVolumeRecommendationsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_ebs_volume_recommendations::GetEBSVolumeRecommendationsError,
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
            crate::operation::get_ebs_volume_recommendations::GetEBSVolumeRecommendations,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_ebs_volume_recommendations::GetEBSVolumeRecommendationsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `volumeArns`.
    ///
    /// To override the contents of this collection use [`set_volume_arns`](Self::set_volume_arns).
    ///
    /// <p>The Amazon Resource Name (ARN) of the volumes for which to return recommendations.</p>
    pub fn volume_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.volume_arns(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the volumes for which to return recommendations.</p>
    pub fn set_volume_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_volume_arns(input);
        self
    }
    /// <p>The token to advance to the next page of volume recommendations.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to advance to the next page of volume recommendations.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of volume recommendations to return with a single request.</p>
    /// <p>To retrieve the remaining results, make another request with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of volume recommendations to return with a single request.</p>
    /// <p>To retrieve the remaining results, make another request with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>An array of objects to specify a filter that returns a more specific list of volume recommendations.</p>
    pub fn filters(mut self, input: crate::types::EbsFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>An array of objects to specify a filter that returns a more specific list of volume recommendations.</p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EbsFilter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// Appends an item to `accountIds`.
    ///
    /// To override the contents of this collection use [`set_account_ids`](Self::set_account_ids).
    ///
    /// <p>The ID of the Amazon Web Services account for which to return volume recommendations.</p>
    /// <p>If your account is the management account of an organization, use this parameter to specify the member account for which you want to return volume recommendations.</p>
    /// <p>Only one account ID can be specified per request.</p>
    pub fn account_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_ids(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account for which to return volume recommendations.</p>
    /// <p>If your account is the management account of an organization, use this parameter to specify the member account for which you want to return volume recommendations.</p>
    /// <p>Only one account ID can be specified per request.</p>
    pub fn set_account_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_account_ids(input);
        self
    }
}
