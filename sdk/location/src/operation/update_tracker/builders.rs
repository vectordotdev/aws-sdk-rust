// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_tracker::_update_tracker_output::UpdateTrackerOutputBuilder;

pub use crate::operation::update_tracker::_update_tracker_input::UpdateTrackerInputBuilder;

/// Fluent builder constructing a request to `UpdateTracker`.
///
/// <p>Updates the specified properties of a given tracker resource.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateTrackerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_tracker::builders::UpdateTrackerInputBuilder,
}
impl UpdateTrackerFluentBuilder {
    /// Creates a new `UpdateTracker`.
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
            crate::operation::update_tracker::UpdateTracker,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_tracker::UpdateTrackerError>,
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
        crate::operation::update_tracker::UpdateTrackerOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_tracker::UpdateTrackerError>,
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
        crate::operation::update_tracker::UpdateTrackerOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_tracker::UpdateTrackerError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_tracker::UpdateTracker,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_tracker::UpdateTrackerError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the tracker resource to update.</p>
    pub fn tracker_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tracker_name(input.into());
        self
    }
    /// <p>The name of the tracker resource to update.</p>
    pub fn set_tracker_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_tracker_name(input);
        self
    }
    /// <p>No longer used. If included, the only allowed value is <code>RequestBasedUsage</code>.</p>
    #[deprecated(
        note = "Deprecated. If included, the only allowed value is RequestBasedUsage.",
        since = "2022-02-01"
    )]
    pub fn pricing_plan(mut self, input: crate::types::PricingPlan) -> Self {
        self.inner = self.inner.pricing_plan(input);
        self
    }
    /// <p>No longer used. If included, the only allowed value is <code>RequestBasedUsage</code>.</p>
    #[deprecated(
        note = "Deprecated. If included, the only allowed value is RequestBasedUsage.",
        since = "2022-02-01"
    )]
    pub fn set_pricing_plan(
        mut self,
        input: ::std::option::Option<crate::types::PricingPlan>,
    ) -> Self {
        self.inner = self.inner.set_pricing_plan(input);
        self
    }
    /// <p>This parameter is no longer used.</p>
    #[deprecated(note = "Deprecated. No longer allowed.", since = "2022-02-01")]
    pub fn pricing_plan_data_source(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.pricing_plan_data_source(input.into());
        self
    }
    /// <p>This parameter is no longer used.</p>
    #[deprecated(note = "Deprecated. No longer allowed.", since = "2022-02-01")]
    pub fn set_pricing_plan_data_source(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_pricing_plan_data_source(input);
        self
    }
    /// <p>Updates the description for the tracker resource.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>Updates the description for the tracker resource.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Updates the position filtering for the tracker resource.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li> <p> <code>TimeBased</code> - Location updates are evaluated against linked geofence collections, but not every location update is stored. If your update frequency is more often than 30 seconds, only one update per 30 seconds is stored for each unique device ID. </p> </li>
    /// <li> <p> <code>DistanceBased</code> - If the device has moved less than 30 m (98.4 ft), location updates are ignored. Location updates within this distance are neither evaluated against linked geofence collections, nor stored. This helps control costs by reducing the number of geofence evaluations and historical device positions to paginate through. Distance-based filtering can also reduce the effects of GPS noise when displaying device trajectories on a map. </p> </li>
    /// <li> <p> <code>AccuracyBased</code> - If the device has moved less than the measured accuracy, location updates are ignored. For example, if two consecutive updates from a device have a horizontal accuracy of 5 m and 10 m, the second update is ignored if the device has moved less than 15 m. Ignored location updates are neither evaluated against linked geofence collections, nor stored. This helps educe the effects of GPS noise when displaying device trajectories on a map, and can help control costs by reducing the number of geofence evaluations. </p> </li>
    /// </ul>
    pub fn position_filtering(mut self, input: crate::types::PositionFiltering) -> Self {
        self.inner = self.inner.position_filtering(input);
        self
    }
    /// <p>Updates the position filtering for the tracker resource.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li> <p> <code>TimeBased</code> - Location updates are evaluated against linked geofence collections, but not every location update is stored. If your update frequency is more often than 30 seconds, only one update per 30 seconds is stored for each unique device ID. </p> </li>
    /// <li> <p> <code>DistanceBased</code> - If the device has moved less than 30 m (98.4 ft), location updates are ignored. Location updates within this distance are neither evaluated against linked geofence collections, nor stored. This helps control costs by reducing the number of geofence evaluations and historical device positions to paginate through. Distance-based filtering can also reduce the effects of GPS noise when displaying device trajectories on a map. </p> </li>
    /// <li> <p> <code>AccuracyBased</code> - If the device has moved less than the measured accuracy, location updates are ignored. For example, if two consecutive updates from a device have a horizontal accuracy of 5 m and 10 m, the second update is ignored if the device has moved less than 15 m. Ignored location updates are neither evaluated against linked geofence collections, nor stored. This helps educe the effects of GPS noise when displaying device trajectories on a map, and can help control costs by reducing the number of geofence evaluations. </p> </li>
    /// </ul>
    pub fn set_position_filtering(
        mut self,
        input: ::std::option::Option<crate::types::PositionFiltering>,
    ) -> Self {
        self.inner = self.inner.set_position_filtering(input);
        self
    }
}
