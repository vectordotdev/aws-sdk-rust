// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The ThingTypeProperties contains information about the thing type including: a thing type description, and a list of searchable thing attribute names.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ThingTypeProperties {
    /// <p>The description of the thing type.</p>
    #[doc(hidden)]
    pub thing_type_description: ::std::option::Option<::std::string::String>,
    /// <p>A list of searchable thing attribute names.</p>
    #[doc(hidden)]
    pub searchable_attributes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ThingTypeProperties {
    /// <p>The description of the thing type.</p>
    pub fn thing_type_description(&self) -> ::std::option::Option<&str> {
        self.thing_type_description.as_deref()
    }
    /// <p>A list of searchable thing attribute names.</p>
    pub fn searchable_attributes(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.searchable_attributes.as_deref()
    }
}
impl ThingTypeProperties {
    /// Creates a new builder-style object to manufacture [`ThingTypeProperties`](crate::types::ThingTypeProperties).
    pub fn builder() -> crate::types::builders::ThingTypePropertiesBuilder {
        crate::types::builders::ThingTypePropertiesBuilder::default()
    }
}

/// A builder for [`ThingTypeProperties`](crate::types::ThingTypeProperties).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ThingTypePropertiesBuilder {
    pub(crate) thing_type_description: ::std::option::Option<::std::string::String>,
    pub(crate) searchable_attributes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ThingTypePropertiesBuilder {
    /// <p>The description of the thing type.</p>
    pub fn thing_type_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.thing_type_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the thing type.</p>
    pub fn set_thing_type_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.thing_type_description = input;
        self
    }
    /// Appends an item to `searchable_attributes`.
    ///
    /// To override the contents of this collection use [`set_searchable_attributes`](Self::set_searchable_attributes).
    ///
    /// <p>A list of searchable thing attribute names.</p>
    pub fn searchable_attributes(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.searchable_attributes.unwrap_or_default();
        v.push(input.into());
        self.searchable_attributes = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of searchable thing attribute names.</p>
    pub fn set_searchable_attributes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.searchable_attributes = input;
        self
    }
    /// Consumes the builder and constructs a [`ThingTypeProperties`](crate::types::ThingTypeProperties).
    pub fn build(self) -> crate::types::ThingTypeProperties {
        crate::types::ThingTypeProperties {
            thing_type_description: self.thing_type_description,
            searchable_attributes: self.searchable_attributes,
        }
    }
}
