// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The log options for wireless devices and can be used to set log levels for a specific type of wireless device.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct WirelessDeviceLogOption {
    /// <p>The wireless device type.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::WirelessDeviceType>,
    /// <p>The log level for a log message. The log levels can be disabled, or set to <code>ERROR</code> to display less verbose logs containing only error information, or to <code>INFO</code> for more detailed logs.</p>
    #[doc(hidden)]
    pub log_level: ::std::option::Option<crate::types::LogLevel>,
    /// <p>The list of wireless device event log options.</p>
    #[doc(hidden)]
    pub events: ::std::option::Option<::std::vec::Vec<crate::types::WirelessDeviceEventLogOption>>,
}
impl WirelessDeviceLogOption {
    /// <p>The wireless device type.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::WirelessDeviceType> {
        self.r#type.as_ref()
    }
    /// <p>The log level for a log message. The log levels can be disabled, or set to <code>ERROR</code> to display less verbose logs containing only error information, or to <code>INFO</code> for more detailed logs.</p>
    pub fn log_level(&self) -> ::std::option::Option<&crate::types::LogLevel> {
        self.log_level.as_ref()
    }
    /// <p>The list of wireless device event log options.</p>
    pub fn events(&self) -> ::std::option::Option<&[crate::types::WirelessDeviceEventLogOption]> {
        self.events.as_deref()
    }
}
impl WirelessDeviceLogOption {
    /// Creates a new builder-style object to manufacture [`WirelessDeviceLogOption`](crate::types::WirelessDeviceLogOption).
    pub fn builder() -> crate::types::builders::WirelessDeviceLogOptionBuilder {
        crate::types::builders::WirelessDeviceLogOptionBuilder::default()
    }
}

/// A builder for [`WirelessDeviceLogOption`](crate::types::WirelessDeviceLogOption).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct WirelessDeviceLogOptionBuilder {
    pub(crate) r#type: ::std::option::Option<crate::types::WirelessDeviceType>,
    pub(crate) log_level: ::std::option::Option<crate::types::LogLevel>,
    pub(crate) events:
        ::std::option::Option<::std::vec::Vec<crate::types::WirelessDeviceEventLogOption>>,
}
impl WirelessDeviceLogOptionBuilder {
    /// <p>The wireless device type.</p>
    pub fn r#type(mut self, input: crate::types::WirelessDeviceType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The wireless device type.</p>
    pub fn set_type(
        mut self,
        input: ::std::option::Option<crate::types::WirelessDeviceType>,
    ) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The log level for a log message. The log levels can be disabled, or set to <code>ERROR</code> to display less verbose logs containing only error information, or to <code>INFO</code> for more detailed logs.</p>
    pub fn log_level(mut self, input: crate::types::LogLevel) -> Self {
        self.log_level = ::std::option::Option::Some(input);
        self
    }
    /// <p>The log level for a log message. The log levels can be disabled, or set to <code>ERROR</code> to display less verbose logs containing only error information, or to <code>INFO</code> for more detailed logs.</p>
    pub fn set_log_level(mut self, input: ::std::option::Option<crate::types::LogLevel>) -> Self {
        self.log_level = input;
        self
    }
    /// Appends an item to `events`.
    ///
    /// To override the contents of this collection use [`set_events`](Self::set_events).
    ///
    /// <p>The list of wireless device event log options.</p>
    pub fn events(mut self, input: crate::types::WirelessDeviceEventLogOption) -> Self {
        let mut v = self.events.unwrap_or_default();
        v.push(input);
        self.events = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of wireless device event log options.</p>
    pub fn set_events(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::WirelessDeviceEventLogOption>>,
    ) -> Self {
        self.events = input;
        self
    }
    /// Consumes the builder and constructs a [`WirelessDeviceLogOption`](crate::types::WirelessDeviceLogOption).
    pub fn build(self) -> crate::types::WirelessDeviceLogOption {
        crate::types::WirelessDeviceLogOption {
            r#type: self.r#type,
            log_level: self.log_level,
            events: self.events,
        }
    }
}
