// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_usage_forecast::_get_usage_forecast_output::GetUsageForecastOutputBuilder;

pub use crate::operation::get_usage_forecast::_get_usage_forecast_input::GetUsageForecastInputBuilder;

/// Fluent builder constructing a request to `GetUsageForecast`.
///
/// <p>Retrieves a forecast for how much Amazon Web Services predicts that you will use over the forecast time period that you select, based on your past usage. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetUsageForecastFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_usage_forecast::builders::GetUsageForecastInputBuilder,
}
impl GetUsageForecastFluentBuilder {
    /// Creates a new `GetUsageForecast`.
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
            crate::operation::get_usage_forecast::GetUsageForecast,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_usage_forecast::GetUsageForecastError,
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
        crate::operation::get_usage_forecast::GetUsageForecastOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_usage_forecast::GetUsageForecastError,
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
        crate::operation::get_usage_forecast::GetUsageForecastOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_usage_forecast::GetUsageForecastError,
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
            crate::operation::get_usage_forecast::GetUsageForecast,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_usage_forecast::GetUsageForecastError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The start and end dates of the period that you want to retrieve usage forecast for. The start date is included in the period, but the end date isn't included in the period. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>. The start date must be equal to or later than the current date to avoid a validation error.</p>
    pub fn time_period(mut self, input: crate::types::DateInterval) -> Self {
        self.inner = self.inner.time_period(input);
        self
    }
    /// <p>The start and end dates of the period that you want to retrieve usage forecast for. The start date is included in the period, but the end date isn't included in the period. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>. The start date must be equal to or later than the current date to avoid a validation error.</p>
    pub fn set_time_period(
        mut self,
        input: ::std::option::Option<crate::types::DateInterval>,
    ) -> Self {
        self.inner = self.inner.set_time_period(input);
        self
    }
    /// <p>Which metric Cost Explorer uses to create your forecast.</p>
    /// <p>Valid values for a <code>GetUsageForecast</code> call are the following:</p>
    /// <ul>
    /// <li> <p>USAGE_QUANTITY</p> </li>
    /// <li> <p>NORMALIZED_USAGE_AMOUNT</p> </li>
    /// </ul>
    pub fn metric(mut self, input: crate::types::Metric) -> Self {
        self.inner = self.inner.metric(input);
        self
    }
    /// <p>Which metric Cost Explorer uses to create your forecast.</p>
    /// <p>Valid values for a <code>GetUsageForecast</code> call are the following:</p>
    /// <ul>
    /// <li> <p>USAGE_QUANTITY</p> </li>
    /// <li> <p>NORMALIZED_USAGE_AMOUNT</p> </li>
    /// </ul>
    pub fn set_metric(mut self, input: ::std::option::Option<crate::types::Metric>) -> Self {
        self.inner = self.inner.set_metric(input);
        self
    }
    /// <p>How granular you want the forecast to be. You can get 3 months of <code>DAILY</code> forecasts or 12 months of <code>MONTHLY</code> forecasts.</p>
    /// <p>The <code>GetUsageForecast</code> operation supports only <code>DAILY</code> and <code>MONTHLY</code> granularities.</p>
    pub fn granularity(mut self, input: crate::types::Granularity) -> Self {
        self.inner = self.inner.granularity(input);
        self
    }
    /// <p>How granular you want the forecast to be. You can get 3 months of <code>DAILY</code> forecasts or 12 months of <code>MONTHLY</code> forecasts.</p>
    /// <p>The <code>GetUsageForecast</code> operation supports only <code>DAILY</code> and <code>MONTHLY</code> granularities.</p>
    pub fn set_granularity(
        mut self,
        input: ::std::option::Option<crate::types::Granularity>,
    ) -> Self {
        self.inner = self.inner.set_granularity(input);
        self
    }
    /// <p>The filters that you want to use to filter your forecast. The <code>GetUsageForecast</code> API supports filtering by the following dimensions:</p>
    /// <ul>
    /// <li> <p> <code>AZ</code> </p> </li>
    /// <li> <p> <code>INSTANCE_TYPE</code> </p> </li>
    /// <li> <p> <code>LINKED_ACCOUNT</code> </p> </li>
    /// <li> <p> <code>LINKED_ACCOUNT_NAME</code> </p> </li>
    /// <li> <p> <code>OPERATION</code> </p> </li>
    /// <li> <p> <code>PURCHASE_TYPE</code> </p> </li>
    /// <li> <p> <code>REGION</code> </p> </li>
    /// <li> <p> <code>SERVICE</code> </p> </li>
    /// <li> <p> <code>USAGE_TYPE</code> </p> </li>
    /// <li> <p> <code>USAGE_TYPE_GROUP</code> </p> </li>
    /// <li> <p> <code>RECORD_TYPE</code> </p> </li>
    /// <li> <p> <code>OPERATING_SYSTEM</code> </p> </li>
    /// <li> <p> <code>TENANCY</code> </p> </li>
    /// <li> <p> <code>SCOPE</code> </p> </li>
    /// <li> <p> <code>PLATFORM</code> </p> </li>
    /// <li> <p> <code>SUBSCRIPTION_ID</code> </p> </li>
    /// <li> <p> <code>LEGAL_ENTITY_NAME</code> </p> </li>
    /// <li> <p> <code>DEPLOYMENT_OPTION</code> </p> </li>
    /// <li> <p> <code>DATABASE_ENGINE</code> </p> </li>
    /// <li> <p> <code>INSTANCE_TYPE_FAMILY</code> </p> </li>
    /// <li> <p> <code>BILLING_ENTITY</code> </p> </li>
    /// <li> <p> <code>RESERVATION_ID</code> </p> </li>
    /// <li> <p> <code>SAVINGS_PLAN_ARN</code> </p> </li>
    /// </ul>
    pub fn filter(mut self, input: crate::types::Expression) -> Self {
        self.inner = self.inner.filter(input);
        self
    }
    /// <p>The filters that you want to use to filter your forecast. The <code>GetUsageForecast</code> API supports filtering by the following dimensions:</p>
    /// <ul>
    /// <li> <p> <code>AZ</code> </p> </li>
    /// <li> <p> <code>INSTANCE_TYPE</code> </p> </li>
    /// <li> <p> <code>LINKED_ACCOUNT</code> </p> </li>
    /// <li> <p> <code>LINKED_ACCOUNT_NAME</code> </p> </li>
    /// <li> <p> <code>OPERATION</code> </p> </li>
    /// <li> <p> <code>PURCHASE_TYPE</code> </p> </li>
    /// <li> <p> <code>REGION</code> </p> </li>
    /// <li> <p> <code>SERVICE</code> </p> </li>
    /// <li> <p> <code>USAGE_TYPE</code> </p> </li>
    /// <li> <p> <code>USAGE_TYPE_GROUP</code> </p> </li>
    /// <li> <p> <code>RECORD_TYPE</code> </p> </li>
    /// <li> <p> <code>OPERATING_SYSTEM</code> </p> </li>
    /// <li> <p> <code>TENANCY</code> </p> </li>
    /// <li> <p> <code>SCOPE</code> </p> </li>
    /// <li> <p> <code>PLATFORM</code> </p> </li>
    /// <li> <p> <code>SUBSCRIPTION_ID</code> </p> </li>
    /// <li> <p> <code>LEGAL_ENTITY_NAME</code> </p> </li>
    /// <li> <p> <code>DEPLOYMENT_OPTION</code> </p> </li>
    /// <li> <p> <code>DATABASE_ENGINE</code> </p> </li>
    /// <li> <p> <code>INSTANCE_TYPE_FAMILY</code> </p> </li>
    /// <li> <p> <code>BILLING_ENTITY</code> </p> </li>
    /// <li> <p> <code>RESERVATION_ID</code> </p> </li>
    /// <li> <p> <code>SAVINGS_PLAN_ARN</code> </p> </li>
    /// </ul>
    pub fn set_filter(mut self, input: ::std::option::Option<crate::types::Expression>) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
    /// <p>Amazon Web Services Cost Explorer always returns the mean forecast as a single point. You can request a prediction interval around the mean by specifying a confidence level. The higher the confidence level, the more confident Cost Explorer is about the actual value falling in the prediction interval. Higher confidence levels result in wider prediction intervals.</p>
    pub fn prediction_interval_level(mut self, input: i32) -> Self {
        self.inner = self.inner.prediction_interval_level(input);
        self
    }
    /// <p>Amazon Web Services Cost Explorer always returns the mean forecast as a single point. You can request a prediction interval around the mean by specifying a confidence level. The higher the confidence level, the more confident Cost Explorer is about the actual value falling in the prediction interval. Higher confidence levels result in wider prediction intervals.</p>
    pub fn set_prediction_interval_level(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_prediction_interval_level(input);
        self
    }
}
