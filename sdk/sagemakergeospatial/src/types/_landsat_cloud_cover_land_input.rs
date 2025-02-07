// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The structure representing Land Cloud Cover property for Landsat data collection.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LandsatCloudCoverLandInput {
    /// <p>The minimum value for Land Cloud Cover property filter. This will filter items having Land Cloud Cover greater than or equal to this value.</p>
    #[doc(hidden)]
    pub lower_bound: ::std::option::Option<f32>,
    /// <p>The maximum value for Land Cloud Cover property filter. This will filter items having Land Cloud Cover less than or equal to this value.</p>
    #[doc(hidden)]
    pub upper_bound: ::std::option::Option<f32>,
}
impl LandsatCloudCoverLandInput {
    /// <p>The minimum value for Land Cloud Cover property filter. This will filter items having Land Cloud Cover greater than or equal to this value.</p>
    pub fn lower_bound(&self) -> ::std::option::Option<f32> {
        self.lower_bound
    }
    /// <p>The maximum value for Land Cloud Cover property filter. This will filter items having Land Cloud Cover less than or equal to this value.</p>
    pub fn upper_bound(&self) -> ::std::option::Option<f32> {
        self.upper_bound
    }
}
impl LandsatCloudCoverLandInput {
    /// Creates a new builder-style object to manufacture [`LandsatCloudCoverLandInput`](crate::types::LandsatCloudCoverLandInput).
    pub fn builder() -> crate::types::builders::LandsatCloudCoverLandInputBuilder {
        crate::types::builders::LandsatCloudCoverLandInputBuilder::default()
    }
}

/// A builder for [`LandsatCloudCoverLandInput`](crate::types::LandsatCloudCoverLandInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LandsatCloudCoverLandInputBuilder {
    pub(crate) lower_bound: ::std::option::Option<f32>,
    pub(crate) upper_bound: ::std::option::Option<f32>,
}
impl LandsatCloudCoverLandInputBuilder {
    /// <p>The minimum value for Land Cloud Cover property filter. This will filter items having Land Cloud Cover greater than or equal to this value.</p>
    pub fn lower_bound(mut self, input: f32) -> Self {
        self.lower_bound = ::std::option::Option::Some(input);
        self
    }
    /// <p>The minimum value for Land Cloud Cover property filter. This will filter items having Land Cloud Cover greater than or equal to this value.</p>
    pub fn set_lower_bound(mut self, input: ::std::option::Option<f32>) -> Self {
        self.lower_bound = input;
        self
    }
    /// <p>The maximum value for Land Cloud Cover property filter. This will filter items having Land Cloud Cover less than or equal to this value.</p>
    pub fn upper_bound(mut self, input: f32) -> Self {
        self.upper_bound = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum value for Land Cloud Cover property filter. This will filter items having Land Cloud Cover less than or equal to this value.</p>
    pub fn set_upper_bound(mut self, input: ::std::option::Option<f32>) -> Self {
        self.upper_bound = input;
        self
    }
    /// Consumes the builder and constructs a [`LandsatCloudCoverLandInput`](crate::types::LandsatCloudCoverLandInput).
    pub fn build(self) -> crate::types::LandsatCloudCoverLandInput {
        crate::types::LandsatCloudCoverLandInput {
            lower_bound: self.lower_bound,
            upper_bound: self.upper_bound,
        }
    }
}
