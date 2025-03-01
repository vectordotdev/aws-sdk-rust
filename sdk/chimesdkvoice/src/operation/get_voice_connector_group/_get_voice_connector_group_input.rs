// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetVoiceConnectorGroupInput {
    /// <p>The Voice Connector group ID.</p>
    #[doc(hidden)]
    pub voice_connector_group_id: ::std::option::Option<::std::string::String>,
}
impl GetVoiceConnectorGroupInput {
    /// <p>The Voice Connector group ID.</p>
    pub fn voice_connector_group_id(&self) -> ::std::option::Option<&str> {
        self.voice_connector_group_id.as_deref()
    }
}
impl GetVoiceConnectorGroupInput {
    /// Creates a new builder-style object to manufacture [`GetVoiceConnectorGroupInput`](crate::operation::get_voice_connector_group::GetVoiceConnectorGroupInput).
    pub fn builder(
    ) -> crate::operation::get_voice_connector_group::builders::GetVoiceConnectorGroupInputBuilder
    {
        crate::operation::get_voice_connector_group::builders::GetVoiceConnectorGroupInputBuilder::default()
    }
}

/// A builder for [`GetVoiceConnectorGroupInput`](crate::operation::get_voice_connector_group::GetVoiceConnectorGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetVoiceConnectorGroupInputBuilder {
    pub(crate) voice_connector_group_id: ::std::option::Option<::std::string::String>,
}
impl GetVoiceConnectorGroupInputBuilder {
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
    /// Consumes the builder and constructs a [`GetVoiceConnectorGroupInput`](crate::operation::get_voice_connector_group::GetVoiceConnectorGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_voice_connector_group::GetVoiceConnectorGroupInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_voice_connector_group::GetVoiceConnectorGroupInput {
                voice_connector_group_id: self.voice_connector_group_id,
            },
        )
    }
}
