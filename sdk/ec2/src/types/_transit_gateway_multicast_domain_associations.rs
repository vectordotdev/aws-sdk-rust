// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the multicast domain associations.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransitGatewayMulticastDomainAssociations {
    /// <p>The ID of the transit gateway multicast domain.</p>
    #[doc(hidden)]
    pub transit_gateway_multicast_domain_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the transit gateway attachment.</p>
    #[doc(hidden)]
    pub transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the resource.</p>
    #[doc(hidden)]
    pub resource_id: ::std::option::Option<::std::string::String>,
    /// <p>The type of resource, for example a VPC attachment.</p>
    #[doc(hidden)]
    pub resource_type: ::std::option::Option<crate::types::TransitGatewayAttachmentResourceType>,
    /// <p> The ID of the Amazon Web Services account that owns the resource.</p>
    #[doc(hidden)]
    pub resource_owner_id: ::std::option::Option<::std::string::String>,
    /// <p>The subnets associated with the multicast domain.</p>
    #[doc(hidden)]
    pub subnets: ::std::option::Option<::std::vec::Vec<crate::types::SubnetAssociation>>,
}
impl TransitGatewayMulticastDomainAssociations {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_multicast_domain_id.as_deref()
    }
    /// <p>The ID of the transit gateway attachment.</p>
    pub fn transit_gateway_attachment_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_attachment_id.as_deref()
    }
    /// <p>The ID of the resource.</p>
    pub fn resource_id(&self) -> ::std::option::Option<&str> {
        self.resource_id.as_deref()
    }
    /// <p>The type of resource, for example a VPC attachment.</p>
    pub fn resource_type(
        &self,
    ) -> ::std::option::Option<&crate::types::TransitGatewayAttachmentResourceType> {
        self.resource_type.as_ref()
    }
    /// <p> The ID of the Amazon Web Services account that owns the resource.</p>
    pub fn resource_owner_id(&self) -> ::std::option::Option<&str> {
        self.resource_owner_id.as_deref()
    }
    /// <p>The subnets associated with the multicast domain.</p>
    pub fn subnets(&self) -> ::std::option::Option<&[crate::types::SubnetAssociation]> {
        self.subnets.as_deref()
    }
}
impl TransitGatewayMulticastDomainAssociations {
    /// Creates a new builder-style object to manufacture [`TransitGatewayMulticastDomainAssociations`](crate::types::TransitGatewayMulticastDomainAssociations).
    pub fn builder() -> crate::types::builders::TransitGatewayMulticastDomainAssociationsBuilder {
        crate::types::builders::TransitGatewayMulticastDomainAssociationsBuilder::default()
    }
}

/// A builder for [`TransitGatewayMulticastDomainAssociations`](crate::types::TransitGatewayMulticastDomainAssociations).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TransitGatewayMulticastDomainAssociationsBuilder {
    pub(crate) transit_gateway_multicast_domain_id: ::std::option::Option<::std::string::String>,
    pub(crate) transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    pub(crate) resource_id: ::std::option::Option<::std::string::String>,
    pub(crate) resource_type:
        ::std::option::Option<crate::types::TransitGatewayAttachmentResourceType>,
    pub(crate) resource_owner_id: ::std::option::Option<::std::string::String>,
    pub(crate) subnets: ::std::option::Option<::std::vec::Vec<crate::types::SubnetAssociation>>,
}
impl TransitGatewayMulticastDomainAssociationsBuilder {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.transit_gateway_multicast_domain_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn set_transit_gateway_multicast_domain_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.transit_gateway_multicast_domain_id = input;
        self
    }
    /// <p>The ID of the transit gateway attachment.</p>
    pub fn transit_gateway_attachment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.transit_gateway_attachment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway attachment.</p>
    pub fn set_transit_gateway_attachment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.transit_gateway_attachment_id = input;
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_id = input;
        self
    }
    /// <p>The type of resource, for example a VPC attachment.</p>
    pub fn resource_type(
        mut self,
        input: crate::types::TransitGatewayAttachmentResourceType,
    ) -> Self {
        self.resource_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of resource, for example a VPC attachment.</p>
    pub fn set_resource_type(
        mut self,
        input: ::std::option::Option<crate::types::TransitGatewayAttachmentResourceType>,
    ) -> Self {
        self.resource_type = input;
        self
    }
    /// <p> The ID of the Amazon Web Services account that owns the resource.</p>
    pub fn resource_owner_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.resource_owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The ID of the Amazon Web Services account that owns the resource.</p>
    pub fn set_resource_owner_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.resource_owner_id = input;
        self
    }
    /// Appends an item to `subnets`.
    ///
    /// To override the contents of this collection use [`set_subnets`](Self::set_subnets).
    ///
    /// <p>The subnets associated with the multicast domain.</p>
    pub fn subnets(mut self, input: crate::types::SubnetAssociation) -> Self {
        let mut v = self.subnets.unwrap_or_default();
        v.push(input);
        self.subnets = ::std::option::Option::Some(v);
        self
    }
    /// <p>The subnets associated with the multicast domain.</p>
    pub fn set_subnets(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SubnetAssociation>>,
    ) -> Self {
        self.subnets = input;
        self
    }
    /// Consumes the builder and constructs a [`TransitGatewayMulticastDomainAssociations`](crate::types::TransitGatewayMulticastDomainAssociations).
    pub fn build(self) -> crate::types::TransitGatewayMulticastDomainAssociations {
        crate::types::TransitGatewayMulticastDomainAssociations {
            transit_gateway_multicast_domain_id: self.transit_gateway_multicast_domain_id,
            transit_gateway_attachment_id: self.transit_gateway_attachment_id,
            resource_id: self.resource_id,
            resource_type: self.resource_type,
            resource_owner_id: self.resource_owner_id,
            subnets: self.subnets,
        }
    }
}
