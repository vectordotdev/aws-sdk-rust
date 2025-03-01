// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTrunkInterfaceAssociationsOutput {
    /// <p>Information about the trunk associations.</p>
    #[doc(hidden)]
    pub interface_associations:
        ::std::option::Option<::std::vec::Vec<crate::types::TrunkInterfaceAssociation>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeTrunkInterfaceAssociationsOutput {
    /// <p>Information about the trunk associations.</p>
    pub fn interface_associations(
        &self,
    ) -> ::std::option::Option<&[crate::types::TrunkInterfaceAssociation]> {
        self.interface_associations.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeTrunkInterfaceAssociationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeTrunkInterfaceAssociationsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeTrunkInterfaceAssociationsOutput`](crate::operation::describe_trunk_interface_associations::DescribeTrunkInterfaceAssociationsOutput).
    pub fn builder() -> crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsOutputBuilder{
        crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsOutputBuilder::default()
    }
}

/// A builder for [`DescribeTrunkInterfaceAssociationsOutput`](crate::operation::describe_trunk_interface_associations::DescribeTrunkInterfaceAssociationsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeTrunkInterfaceAssociationsOutputBuilder {
    pub(crate) interface_associations:
        ::std::option::Option<::std::vec::Vec<crate::types::TrunkInterfaceAssociation>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeTrunkInterfaceAssociationsOutputBuilder {
    /// Appends an item to `interface_associations`.
    ///
    /// To override the contents of this collection use [`set_interface_associations`](Self::set_interface_associations).
    ///
    /// <p>Information about the trunk associations.</p>
    pub fn interface_associations(
        mut self,
        input: crate::types::TrunkInterfaceAssociation,
    ) -> Self {
        let mut v = self.interface_associations.unwrap_or_default();
        v.push(input);
        self.interface_associations = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the trunk associations.</p>
    pub fn set_interface_associations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TrunkInterfaceAssociation>>,
    ) -> Self {
        self.interface_associations = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
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
    /// Consumes the builder and constructs a [`DescribeTrunkInterfaceAssociationsOutput`](crate::operation::describe_trunk_interface_associations::DescribeTrunkInterfaceAssociationsOutput).
    pub fn build(self) -> crate::operation::describe_trunk_interface_associations::DescribeTrunkInterfaceAssociationsOutput{
        crate::operation::describe_trunk_interface_associations::DescribeTrunkInterfaceAssociationsOutput {
            interface_associations: self.interface_associations
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
