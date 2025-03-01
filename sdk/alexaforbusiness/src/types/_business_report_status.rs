// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `BusinessReportStatus`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let businessreportstatus = unimplemented!();
/// match businessreportstatus {
///     BusinessReportStatus::Failed => { /* ... */ },
///     BusinessReportStatus::Running => { /* ... */ },
///     BusinessReportStatus::Succeeded => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `businessreportstatus` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `BusinessReportStatus::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `BusinessReportStatus::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `BusinessReportStatus::NewFeature` is defined.
/// Specifically, when `businessreportstatus` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `BusinessReportStatus::NewFeature` also yielding `"NewFeature"`.
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
pub enum BusinessReportStatus {
    #[allow(missing_docs)] // documentation missing in model
    Failed,
    #[allow(missing_docs)] // documentation missing in model
    Running,
    #[allow(missing_docs)] // documentation missing in model
    Succeeded,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl ::std::convert::From<&str> for BusinessReportStatus {
    fn from(s: &str) -> Self {
        match s {
            "FAILED" => BusinessReportStatus::Failed,
            "RUNNING" => BusinessReportStatus::Running,
            "SUCCEEDED" => BusinessReportStatus::Succeeded,
            other => BusinessReportStatus::Unknown(crate::primitives::UnknownVariantValue(
                other.to_owned(),
            )),
        }
    }
}
impl ::std::str::FromStr for BusinessReportStatus {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(BusinessReportStatus::from(s))
    }
}
impl BusinessReportStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            BusinessReportStatus::Failed => "FAILED",
            BusinessReportStatus::Running => "RUNNING",
            BusinessReportStatus::Succeeded => "SUCCEEDED",
            BusinessReportStatus::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["FAILED", "RUNNING", "SUCCEEDED"]
    }
}
impl ::std::convert::AsRef<str> for BusinessReportStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
