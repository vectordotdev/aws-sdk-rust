// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteVoiceConnectorProxyInput {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    #[doc(hidden)]
    pub voice_connector_id: ::std::option::Option<::std::string::String>,
}
impl DeleteVoiceConnectorProxyInput {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    pub fn voice_connector_id(&self) -> ::std::option::Option<&str> {
        self.voice_connector_id.as_deref()
    }
}
impl DeleteVoiceConnectorProxyInput {
    /// Creates a new builder-style object to manufacture [`DeleteVoiceConnectorProxyInput`](crate::operation::delete_voice_connector_proxy::DeleteVoiceConnectorProxyInput).
    pub fn builder() -> crate::operation::delete_voice_connector_proxy::builders::DeleteVoiceConnectorProxyInputBuilder{
        crate::operation::delete_voice_connector_proxy::builders::DeleteVoiceConnectorProxyInputBuilder::default()
    }
}

/// A builder for [`DeleteVoiceConnectorProxyInput`](crate::operation::delete_voice_connector_proxy::DeleteVoiceConnectorProxyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteVoiceConnectorProxyInputBuilder {
    pub(crate) voice_connector_id: ::std::option::Option<::std::string::String>,
}
impl DeleteVoiceConnectorProxyInputBuilder {
    /// <p>The Amazon Chime Voice Connector ID.</p>
    pub fn voice_connector_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.voice_connector_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Chime Voice Connector ID.</p>
    pub fn set_voice_connector_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.voice_connector_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteVoiceConnectorProxyInput`](crate::operation::delete_voice_connector_proxy::DeleteVoiceConnectorProxyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_voice_connector_proxy::DeleteVoiceConnectorProxyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_voice_connector_proxy::DeleteVoiceConnectorProxyInput {
                voice_connector_id: self.voice_connector_id,
            },
        )
    }
}
