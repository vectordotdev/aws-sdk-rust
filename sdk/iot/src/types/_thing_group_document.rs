// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The thing group search index document.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ThingGroupDocument {
    /// <p>The thing group name.</p>
    #[doc(hidden)]
    pub thing_group_name: ::std::option::Option<::std::string::String>,
    /// <p>The thing group ID.</p>
    #[doc(hidden)]
    pub thing_group_id: ::std::option::Option<::std::string::String>,
    /// <p>The thing group description.</p>
    #[doc(hidden)]
    pub thing_group_description: ::std::option::Option<::std::string::String>,
    /// <p>The thing group attributes.</p>
    #[doc(hidden)]
    pub attributes: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>Parent group names.</p>
    #[doc(hidden)]
    pub parent_group_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ThingGroupDocument {
    /// <p>The thing group name.</p>
    pub fn thing_group_name(&self) -> ::std::option::Option<&str> {
        self.thing_group_name.as_deref()
    }
    /// <p>The thing group ID.</p>
    pub fn thing_group_id(&self) -> ::std::option::Option<&str> {
        self.thing_group_id.as_deref()
    }
    /// <p>The thing group description.</p>
    pub fn thing_group_description(&self) -> ::std::option::Option<&str> {
        self.thing_group_description.as_deref()
    }
    /// <p>The thing group attributes.</p>
    pub fn attributes(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.attributes.as_ref()
    }
    /// <p>Parent group names.</p>
    pub fn parent_group_names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.parent_group_names.as_deref()
    }
}
impl ThingGroupDocument {
    /// Creates a new builder-style object to manufacture [`ThingGroupDocument`](crate::types::ThingGroupDocument).
    pub fn builder() -> crate::types::builders::ThingGroupDocumentBuilder {
        crate::types::builders::ThingGroupDocumentBuilder::default()
    }
}

/// A builder for [`ThingGroupDocument`](crate::types::ThingGroupDocument).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ThingGroupDocumentBuilder {
    pub(crate) thing_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) thing_group_id: ::std::option::Option<::std::string::String>,
    pub(crate) thing_group_description: ::std::option::Option<::std::string::String>,
    pub(crate) attributes: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) parent_group_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ThingGroupDocumentBuilder {
    /// <p>The thing group name.</p>
    pub fn thing_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.thing_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The thing group name.</p>
    pub fn set_thing_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.thing_group_name = input;
        self
    }
    /// <p>The thing group ID.</p>
    pub fn thing_group_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.thing_group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The thing group ID.</p>
    pub fn set_thing_group_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.thing_group_id = input;
        self
    }
    /// <p>The thing group description.</p>
    pub fn thing_group_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.thing_group_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The thing group description.</p>
    pub fn set_thing_group_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.thing_group_description = input;
        self
    }
    /// Adds a key-value pair to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>The thing group attributes.</p>
    pub fn attributes(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.attributes.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.attributes = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The thing group attributes.</p>
    pub fn set_attributes(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.attributes = input;
        self
    }
    /// Appends an item to `parent_group_names`.
    ///
    /// To override the contents of this collection use [`set_parent_group_names`](Self::set_parent_group_names).
    ///
    /// <p>Parent group names.</p>
    pub fn parent_group_names(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.parent_group_names.unwrap_or_default();
        v.push(input.into());
        self.parent_group_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>Parent group names.</p>
    pub fn set_parent_group_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.parent_group_names = input;
        self
    }
    /// Consumes the builder and constructs a [`ThingGroupDocument`](crate::types::ThingGroupDocument).
    pub fn build(self) -> crate::types::ThingGroupDocument {
        crate::types::ThingGroupDocument {
            thing_group_name: self.thing_group_name,
            thing_group_id: self.thing_group_id,
            thing_group_description: self.thing_group_description,
            attributes: self.attributes,
            parent_group_names: self.parent_group_names,
        }
    }
}
