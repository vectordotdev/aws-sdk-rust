// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::purchase_offering::_purchase_offering_output::PurchaseOfferingOutputBuilder;

pub use crate::operation::purchase_offering::_purchase_offering_input::PurchaseOfferingInputBuilder;

/// Fluent builder constructing a request to `PurchaseOffering`.
///
/// Submits a request to purchase an offering. If you already have an active reservation, you can't purchase another offering.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PurchaseOfferingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::purchase_offering::builders::PurchaseOfferingInputBuilder,
}
impl PurchaseOfferingFluentBuilder {
    /// Creates a new `PurchaseOffering`.
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
            crate::operation::purchase_offering::PurchaseOffering,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::purchase_offering::PurchaseOfferingError,
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
        crate::operation::purchase_offering::PurchaseOfferingOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::purchase_offering::PurchaseOfferingError,
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
        crate::operation::purchase_offering::PurchaseOfferingOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::purchase_offering::PurchaseOfferingError,
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
            crate::operation::purchase_offering::PurchaseOffering,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::purchase_offering::PurchaseOfferingError,
        >,
    > {
        self.customize_middleware().await
    }
    /// The Amazon Resource Name (ARN) of the offering.
    pub fn offering_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.offering_arn(input.into());
        self
    }
    /// The Amazon Resource Name (ARN) of the offering.
    pub fn set_offering_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_offering_arn(input);
        self
    }
    /// The name that you want to use for the reservation.
    pub fn reservation_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.reservation_name(input.into());
        self
    }
    /// The name that you want to use for the reservation.
    pub fn set_reservation_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_reservation_name(input);
        self
    }
    /// The date and time that you want the reservation to begin, in Coordinated Universal Time (UTC). You can specify any date and time between 12:00am on the first day of the current month to the current time on today's date, inclusive. Specify the start in a 24-hour notation. Use the following format: YYYY-MM-DDTHH:mm:SSZ, where T and Z are literal characters. For example, to specify 11:30pm on March 5, 2020, enter 2020-03-05T23:30:00Z.
    pub fn start(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.start(input.into());
        self
    }
    /// The date and time that you want the reservation to begin, in Coordinated Universal Time (UTC). You can specify any date and time between 12:00am on the first day of the current month to the current time on today's date, inclusive. Specify the start in a 24-hour notation. Use the following format: YYYY-MM-DDTHH:mm:SSZ, where T and Z are literal characters. For example, to specify 11:30pm on March 5, 2020, enter 2020-03-05T23:30:00Z.
    pub fn set_start(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_start(input);
        self
    }
}
