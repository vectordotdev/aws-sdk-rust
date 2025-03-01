// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociatePhoneNumbersFromVoiceConnectorGroupInput {
    /// <p>The Voice Connector group ID.</p>
    #[doc(hidden)]
    pub voice_connector_group_id: ::std::option::Option<::std::string::String>,
    /// <p>The list of phone numbers, in E.164 format.</p>
    #[doc(hidden)]
    pub e164_phone_numbers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DisassociatePhoneNumbersFromVoiceConnectorGroupInput {
    /// <p>The Voice Connector group ID.</p>
    pub fn voice_connector_group_id(&self) -> ::std::option::Option<&str> {
        self.voice_connector_group_id.as_deref()
    }
    /// <p>The list of phone numbers, in E.164 format.</p>
    pub fn e164_phone_numbers(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.e164_phone_numbers.as_deref()
    }
}
impl DisassociatePhoneNumbersFromVoiceConnectorGroupInput {
    /// Creates a new builder-style object to manufacture [`DisassociatePhoneNumbersFromVoiceConnectorGroupInput`](crate::operation::disassociate_phone_numbers_from_voice_connector_group::DisassociatePhoneNumbersFromVoiceConnectorGroupInput).
    pub fn builder() -> crate::operation::disassociate_phone_numbers_from_voice_connector_group::builders::DisassociatePhoneNumbersFromVoiceConnectorGroupInputBuilder{
        crate::operation::disassociate_phone_numbers_from_voice_connector_group::builders::DisassociatePhoneNumbersFromVoiceConnectorGroupInputBuilder::default()
    }
}

/// A builder for [`DisassociatePhoneNumbersFromVoiceConnectorGroupInput`](crate::operation::disassociate_phone_numbers_from_voice_connector_group::DisassociatePhoneNumbersFromVoiceConnectorGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DisassociatePhoneNumbersFromVoiceConnectorGroupInputBuilder {
    pub(crate) voice_connector_group_id: ::std::option::Option<::std::string::String>,
    pub(crate) e164_phone_numbers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DisassociatePhoneNumbersFromVoiceConnectorGroupInputBuilder {
    /// <p>The Voice Connector group ID.</p>
    pub fn voice_connector_group_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.voice_connector_group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Voice Connector group ID.</p>
    pub fn set_voice_connector_group_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.voice_connector_group_id = input;
        self
    }
    /// Appends an item to `e164_phone_numbers`.
    ///
    /// To override the contents of this collection use [`set_e164_phone_numbers`](Self::set_e164_phone_numbers).
    ///
    /// <p>The list of phone numbers, in E.164 format.</p>
    pub fn e164_phone_numbers(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.e164_phone_numbers.unwrap_or_default();
        v.push(input.into());
        self.e164_phone_numbers = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of phone numbers, in E.164 format.</p>
    pub fn set_e164_phone_numbers(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.e164_phone_numbers = input;
        self
    }
    /// Consumes the builder and constructs a [`DisassociatePhoneNumbersFromVoiceConnectorGroupInput`](crate::operation::disassociate_phone_numbers_from_voice_connector_group::DisassociatePhoneNumbersFromVoiceConnectorGroupInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::disassociate_phone_numbers_from_voice_connector_group::DisassociatePhoneNumbersFromVoiceConnectorGroupInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::disassociate_phone_numbers_from_voice_connector_group::DisassociatePhoneNumbersFromVoiceConnectorGroupInput {
                voice_connector_group_id: self.voice_connector_group_id
                ,
                e164_phone_numbers: self.e164_phone_numbers
                ,
            }
        )
    }
}
