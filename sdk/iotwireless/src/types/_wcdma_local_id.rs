// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>WCDMA local identification (local ID) information.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct WcdmaLocalId {
    /// <p>WCDMA UTRA Absolute RF Channel Number downlink.</p>
    #[doc(hidden)]
    pub uarfcndl: ::std::option::Option<i32>,
    /// <p>Primary Scrambling Code.</p>
    #[doc(hidden)]
    pub psc: ::std::option::Option<i32>,
}
impl WcdmaLocalId {
    /// <p>WCDMA UTRA Absolute RF Channel Number downlink.</p>
    pub fn uarfcndl(&self) -> ::std::option::Option<i32> {
        self.uarfcndl
    }
    /// <p>Primary Scrambling Code.</p>
    pub fn psc(&self) -> ::std::option::Option<i32> {
        self.psc
    }
}
impl WcdmaLocalId {
    /// Creates a new builder-style object to manufacture [`WcdmaLocalId`](crate::types::WcdmaLocalId).
    pub fn builder() -> crate::types::builders::WcdmaLocalIdBuilder {
        crate::types::builders::WcdmaLocalIdBuilder::default()
    }
}

/// A builder for [`WcdmaLocalId`](crate::types::WcdmaLocalId).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct WcdmaLocalIdBuilder {
    pub(crate) uarfcndl: ::std::option::Option<i32>,
    pub(crate) psc: ::std::option::Option<i32>,
}
impl WcdmaLocalIdBuilder {
    /// <p>WCDMA UTRA Absolute RF Channel Number downlink.</p>
    pub fn uarfcndl(mut self, input: i32) -> Self {
        self.uarfcndl = ::std::option::Option::Some(input);
        self
    }
    /// <p>WCDMA UTRA Absolute RF Channel Number downlink.</p>
    pub fn set_uarfcndl(mut self, input: ::std::option::Option<i32>) -> Self {
        self.uarfcndl = input;
        self
    }
    /// <p>Primary Scrambling Code.</p>
    pub fn psc(mut self, input: i32) -> Self {
        self.psc = ::std::option::Option::Some(input);
        self
    }
    /// <p>Primary Scrambling Code.</p>
    pub fn set_psc(mut self, input: ::std::option::Option<i32>) -> Self {
        self.psc = input;
        self
    }
    /// Consumes the builder and constructs a [`WcdmaLocalId`](crate::types::WcdmaLocalId).
    pub fn build(self) -> crate::types::WcdmaLocalId {
        crate::types::WcdmaLocalId {
            uarfcndl: self.uarfcndl,
            psc: self.psc,
        }
    }
}
