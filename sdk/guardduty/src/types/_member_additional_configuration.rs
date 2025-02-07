// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the additional configuration for the member account.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MemberAdditionalConfiguration {
    /// <p>Name of the additional configuration.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<crate::types::OrgFeatureAdditionalConfiguration>,
    /// <p>Status of the additional configuration.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::FeatureStatus>,
}
impl MemberAdditionalConfiguration {
    /// <p>Name of the additional configuration.</p>
    pub fn name(&self) -> ::std::option::Option<&crate::types::OrgFeatureAdditionalConfiguration> {
        self.name.as_ref()
    }
    /// <p>Status of the additional configuration.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::FeatureStatus> {
        self.status.as_ref()
    }
}
impl MemberAdditionalConfiguration {
    /// Creates a new builder-style object to manufacture [`MemberAdditionalConfiguration`](crate::types::MemberAdditionalConfiguration).
    pub fn builder() -> crate::types::builders::MemberAdditionalConfigurationBuilder {
        crate::types::builders::MemberAdditionalConfigurationBuilder::default()
    }
}

/// A builder for [`MemberAdditionalConfiguration`](crate::types::MemberAdditionalConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MemberAdditionalConfigurationBuilder {
    pub(crate) name: ::std::option::Option<crate::types::OrgFeatureAdditionalConfiguration>,
    pub(crate) status: ::std::option::Option<crate::types::FeatureStatus>,
}
impl MemberAdditionalConfigurationBuilder {
    /// <p>Name of the additional configuration.</p>
    pub fn name(mut self, input: crate::types::OrgFeatureAdditionalConfiguration) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>Name of the additional configuration.</p>
    pub fn set_name(
        mut self,
        input: ::std::option::Option<crate::types::OrgFeatureAdditionalConfiguration>,
    ) -> Self {
        self.name = input;
        self
    }
    /// <p>Status of the additional configuration.</p>
    pub fn status(mut self, input: crate::types::FeatureStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Status of the additional configuration.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::FeatureStatus>) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`MemberAdditionalConfiguration`](crate::types::MemberAdditionalConfiguration).
    pub fn build(self) -> crate::types::MemberAdditionalConfiguration {
        crate::types::MemberAdditionalConfiguration {
            name: self.name,
            status: self.status,
        }
    }
}
