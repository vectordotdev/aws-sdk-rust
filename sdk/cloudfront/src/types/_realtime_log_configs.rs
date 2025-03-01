// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A list of real-time log configurations.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RealtimeLogConfigs {
    /// <p>The maximum number of real-time log configurations requested.</p>
    #[doc(hidden)]
    pub max_items: ::std::option::Option<i32>,
    /// <p>Contains the list of real-time log configurations.</p>
    #[doc(hidden)]
    pub items: ::std::option::Option<::std::vec::Vec<crate::types::RealtimeLogConfig>>,
    /// <p>A flag that indicates whether there are more real-time log configurations than are contained in this list.</p>
    #[doc(hidden)]
    pub is_truncated: ::std::option::Option<bool>,
    /// <p>This parameter indicates where this list of real-time log configurations begins. This list includes real-time log configurations that occur after the marker.</p>
    #[doc(hidden)]
    pub marker: ::std::option::Option<::std::string::String>,
    /// <p>If there are more items in the list than are in this response, this element is present. It contains the value that you should use in the <code>Marker</code> field of a subsequent request to continue listing real-time log configurations where you left off. </p>
    #[doc(hidden)]
    pub next_marker: ::std::option::Option<::std::string::String>,
}
impl RealtimeLogConfigs {
    /// <p>The maximum number of real-time log configurations requested.</p>
    pub fn max_items(&self) -> ::std::option::Option<i32> {
        self.max_items
    }
    /// <p>Contains the list of real-time log configurations.</p>
    pub fn items(&self) -> ::std::option::Option<&[crate::types::RealtimeLogConfig]> {
        self.items.as_deref()
    }
    /// <p>A flag that indicates whether there are more real-time log configurations than are contained in this list.</p>
    pub fn is_truncated(&self) -> ::std::option::Option<bool> {
        self.is_truncated
    }
    /// <p>This parameter indicates where this list of real-time log configurations begins. This list includes real-time log configurations that occur after the marker.</p>
    pub fn marker(&self) -> ::std::option::Option<&str> {
        self.marker.as_deref()
    }
    /// <p>If there are more items in the list than are in this response, this element is present. It contains the value that you should use in the <code>Marker</code> field of a subsequent request to continue listing real-time log configurations where you left off. </p>
    pub fn next_marker(&self) -> ::std::option::Option<&str> {
        self.next_marker.as_deref()
    }
}
impl RealtimeLogConfigs {
    /// Creates a new builder-style object to manufacture [`RealtimeLogConfigs`](crate::types::RealtimeLogConfigs).
    pub fn builder() -> crate::types::builders::RealtimeLogConfigsBuilder {
        crate::types::builders::RealtimeLogConfigsBuilder::default()
    }
}

/// A builder for [`RealtimeLogConfigs`](crate::types::RealtimeLogConfigs).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RealtimeLogConfigsBuilder {
    pub(crate) max_items: ::std::option::Option<i32>,
    pub(crate) items: ::std::option::Option<::std::vec::Vec<crate::types::RealtimeLogConfig>>,
    pub(crate) is_truncated: ::std::option::Option<bool>,
    pub(crate) marker: ::std::option::Option<::std::string::String>,
    pub(crate) next_marker: ::std::option::Option<::std::string::String>,
}
impl RealtimeLogConfigsBuilder {
    /// <p>The maximum number of real-time log configurations requested.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.max_items = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of real-time log configurations requested.</p>
    pub fn set_max_items(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_items = input;
        self
    }
    /// Appends an item to `items`.
    ///
    /// To override the contents of this collection use [`set_items`](Self::set_items).
    ///
    /// <p>Contains the list of real-time log configurations.</p>
    pub fn items(mut self, input: crate::types::RealtimeLogConfig) -> Self {
        let mut v = self.items.unwrap_or_default();
        v.push(input);
        self.items = ::std::option::Option::Some(v);
        self
    }
    /// <p>Contains the list of real-time log configurations.</p>
    pub fn set_items(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RealtimeLogConfig>>,
    ) -> Self {
        self.items = input;
        self
    }
    /// <p>A flag that indicates whether there are more real-time log configurations than are contained in this list.</p>
    pub fn is_truncated(mut self, input: bool) -> Self {
        self.is_truncated = ::std::option::Option::Some(input);
        self
    }
    /// <p>A flag that indicates whether there are more real-time log configurations than are contained in this list.</p>
    pub fn set_is_truncated(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_truncated = input;
        self
    }
    /// <p>This parameter indicates where this list of real-time log configurations begins. This list includes real-time log configurations that occur after the marker.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>This parameter indicates where this list of real-time log configurations begins. This list includes real-time log configurations that occur after the marker.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// <p>If there are more items in the list than are in this response, this element is present. It contains the value that you should use in the <code>Marker</code> field of a subsequent request to continue listing real-time log configurations where you left off. </p>
    pub fn next_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If there are more items in the list than are in this response, this element is present. It contains the value that you should use in the <code>Marker</code> field of a subsequent request to continue listing real-time log configurations where you left off. </p>
    pub fn set_next_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_marker = input;
        self
    }
    /// Consumes the builder and constructs a [`RealtimeLogConfigs`](crate::types::RealtimeLogConfigs).
    pub fn build(self) -> crate::types::RealtimeLogConfigs {
        crate::types::RealtimeLogConfigs {
            max_items: self.max_items,
            items: self.items,
            is_truncated: self.is_truncated,
            marker: self.marker,
            next_marker: self.next_marker,
        }
    }
}
