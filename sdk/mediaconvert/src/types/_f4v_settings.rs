// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Settings for F4v container
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct F4vSettings {
    /// If set to PROGRESSIVE_DOWNLOAD, the MOOV atom is relocated to the beginning of the archive as required for progressive downloading. Otherwise it is placed normally at the end.
    #[doc(hidden)]
    pub moov_placement: ::std::option::Option<crate::types::F4vMoovPlacement>,
}
impl F4vSettings {
    /// If set to PROGRESSIVE_DOWNLOAD, the MOOV atom is relocated to the beginning of the archive as required for progressive downloading. Otherwise it is placed normally at the end.
    pub fn moov_placement(&self) -> ::std::option::Option<&crate::types::F4vMoovPlacement> {
        self.moov_placement.as_ref()
    }
}
impl F4vSettings {
    /// Creates a new builder-style object to manufacture [`F4vSettings`](crate::types::F4vSettings).
    pub fn builder() -> crate::types::builders::F4vSettingsBuilder {
        crate::types::builders::F4vSettingsBuilder::default()
    }
}

/// A builder for [`F4vSettings`](crate::types::F4vSettings).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct F4vSettingsBuilder {
    pub(crate) moov_placement: ::std::option::Option<crate::types::F4vMoovPlacement>,
}
impl F4vSettingsBuilder {
    /// If set to PROGRESSIVE_DOWNLOAD, the MOOV atom is relocated to the beginning of the archive as required for progressive downloading. Otherwise it is placed normally at the end.
    pub fn moov_placement(mut self, input: crate::types::F4vMoovPlacement) -> Self {
        self.moov_placement = ::std::option::Option::Some(input);
        self
    }
    /// If set to PROGRESSIVE_DOWNLOAD, the MOOV atom is relocated to the beginning of the archive as required for progressive downloading. Otherwise it is placed normally at the end.
    pub fn set_moov_placement(
        mut self,
        input: ::std::option::Option<crate::types::F4vMoovPlacement>,
    ) -> Self {
        self.moov_placement = input;
        self
    }
    /// Consumes the builder and constructs a [`F4vSettings`](crate::types::F4vSettings).
    pub fn build(self) -> crate::types::F4vSettings {
        crate::types::F4vSettings {
            moov_placement: self.moov_placement,
        }
    }
}
