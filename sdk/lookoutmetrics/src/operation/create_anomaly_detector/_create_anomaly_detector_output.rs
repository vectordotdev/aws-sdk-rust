// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateAnomalyDetectorOutput {
    /// <p>The ARN of the detector.</p>
    #[doc(hidden)]
    pub anomaly_detector_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateAnomalyDetectorOutput {
    /// <p>The ARN of the detector.</p>
    pub fn anomaly_detector_arn(&self) -> ::std::option::Option<&str> {
        self.anomaly_detector_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateAnomalyDetectorOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateAnomalyDetectorOutput {
    /// Creates a new builder-style object to manufacture [`CreateAnomalyDetectorOutput`](crate::operation::create_anomaly_detector::CreateAnomalyDetectorOutput).
    pub fn builder(
    ) -> crate::operation::create_anomaly_detector::builders::CreateAnomalyDetectorOutputBuilder
    {
        crate::operation::create_anomaly_detector::builders::CreateAnomalyDetectorOutputBuilder::default()
    }
}

/// A builder for [`CreateAnomalyDetectorOutput`](crate::operation::create_anomaly_detector::CreateAnomalyDetectorOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateAnomalyDetectorOutputBuilder {
    pub(crate) anomaly_detector_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateAnomalyDetectorOutputBuilder {
    /// <p>The ARN of the detector.</p>
    pub fn anomaly_detector_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.anomaly_detector_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the detector.</p>
    pub fn set_anomaly_detector_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.anomaly_detector_arn = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateAnomalyDetectorOutput`](crate::operation::create_anomaly_detector::CreateAnomalyDetectorOutput).
    pub fn build(self) -> crate::operation::create_anomaly_detector::CreateAnomalyDetectorOutput {
        crate::operation::create_anomaly_detector::CreateAnomalyDetectorOutput {
            anomaly_detector_arn: self.anomaly_detector_arn,
            _request_id: self._request_id,
        }
    }
}
