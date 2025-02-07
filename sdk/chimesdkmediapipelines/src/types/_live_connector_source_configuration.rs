// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The data source configuration object of a streaming media pipeline.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LiveConnectorSourceConfiguration {
    /// <p>The source configuration's media source type.</p>
    #[doc(hidden)]
    pub source_type: ::std::option::Option<crate::types::LiveConnectorSourceType>,
    /// <p>The configuration settings of the connector pipeline.</p>
    #[doc(hidden)]
    pub chime_sdk_meeting_live_connector_configuration:
        ::std::option::Option<crate::types::ChimeSdkMeetingLiveConnectorConfiguration>,
}
impl LiveConnectorSourceConfiguration {
    /// <p>The source configuration's media source type.</p>
    pub fn source_type(&self) -> ::std::option::Option<&crate::types::LiveConnectorSourceType> {
        self.source_type.as_ref()
    }
    /// <p>The configuration settings of the connector pipeline.</p>
    pub fn chime_sdk_meeting_live_connector_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::ChimeSdkMeetingLiveConnectorConfiguration> {
        self.chime_sdk_meeting_live_connector_configuration.as_ref()
    }
}
impl LiveConnectorSourceConfiguration {
    /// Creates a new builder-style object to manufacture [`LiveConnectorSourceConfiguration`](crate::types::LiveConnectorSourceConfiguration).
    pub fn builder() -> crate::types::builders::LiveConnectorSourceConfigurationBuilder {
        crate::types::builders::LiveConnectorSourceConfigurationBuilder::default()
    }
}

/// A builder for [`LiveConnectorSourceConfiguration`](crate::types::LiveConnectorSourceConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LiveConnectorSourceConfigurationBuilder {
    pub(crate) source_type: ::std::option::Option<crate::types::LiveConnectorSourceType>,
    pub(crate) chime_sdk_meeting_live_connector_configuration:
        ::std::option::Option<crate::types::ChimeSdkMeetingLiveConnectorConfiguration>,
}
impl LiveConnectorSourceConfigurationBuilder {
    /// <p>The source configuration's media source type.</p>
    pub fn source_type(mut self, input: crate::types::LiveConnectorSourceType) -> Self {
        self.source_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The source configuration's media source type.</p>
    pub fn set_source_type(
        mut self,
        input: ::std::option::Option<crate::types::LiveConnectorSourceType>,
    ) -> Self {
        self.source_type = input;
        self
    }
    /// <p>The configuration settings of the connector pipeline.</p>
    pub fn chime_sdk_meeting_live_connector_configuration(
        mut self,
        input: crate::types::ChimeSdkMeetingLiveConnectorConfiguration,
    ) -> Self {
        self.chime_sdk_meeting_live_connector_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration settings of the connector pipeline.</p>
    pub fn set_chime_sdk_meeting_live_connector_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ChimeSdkMeetingLiveConnectorConfiguration>,
    ) -> Self {
        self.chime_sdk_meeting_live_connector_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`LiveConnectorSourceConfiguration`](crate::types::LiveConnectorSourceConfiguration).
    pub fn build(self) -> crate::types::LiveConnectorSourceConfiguration {
        crate::types::LiveConnectorSourceConfiguration {
            source_type: self.source_type,
            chime_sdk_meeting_live_connector_configuration: self
                .chime_sdk_meeting_live_connector_configuration,
        }
    }
}
