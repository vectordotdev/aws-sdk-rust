// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListAssociationsForLicenseConfigurationOutput {
    /// <p>Information about the associations for the license configuration.</p>
    #[doc(hidden)]
    pub license_configuration_associations:
        ::std::option::Option<::std::vec::Vec<crate::types::LicenseConfigurationAssociation>>,
    /// <p>Token for the next set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListAssociationsForLicenseConfigurationOutput {
    /// <p>Information about the associations for the license configuration.</p>
    pub fn license_configuration_associations(
        &self,
    ) -> ::std::option::Option<&[crate::types::LicenseConfigurationAssociation]> {
        self.license_configuration_associations.as_deref()
    }
    /// <p>Token for the next set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListAssociationsForLicenseConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListAssociationsForLicenseConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`ListAssociationsForLicenseConfigurationOutput`](crate::operation::list_associations_for_license_configuration::ListAssociationsForLicenseConfigurationOutput).
    pub fn builder() -> crate::operation::list_associations_for_license_configuration::builders::ListAssociationsForLicenseConfigurationOutputBuilder{
        crate::operation::list_associations_for_license_configuration::builders::ListAssociationsForLicenseConfigurationOutputBuilder::default()
    }
}

/// A builder for [`ListAssociationsForLicenseConfigurationOutput`](crate::operation::list_associations_for_license_configuration::ListAssociationsForLicenseConfigurationOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListAssociationsForLicenseConfigurationOutputBuilder {
    pub(crate) license_configuration_associations:
        ::std::option::Option<::std::vec::Vec<crate::types::LicenseConfigurationAssociation>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListAssociationsForLicenseConfigurationOutputBuilder {
    /// Appends an item to `license_configuration_associations`.
    ///
    /// To override the contents of this collection use [`set_license_configuration_associations`](Self::set_license_configuration_associations).
    ///
    /// <p>Information about the associations for the license configuration.</p>
    pub fn license_configuration_associations(
        mut self,
        input: crate::types::LicenseConfigurationAssociation,
    ) -> Self {
        let mut v = self.license_configuration_associations.unwrap_or_default();
        v.push(input);
        self.license_configuration_associations = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the associations for the license configuration.</p>
    pub fn set_license_configuration_associations(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::LicenseConfigurationAssociation>,
        >,
    ) -> Self {
        self.license_configuration_associations = input;
        self
    }
    /// <p>Token for the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Token for the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`ListAssociationsForLicenseConfigurationOutput`](crate::operation::list_associations_for_license_configuration::ListAssociationsForLicenseConfigurationOutput).
    pub fn build(self) -> crate::operation::list_associations_for_license_configuration::ListAssociationsForLicenseConfigurationOutput{
        crate::operation::list_associations_for_license_configuration::ListAssociationsForLicenseConfigurationOutput {
            license_configuration_associations: self.license_configuration_associations
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
