// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details of the Amazon Inspector delegated administrator for your organization.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DelegatedAdmin {
    /// <p>The Amazon Web Services account ID of the Amazon Inspector delegated administrator for your organization.</p>
    #[doc(hidden)]
    pub account_id: ::std::option::Option<::std::string::String>,
    /// <p>The status of the Amazon Inspector delegated administrator.</p>
    #[doc(hidden)]
    pub relationship_status: ::std::option::Option<crate::types::RelationshipStatus>,
}
impl DelegatedAdmin {
    /// <p>The Amazon Web Services account ID of the Amazon Inspector delegated administrator for your organization.</p>
    pub fn account_id(&self) -> ::std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The status of the Amazon Inspector delegated administrator.</p>
    pub fn relationship_status(&self) -> ::std::option::Option<&crate::types::RelationshipStatus> {
        self.relationship_status.as_ref()
    }
}
impl DelegatedAdmin {
    /// Creates a new builder-style object to manufacture [`DelegatedAdmin`](crate::types::DelegatedAdmin).
    pub fn builder() -> crate::types::builders::DelegatedAdminBuilder {
        crate::types::builders::DelegatedAdminBuilder::default()
    }
}

/// A builder for [`DelegatedAdmin`](crate::types::DelegatedAdmin).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DelegatedAdminBuilder {
    pub(crate) account_id: ::std::option::Option<::std::string::String>,
    pub(crate) relationship_status: ::std::option::Option<crate::types::RelationshipStatus>,
}
impl DelegatedAdminBuilder {
    /// <p>The Amazon Web Services account ID of the Amazon Inspector delegated administrator for your organization.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID of the Amazon Inspector delegated administrator for your organization.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The status of the Amazon Inspector delegated administrator.</p>
    pub fn relationship_status(mut self, input: crate::types::RelationshipStatus) -> Self {
        self.relationship_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the Amazon Inspector delegated administrator.</p>
    pub fn set_relationship_status(
        mut self,
        input: ::std::option::Option<crate::types::RelationshipStatus>,
    ) -> Self {
        self.relationship_status = input;
        self
    }
    /// Consumes the builder and constructs a [`DelegatedAdmin`](crate::types::DelegatedAdmin).
    pub fn build(self) -> crate::types::DelegatedAdmin {
        crate::types::DelegatedAdmin {
            account_id: self.account_id,
            relationship_status: self.relationship_status,
        }
    }
}
