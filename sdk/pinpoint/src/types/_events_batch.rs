// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies a batch of endpoints and events to process.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EventsBatch {
    /// <p>A set of properties and attributes that are associated with the endpoint.</p>
    #[doc(hidden)]
    pub endpoint: ::std::option::Option<crate::types::PublicEndpoint>,
    /// <p>A set of properties that are associated with the event.</p>
    #[doc(hidden)]
    pub events: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::Event>,
    >,
}
impl EventsBatch {
    /// <p>A set of properties and attributes that are associated with the endpoint.</p>
    pub fn endpoint(&self) -> ::std::option::Option<&crate::types::PublicEndpoint> {
        self.endpoint.as_ref()
    }
    /// <p>A set of properties that are associated with the event.</p>
    pub fn events(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, crate::types::Event>,
    > {
        self.events.as_ref()
    }
}
impl EventsBatch {
    /// Creates a new builder-style object to manufacture [`EventsBatch`](crate::types::EventsBatch).
    pub fn builder() -> crate::types::builders::EventsBatchBuilder {
        crate::types::builders::EventsBatchBuilder::default()
    }
}

/// A builder for [`EventsBatch`](crate::types::EventsBatch).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EventsBatchBuilder {
    pub(crate) endpoint: ::std::option::Option<crate::types::PublicEndpoint>,
    pub(crate) events: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::Event>,
    >,
}
impl EventsBatchBuilder {
    /// <p>A set of properties and attributes that are associated with the endpoint.</p>
    pub fn endpoint(mut self, input: crate::types::PublicEndpoint) -> Self {
        self.endpoint = ::std::option::Option::Some(input);
        self
    }
    /// <p>A set of properties and attributes that are associated with the endpoint.</p>
    pub fn set_endpoint(
        mut self,
        input: ::std::option::Option<crate::types::PublicEndpoint>,
    ) -> Self {
        self.endpoint = input;
        self
    }
    /// Adds a key-value pair to `events`.
    ///
    /// To override the contents of this collection use [`set_events`](Self::set_events).
    ///
    /// <p>A set of properties that are associated with the event.</p>
    pub fn events(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::Event,
    ) -> Self {
        let mut hash_map = self.events.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.events = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A set of properties that are associated with the event.</p>
    pub fn set_events(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::Event>,
        >,
    ) -> Self {
        self.events = input;
        self
    }
    /// Consumes the builder and constructs a [`EventsBatch`](crate::types::EventsBatch).
    pub fn build(self) -> crate::types::EventsBatch {
        crate::types::EventsBatch {
            endpoint: self.endpoint,
            events: self.events,
        }
    }
}
