// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `ValidationExceptionReason`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let validationexceptionreason = unimplemented!();
/// match validationexceptionreason {
///     ValidationExceptionReason::CannotAddOptedOutNumber => { /* ... */ },
///     ValidationExceptionReason::CannotParse => { /* ... */ },
///     ValidationExceptionReason::CountryCodeMismatch => { /* ... */ },
///     ValidationExceptionReason::DestinationCountryBlocked => { /* ... */ },
///     ValidationExceptionReason::FieldValidationFailed => { /* ... */ },
///     ValidationExceptionReason::InvalidArn => { /* ... */ },
///     ValidationExceptionReason::InvalidFilterValues => { /* ... */ },
///     ValidationExceptionReason::InvalidIdentityForDestinationCountry => { /* ... */ },
///     ValidationExceptionReason::InvalidNextToken => { /* ... */ },
///     ValidationExceptionReason::InvalidParameter => { /* ... */ },
///     ValidationExceptionReason::MissingParameter => { /* ... */ },
///     ValidationExceptionReason::Other => { /* ... */ },
///     ValidationExceptionReason::ParametersCannotBeUsedTogether => { /* ... */ },
///     ValidationExceptionReason::PhoneNumberCannotBeOptedIn => { /* ... */ },
///     ValidationExceptionReason::PhoneNumberCannotBeReleased => { /* ... */ },
///     ValidationExceptionReason::PriceOverThreshold => { /* ... */ },
///     ValidationExceptionReason::RequestedSpendLimitHigherThanServiceLimit => { /* ... */ },
///     ValidationExceptionReason::SenderIdNotRegistered => { /* ... */ },
///     ValidationExceptionReason::SenderIdNotSupported => { /* ... */ },
///     ValidationExceptionReason::TwoWayNotEnabled => { /* ... */ },
///     ValidationExceptionReason::TwoWayNotSupportedInCountry => { /* ... */ },
///     ValidationExceptionReason::TwoWayNotSupportedInRegion => { /* ... */ },
///     ValidationExceptionReason::TwoWayTopicNotPresent => { /* ... */ },
///     ValidationExceptionReason::UnknownOperation => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `validationexceptionreason` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ValidationExceptionReason::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ValidationExceptionReason::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ValidationExceptionReason::NewFeature` is defined.
/// Specifically, when `validationexceptionreason` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ValidationExceptionReason::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    ::std::clone::Clone,
    ::std::cmp::Eq,
    ::std::cmp::Ord,
    ::std::cmp::PartialEq,
    ::std::cmp::PartialOrd,
    ::std::fmt::Debug,
    ::std::hash::Hash,
)]
pub enum ValidationExceptionReason {
    #[allow(missing_docs)] // documentation missing in model
    CannotAddOptedOutNumber,
    #[allow(missing_docs)] // documentation missing in model
    CannotParse,
    #[allow(missing_docs)] // documentation missing in model
    CountryCodeMismatch,
    #[allow(missing_docs)] // documentation missing in model
    DestinationCountryBlocked,
    #[allow(missing_docs)] // documentation missing in model
    FieldValidationFailed,
    #[allow(missing_docs)] // documentation missing in model
    InvalidArn,
    #[allow(missing_docs)] // documentation missing in model
    InvalidFilterValues,
    #[allow(missing_docs)] // documentation missing in model
    InvalidIdentityForDestinationCountry,
    #[allow(missing_docs)] // documentation missing in model
    InvalidNextToken,
    #[allow(missing_docs)] // documentation missing in model
    InvalidParameter,
    #[allow(missing_docs)] // documentation missing in model
    MissingParameter,
    #[allow(missing_docs)] // documentation missing in model
    Other,
    #[allow(missing_docs)] // documentation missing in model
    ParametersCannotBeUsedTogether,
    #[allow(missing_docs)] // documentation missing in model
    PhoneNumberCannotBeOptedIn,
    #[allow(missing_docs)] // documentation missing in model
    PhoneNumberCannotBeReleased,
    #[allow(missing_docs)] // documentation missing in model
    PriceOverThreshold,
    #[allow(missing_docs)] // documentation missing in model
    RequestedSpendLimitHigherThanServiceLimit,
    #[allow(missing_docs)] // documentation missing in model
    SenderIdNotRegistered,
    #[allow(missing_docs)] // documentation missing in model
    SenderIdNotSupported,
    #[allow(missing_docs)] // documentation missing in model
    TwoWayNotEnabled,
    #[allow(missing_docs)] // documentation missing in model
    TwoWayNotSupportedInCountry,
    #[allow(missing_docs)] // documentation missing in model
    TwoWayNotSupportedInRegion,
    #[allow(missing_docs)] // documentation missing in model
    TwoWayTopicNotPresent,
    #[allow(missing_docs)] // documentation missing in model
    UnknownOperation,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl ::std::convert::From<&str> for ValidationExceptionReason {
    fn from(s: &str) -> Self {
        match s {
            "CANNOT_ADD_OPTED_OUT_NUMBER" => ValidationExceptionReason::CannotAddOptedOutNumber,
            "CANNOT_PARSE" => ValidationExceptionReason::CannotParse,
            "COUNTRY_CODE_MISMATCH" => ValidationExceptionReason::CountryCodeMismatch,
            "DESTINATION_COUNTRY_BLOCKED" => ValidationExceptionReason::DestinationCountryBlocked,
            "FIELD_VALIDATION_FAILED" => ValidationExceptionReason::FieldValidationFailed,
            "INVALID_ARN" => ValidationExceptionReason::InvalidArn,
            "INVALID_FILTER_VALUES" => ValidationExceptionReason::InvalidFilterValues,
            "INVALID_IDENTITY_FOR_DESTINATION_COUNTRY" => {
                ValidationExceptionReason::InvalidIdentityForDestinationCountry
            }
            "INVALID_NEXT_TOKEN" => ValidationExceptionReason::InvalidNextToken,
            "INVALID_PARAMETER" => ValidationExceptionReason::InvalidParameter,
            "MISSING_PARAMETER" => ValidationExceptionReason::MissingParameter,
            "OTHER" => ValidationExceptionReason::Other,
            "PARAMETERS_CANNOT_BE_USED_TOGETHER" => {
                ValidationExceptionReason::ParametersCannotBeUsedTogether
            }
            "PHONE_NUMBER_CANNOT_BE_OPTED_IN" => {
                ValidationExceptionReason::PhoneNumberCannotBeOptedIn
            }
            "PHONE_NUMBER_CANNOT_BE_RELEASED" => {
                ValidationExceptionReason::PhoneNumberCannotBeReleased
            }
            "PRICE_OVER_THRESHOLD" => ValidationExceptionReason::PriceOverThreshold,
            "REQUESTED_SPEND_LIMIT_HIGHER_THAN_SERVICE_LIMIT" => {
                ValidationExceptionReason::RequestedSpendLimitHigherThanServiceLimit
            }
            "SENDER_ID_NOT_REGISTERED" => ValidationExceptionReason::SenderIdNotRegistered,
            "SENDER_ID_NOT_SUPPORTED" => ValidationExceptionReason::SenderIdNotSupported,
            "TWO_WAY_NOT_ENABLED" => ValidationExceptionReason::TwoWayNotEnabled,
            "TWO_WAY_NOT_SUPPORTED_IN_COUNTRY" => {
                ValidationExceptionReason::TwoWayNotSupportedInCountry
            }
            "TWO_WAY_NOT_SUPPORTED_IN_REGION" => {
                ValidationExceptionReason::TwoWayNotSupportedInRegion
            }
            "TWO_WAY_TOPIC_NOT_PRESENT" => ValidationExceptionReason::TwoWayTopicNotPresent,
            "UNKNOWN_OPERATION" => ValidationExceptionReason::UnknownOperation,
            other => ValidationExceptionReason::Unknown(crate::primitives::UnknownVariantValue(
                other.to_owned(),
            )),
        }
    }
}
impl ::std::str::FromStr for ValidationExceptionReason {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(ValidationExceptionReason::from(s))
    }
}
impl ValidationExceptionReason {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ValidationExceptionReason::CannotAddOptedOutNumber => "CANNOT_ADD_OPTED_OUT_NUMBER",
            ValidationExceptionReason::CannotParse => "CANNOT_PARSE",
            ValidationExceptionReason::CountryCodeMismatch => "COUNTRY_CODE_MISMATCH",
            ValidationExceptionReason::DestinationCountryBlocked => "DESTINATION_COUNTRY_BLOCKED",
            ValidationExceptionReason::FieldValidationFailed => "FIELD_VALIDATION_FAILED",
            ValidationExceptionReason::InvalidArn => "INVALID_ARN",
            ValidationExceptionReason::InvalidFilterValues => "INVALID_FILTER_VALUES",
            ValidationExceptionReason::InvalidIdentityForDestinationCountry => {
                "INVALID_IDENTITY_FOR_DESTINATION_COUNTRY"
            }
            ValidationExceptionReason::InvalidNextToken => "INVALID_NEXT_TOKEN",
            ValidationExceptionReason::InvalidParameter => "INVALID_PARAMETER",
            ValidationExceptionReason::MissingParameter => "MISSING_PARAMETER",
            ValidationExceptionReason::Other => "OTHER",
            ValidationExceptionReason::ParametersCannotBeUsedTogether => {
                "PARAMETERS_CANNOT_BE_USED_TOGETHER"
            }
            ValidationExceptionReason::PhoneNumberCannotBeOptedIn => {
                "PHONE_NUMBER_CANNOT_BE_OPTED_IN"
            }
            ValidationExceptionReason::PhoneNumberCannotBeReleased => {
                "PHONE_NUMBER_CANNOT_BE_RELEASED"
            }
            ValidationExceptionReason::PriceOverThreshold => "PRICE_OVER_THRESHOLD",
            ValidationExceptionReason::RequestedSpendLimitHigherThanServiceLimit => {
                "REQUESTED_SPEND_LIMIT_HIGHER_THAN_SERVICE_LIMIT"
            }
            ValidationExceptionReason::SenderIdNotRegistered => "SENDER_ID_NOT_REGISTERED",
            ValidationExceptionReason::SenderIdNotSupported => "SENDER_ID_NOT_SUPPORTED",
            ValidationExceptionReason::TwoWayNotEnabled => "TWO_WAY_NOT_ENABLED",
            ValidationExceptionReason::TwoWayNotSupportedInCountry => {
                "TWO_WAY_NOT_SUPPORTED_IN_COUNTRY"
            }
            ValidationExceptionReason::TwoWayNotSupportedInRegion => {
                "TWO_WAY_NOT_SUPPORTED_IN_REGION"
            }
            ValidationExceptionReason::TwoWayTopicNotPresent => "TWO_WAY_TOPIC_NOT_PRESENT",
            ValidationExceptionReason::UnknownOperation => "UNKNOWN_OPERATION",
            ValidationExceptionReason::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "CANNOT_ADD_OPTED_OUT_NUMBER",
            "CANNOT_PARSE",
            "COUNTRY_CODE_MISMATCH",
            "DESTINATION_COUNTRY_BLOCKED",
            "FIELD_VALIDATION_FAILED",
            "INVALID_ARN",
            "INVALID_FILTER_VALUES",
            "INVALID_IDENTITY_FOR_DESTINATION_COUNTRY",
            "INVALID_NEXT_TOKEN",
            "INVALID_PARAMETER",
            "MISSING_PARAMETER",
            "OTHER",
            "PARAMETERS_CANNOT_BE_USED_TOGETHER",
            "PHONE_NUMBER_CANNOT_BE_OPTED_IN",
            "PHONE_NUMBER_CANNOT_BE_RELEASED",
            "PRICE_OVER_THRESHOLD",
            "REQUESTED_SPEND_LIMIT_HIGHER_THAN_SERVICE_LIMIT",
            "SENDER_ID_NOT_REGISTERED",
            "SENDER_ID_NOT_SUPPORTED",
            "TWO_WAY_NOT_ENABLED",
            "TWO_WAY_NOT_SUPPORTED_IN_COUNTRY",
            "TWO_WAY_NOT_SUPPORTED_IN_REGION",
            "TWO_WAY_TOPIC_NOT_PRESENT",
            "UNKNOWN_OPERATION",
        ]
    }
}
impl ::std::convert::AsRef<str> for ValidationExceptionReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
