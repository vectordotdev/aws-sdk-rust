// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>For a custom routing accelerator, describes the port range and protocol for all endpoints (virtual private cloud subnets) in an endpoint group to accept client traffic on.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CustomRoutingDestinationDescription {
    /// <p>The first port, inclusive, in the range of ports for the endpoint group that is associated with a custom routing accelerator.</p>
    #[doc(hidden)]
    pub from_port: ::std::option::Option<i32>,
    /// <p>The last port, inclusive, in the range of ports for the endpoint group that is associated with a custom routing accelerator.</p>
    #[doc(hidden)]
    pub to_port: ::std::option::Option<i32>,
    /// <p>The protocol for the endpoint group that is associated with a custom routing accelerator. The protocol can be either TCP or UDP.</p>
    #[doc(hidden)]
    pub protocols: ::std::option::Option<::std::vec::Vec<crate::types::Protocol>>,
}
impl CustomRoutingDestinationDescription {
    /// <p>The first port, inclusive, in the range of ports for the endpoint group that is associated with a custom routing accelerator.</p>
    pub fn from_port(&self) -> ::std::option::Option<i32> {
        self.from_port
    }
    /// <p>The last port, inclusive, in the range of ports for the endpoint group that is associated with a custom routing accelerator.</p>
    pub fn to_port(&self) -> ::std::option::Option<i32> {
        self.to_port
    }
    /// <p>The protocol for the endpoint group that is associated with a custom routing accelerator. The protocol can be either TCP or UDP.</p>
    pub fn protocols(&self) -> ::std::option::Option<&[crate::types::Protocol]> {
        self.protocols.as_deref()
    }
}
impl CustomRoutingDestinationDescription {
    /// Creates a new builder-style object to manufacture [`CustomRoutingDestinationDescription`](crate::types::CustomRoutingDestinationDescription).
    pub fn builder() -> crate::types::builders::CustomRoutingDestinationDescriptionBuilder {
        crate::types::builders::CustomRoutingDestinationDescriptionBuilder::default()
    }
}

/// A builder for [`CustomRoutingDestinationDescription`](crate::types::CustomRoutingDestinationDescription).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CustomRoutingDestinationDescriptionBuilder {
    pub(crate) from_port: ::std::option::Option<i32>,
    pub(crate) to_port: ::std::option::Option<i32>,
    pub(crate) protocols: ::std::option::Option<::std::vec::Vec<crate::types::Protocol>>,
}
impl CustomRoutingDestinationDescriptionBuilder {
    /// <p>The first port, inclusive, in the range of ports for the endpoint group that is associated with a custom routing accelerator.</p>
    pub fn from_port(mut self, input: i32) -> Self {
        self.from_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The first port, inclusive, in the range of ports for the endpoint group that is associated with a custom routing accelerator.</p>
    pub fn set_from_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.from_port = input;
        self
    }
    /// <p>The last port, inclusive, in the range of ports for the endpoint group that is associated with a custom routing accelerator.</p>
    pub fn to_port(mut self, input: i32) -> Self {
        self.to_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last port, inclusive, in the range of ports for the endpoint group that is associated with a custom routing accelerator.</p>
    pub fn set_to_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.to_port = input;
        self
    }
    /// Appends an item to `protocols`.
    ///
    /// To override the contents of this collection use [`set_protocols`](Self::set_protocols).
    ///
    /// <p>The protocol for the endpoint group that is associated with a custom routing accelerator. The protocol can be either TCP or UDP.</p>
    pub fn protocols(mut self, input: crate::types::Protocol) -> Self {
        let mut v = self.protocols.unwrap_or_default();
        v.push(input);
        self.protocols = ::std::option::Option::Some(v);
        self
    }
    /// <p>The protocol for the endpoint group that is associated with a custom routing accelerator. The protocol can be either TCP or UDP.</p>
    pub fn set_protocols(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Protocol>>,
    ) -> Self {
        self.protocols = input;
        self
    }
    /// Consumes the builder and constructs a [`CustomRoutingDestinationDescription`](crate::types::CustomRoutingDestinationDescription).
    pub fn build(self) -> crate::types::CustomRoutingDestinationDescription {
        crate::types::CustomRoutingDestinationDescription {
            from_port: self.from_port,
            to_port: self.to_port,
            protocols: self.protocols,
        }
    }
}
