// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateWirelessGatewayWithThingInput {
    /// <p>The ID of the resource to update.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the thing to associate with the wireless gateway.</p>
    #[doc(hidden)]
    pub thing_arn: ::std::option::Option<::std::string::String>,
}
impl AssociateWirelessGatewayWithThingInput {
    /// <p>The ID of the resource to update.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The ARN of the thing to associate with the wireless gateway.</p>
    pub fn thing_arn(&self) -> ::std::option::Option<&str> {
        self.thing_arn.as_deref()
    }
}
impl AssociateWirelessGatewayWithThingInput {
    /// Creates a new builder-style object to manufacture [`AssociateWirelessGatewayWithThingInput`](crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingInput).
    pub fn builder() -> crate::operation::associate_wireless_gateway_with_thing::builders::AssociateWirelessGatewayWithThingInputBuilder{
        crate::operation::associate_wireless_gateway_with_thing::builders::AssociateWirelessGatewayWithThingInputBuilder::default()
    }
}

/// A builder for [`AssociateWirelessGatewayWithThingInput`](crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AssociateWirelessGatewayWithThingInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) thing_arn: ::std::option::Option<::std::string::String>,
}
impl AssociateWirelessGatewayWithThingInputBuilder {
    /// <p>The ID of the resource to update.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the resource to update.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The ARN of the thing to associate with the wireless gateway.</p>
    pub fn thing_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.thing_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the thing to associate with the wireless gateway.</p>
    pub fn set_thing_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.thing_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociateWirelessGatewayWithThingInput`](crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingInput {
                id: self.id
                ,
                thing_arn: self.thing_arn
                ,
            }
        )
    }
}
