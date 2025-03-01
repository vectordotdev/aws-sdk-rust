// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a transit gateway policy table.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransitGatewayPolicyTable {
    /// <p>The ID of the transit gateway policy table.</p>
    #[doc(hidden)]
    pub transit_gateway_policy_table_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the transit gateway.</p>
    #[doc(hidden)]
    pub transit_gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The state of the transit gateway policy table</p>
    #[doc(hidden)]
    pub state: ::std::option::Option<crate::types::TransitGatewayPolicyTableState>,
    /// <p>The timestamp when the transit gateway policy table was created.</p>
    #[doc(hidden)]
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>he key-value pairs associated with the transit gateway policy table.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl TransitGatewayPolicyTable {
    /// <p>The ID of the transit gateway policy table.</p>
    pub fn transit_gateway_policy_table_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_policy_table_id.as_deref()
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn transit_gateway_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_id.as_deref()
    }
    /// <p>The state of the transit gateway policy table</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::TransitGatewayPolicyTableState> {
        self.state.as_ref()
    }
    /// <p>The timestamp when the transit gateway policy table was created.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>he key-value pairs associated with the transit gateway policy table.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl TransitGatewayPolicyTable {
    /// Creates a new builder-style object to manufacture [`TransitGatewayPolicyTable`](crate::types::TransitGatewayPolicyTable).
    pub fn builder() -> crate::types::builders::TransitGatewayPolicyTableBuilder {
        crate::types::builders::TransitGatewayPolicyTableBuilder::default()
    }
}

/// A builder for [`TransitGatewayPolicyTable`](crate::types::TransitGatewayPolicyTable).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TransitGatewayPolicyTableBuilder {
    pub(crate) transit_gateway_policy_table_id: ::std::option::Option<::std::string::String>,
    pub(crate) transit_gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::TransitGatewayPolicyTableState>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl TransitGatewayPolicyTableBuilder {
    /// <p>The ID of the transit gateway policy table.</p>
    pub fn transit_gateway_policy_table_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.transit_gateway_policy_table_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway policy table.</p>
    pub fn set_transit_gateway_policy_table_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.transit_gateway_policy_table_id = input;
        self
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn transit_gateway_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.transit_gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn set_transit_gateway_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.transit_gateway_id = input;
        self
    }
    /// <p>The state of the transit gateway policy table</p>
    pub fn state(mut self, input: crate::types::TransitGatewayPolicyTableState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the transit gateway policy table</p>
    pub fn set_state(
        mut self,
        input: ::std::option::Option<crate::types::TransitGatewayPolicyTableState>,
    ) -> Self {
        self.state = input;
        self
    }
    /// <p>The timestamp when the transit gateway policy table was created.</p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp when the transit gateway policy table was created.</p>
    pub fn set_creation_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>he key-value pairs associated with the transit gateway policy table.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>he key-value pairs associated with the transit gateway policy table.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`TransitGatewayPolicyTable`](crate::types::TransitGatewayPolicyTable).
    pub fn build(self) -> crate::types::TransitGatewayPolicyTable {
        crate::types::TransitGatewayPolicyTable {
            transit_gateway_policy_table_id: self.transit_gateway_policy_table_id,
            transit_gateway_id: self.transit_gateway_id,
            state: self.state,
            creation_time: self.creation_time,
            tags: self.tags,
        }
    }
}
