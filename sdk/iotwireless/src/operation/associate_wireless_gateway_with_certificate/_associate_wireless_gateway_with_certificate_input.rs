// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateWirelessGatewayWithCertificateInput {
    /// <p>The ID of the resource to update.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the certificate to associate with the wireless gateway.</p>
    #[doc(hidden)]
    pub iot_certificate_id: ::std::option::Option<::std::string::String>,
}
impl AssociateWirelessGatewayWithCertificateInput {
    /// <p>The ID of the resource to update.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The ID of the certificate to associate with the wireless gateway.</p>
    pub fn iot_certificate_id(&self) -> ::std::option::Option<&str> {
        self.iot_certificate_id.as_deref()
    }
}
impl AssociateWirelessGatewayWithCertificateInput {
    /// Creates a new builder-style object to manufacture [`AssociateWirelessGatewayWithCertificateInput`](crate::operation::associate_wireless_gateway_with_certificate::AssociateWirelessGatewayWithCertificateInput).
    pub fn builder() -> crate::operation::associate_wireless_gateway_with_certificate::builders::AssociateWirelessGatewayWithCertificateInputBuilder{
        crate::operation::associate_wireless_gateway_with_certificate::builders::AssociateWirelessGatewayWithCertificateInputBuilder::default()
    }
}

/// A builder for [`AssociateWirelessGatewayWithCertificateInput`](crate::operation::associate_wireless_gateway_with_certificate::AssociateWirelessGatewayWithCertificateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AssociateWirelessGatewayWithCertificateInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) iot_certificate_id: ::std::option::Option<::std::string::String>,
}
impl AssociateWirelessGatewayWithCertificateInputBuilder {
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
    /// <p>The ID of the certificate to associate with the wireless gateway.</p>
    pub fn iot_certificate_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.iot_certificate_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the certificate to associate with the wireless gateway.</p>
    pub fn set_iot_certificate_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.iot_certificate_id = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociateWirelessGatewayWithCertificateInput`](crate::operation::associate_wireless_gateway_with_certificate::AssociateWirelessGatewayWithCertificateInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::associate_wireless_gateway_with_certificate::AssociateWirelessGatewayWithCertificateInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::associate_wireless_gateway_with_certificate::AssociateWirelessGatewayWithCertificateInput {
                id: self.id
                ,
                iot_certificate_id: self.iot_certificate_id
                ,
            }
        )
    }
}
