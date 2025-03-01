// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// A request to purchase a offering.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PurchaseOfferingInput {
    /// The Amazon Resource Name (ARN) of the offering.
    #[doc(hidden)]
    pub offering_arn: ::std::option::Option<::std::string::String>,
    /// The name that you want to use for the reservation.
    #[doc(hidden)]
    pub reservation_name: ::std::option::Option<::std::string::String>,
    /// The date and time that you want the reservation to begin, in Coordinated Universal Time (UTC). You can specify any date and time between 12:00am on the first day of the current month to the current time on today's date, inclusive. Specify the start in a 24-hour notation. Use the following format: YYYY-MM-DDTHH:mm:SSZ, where T and Z are literal characters. For example, to specify 11:30pm on March 5, 2020, enter 2020-03-05T23:30:00Z.
    #[doc(hidden)]
    pub start: ::std::option::Option<::std::string::String>,
}
impl PurchaseOfferingInput {
    /// The Amazon Resource Name (ARN) of the offering.
    pub fn offering_arn(&self) -> ::std::option::Option<&str> {
        self.offering_arn.as_deref()
    }
    /// The name that you want to use for the reservation.
    pub fn reservation_name(&self) -> ::std::option::Option<&str> {
        self.reservation_name.as_deref()
    }
    /// The date and time that you want the reservation to begin, in Coordinated Universal Time (UTC). You can specify any date and time between 12:00am on the first day of the current month to the current time on today's date, inclusive. Specify the start in a 24-hour notation. Use the following format: YYYY-MM-DDTHH:mm:SSZ, where T and Z are literal characters. For example, to specify 11:30pm on March 5, 2020, enter 2020-03-05T23:30:00Z.
    pub fn start(&self) -> ::std::option::Option<&str> {
        self.start.as_deref()
    }
}
impl PurchaseOfferingInput {
    /// Creates a new builder-style object to manufacture [`PurchaseOfferingInput`](crate::operation::purchase_offering::PurchaseOfferingInput).
    pub fn builder() -> crate::operation::purchase_offering::builders::PurchaseOfferingInputBuilder
    {
        crate::operation::purchase_offering::builders::PurchaseOfferingInputBuilder::default()
    }
}

/// A builder for [`PurchaseOfferingInput`](crate::operation::purchase_offering::PurchaseOfferingInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PurchaseOfferingInputBuilder {
    pub(crate) offering_arn: ::std::option::Option<::std::string::String>,
    pub(crate) reservation_name: ::std::option::Option<::std::string::String>,
    pub(crate) start: ::std::option::Option<::std::string::String>,
}
impl PurchaseOfferingInputBuilder {
    /// The Amazon Resource Name (ARN) of the offering.
    pub fn offering_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.offering_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// The Amazon Resource Name (ARN) of the offering.
    pub fn set_offering_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.offering_arn = input;
        self
    }
    /// The name that you want to use for the reservation.
    pub fn reservation_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.reservation_name = ::std::option::Option::Some(input.into());
        self
    }
    /// The name that you want to use for the reservation.
    pub fn set_reservation_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.reservation_name = input;
        self
    }
    /// The date and time that you want the reservation to begin, in Coordinated Universal Time (UTC). You can specify any date and time between 12:00am on the first day of the current month to the current time on today's date, inclusive. Specify the start in a 24-hour notation. Use the following format: YYYY-MM-DDTHH:mm:SSZ, where T and Z are literal characters. For example, to specify 11:30pm on March 5, 2020, enter 2020-03-05T23:30:00Z.
    pub fn start(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.start = ::std::option::Option::Some(input.into());
        self
    }
    /// The date and time that you want the reservation to begin, in Coordinated Universal Time (UTC). You can specify any date and time between 12:00am on the first day of the current month to the current time on today's date, inclusive. Specify the start in a 24-hour notation. Use the following format: YYYY-MM-DDTHH:mm:SSZ, where T and Z are literal characters. For example, to specify 11:30pm on March 5, 2020, enter 2020-03-05T23:30:00Z.
    pub fn set_start(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.start = input;
        self
    }
    /// Consumes the builder and constructs a [`PurchaseOfferingInput`](crate::operation::purchase_offering::PurchaseOfferingInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::purchase_offering::PurchaseOfferingInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::purchase_offering::PurchaseOfferingInput {
            offering_arn: self.offering_arn,
            reservation_name: self.reservation_name,
            start: self.start,
        })
    }
}
