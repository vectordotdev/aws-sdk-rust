// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `XavcHdIntraCbgProfileClass`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let xavchdintracbgprofileclass = unimplemented!();
/// match xavchdintracbgprofileclass {
///     XavcHdIntraCbgProfileClass::Class100 => { /* ... */ },
///     XavcHdIntraCbgProfileClass::Class200 => { /* ... */ },
///     XavcHdIntraCbgProfileClass::Class50 => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `xavchdintracbgprofileclass` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `XavcHdIntraCbgProfileClass::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `XavcHdIntraCbgProfileClass::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `XavcHdIntraCbgProfileClass::NewFeature` is defined.
/// Specifically, when `xavchdintracbgprofileclass` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `XavcHdIntraCbgProfileClass::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
/// Specify the XAVC Intra HD (CBG) Class to set the bitrate of your output. Outputs of the same class have similar image quality over the operating points that are valid for that class.
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
pub enum XavcHdIntraCbgProfileClass {
    #[allow(missing_docs)] // documentation missing in model
    Class100,
    #[allow(missing_docs)] // documentation missing in model
    Class200,
    #[allow(missing_docs)] // documentation missing in model
    Class50,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::primitives::UnknownVariantValue),
}
impl ::std::convert::From<&str> for XavcHdIntraCbgProfileClass {
    fn from(s: &str) -> Self {
        match s {
            "CLASS_100" => XavcHdIntraCbgProfileClass::Class100,
            "CLASS_200" => XavcHdIntraCbgProfileClass::Class200,
            "CLASS_50" => XavcHdIntraCbgProfileClass::Class50,
            other => XavcHdIntraCbgProfileClass::Unknown(crate::primitives::UnknownVariantValue(
                other.to_owned(),
            )),
        }
    }
}
impl ::std::str::FromStr for XavcHdIntraCbgProfileClass {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(XavcHdIntraCbgProfileClass::from(s))
    }
}
impl XavcHdIntraCbgProfileClass {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            XavcHdIntraCbgProfileClass::Class100 => "CLASS_100",
            XavcHdIntraCbgProfileClass::Class200 => "CLASS_200",
            XavcHdIntraCbgProfileClass::Class50 => "CLASS_50",
            XavcHdIntraCbgProfileClass::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["CLASS_100", "CLASS_200", "CLASS_50"]
    }
}
impl ::std::convert::AsRef<str> for XavcHdIntraCbgProfileClass {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
