// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an event for a database.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RelationalDatabaseEvent {
    /// <p>The database that the database event relates to.</p>
    #[doc(hidden)]
    pub resource: ::std::option::Option<::std::string::String>,
    /// <p>The timestamp when the database event was created.</p>
    #[doc(hidden)]
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The message of the database event.</p>
    #[doc(hidden)]
    pub message: ::std::option::Option<::std::string::String>,
    /// <p>The category that the database event belongs to.</p>
    #[doc(hidden)]
    pub event_categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl RelationalDatabaseEvent {
    /// <p>The database that the database event relates to.</p>
    pub fn resource(&self) -> ::std::option::Option<&str> {
        self.resource.as_deref()
    }
    /// <p>The timestamp when the database event was created.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>The message of the database event.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
    /// <p>The category that the database event belongs to.</p>
    pub fn event_categories(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.event_categories.as_deref()
    }
}
impl RelationalDatabaseEvent {
    /// Creates a new builder-style object to manufacture [`RelationalDatabaseEvent`](crate::types::RelationalDatabaseEvent).
    pub fn builder() -> crate::types::builders::RelationalDatabaseEventBuilder {
        crate::types::builders::RelationalDatabaseEventBuilder::default()
    }
}

/// A builder for [`RelationalDatabaseEvent`](crate::types::RelationalDatabaseEvent).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RelationalDatabaseEventBuilder {
    pub(crate) resource: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
    pub(crate) event_categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl RelationalDatabaseEventBuilder {
    /// <p>The database that the database event relates to.</p>
    pub fn resource(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The database that the database event relates to.</p>
    pub fn set_resource(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource = input;
        self
    }
    /// <p>The timestamp when the database event was created.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp when the database event was created.</p>
    pub fn set_created_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_at = input;
        self
    }
    /// <p>The message of the database event.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The message of the database event.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Appends an item to `event_categories`.
    ///
    /// To override the contents of this collection use [`set_event_categories`](Self::set_event_categories).
    ///
    /// <p>The category that the database event belongs to.</p>
    pub fn event_categories(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.event_categories.unwrap_or_default();
        v.push(input.into());
        self.event_categories = ::std::option::Option::Some(v);
        self
    }
    /// <p>The category that the database event belongs to.</p>
    pub fn set_event_categories(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.event_categories = input;
        self
    }
    /// Consumes the builder and constructs a [`RelationalDatabaseEvent`](crate::types::RelationalDatabaseEvent).
    pub fn build(self) -> crate::types::RelationalDatabaseEvent {
        crate::types::RelationalDatabaseEvent {
            resource: self.resource,
            created_at: self.created_at,
            message: self.message,
            event_categories: self.event_categories,
        }
    }
}
