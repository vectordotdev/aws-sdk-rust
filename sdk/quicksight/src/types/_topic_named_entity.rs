// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A structure that represents a named entity.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TopicNamedEntity {
    /// <p>The name of the named entity.</p>
    #[doc(hidden)]
    pub entity_name: ::std::option::Option<::std::string::String>,
    /// <p>The description of the named entity.</p>
    #[doc(hidden)]
    pub entity_description: ::std::option::Option<::std::string::String>,
    /// <p>The other names or aliases for the named entity.</p>
    #[doc(hidden)]
    pub entity_synonyms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The type of named entity that a topic represents.</p>
    #[doc(hidden)]
    pub semantic_entity_type: ::std::option::Option<crate::types::SemanticEntityType>,
    /// <p>The definition of a named entity.</p>
    #[doc(hidden)]
    pub definition: ::std::option::Option<::std::vec::Vec<crate::types::NamedEntityDefinition>>,
}
impl TopicNamedEntity {
    /// <p>The name of the named entity.</p>
    pub fn entity_name(&self) -> ::std::option::Option<&str> {
        self.entity_name.as_deref()
    }
    /// <p>The description of the named entity.</p>
    pub fn entity_description(&self) -> ::std::option::Option<&str> {
        self.entity_description.as_deref()
    }
    /// <p>The other names or aliases for the named entity.</p>
    pub fn entity_synonyms(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.entity_synonyms.as_deref()
    }
    /// <p>The type of named entity that a topic represents.</p>
    pub fn semantic_entity_type(&self) -> ::std::option::Option<&crate::types::SemanticEntityType> {
        self.semantic_entity_type.as_ref()
    }
    /// <p>The definition of a named entity.</p>
    pub fn definition(&self) -> ::std::option::Option<&[crate::types::NamedEntityDefinition]> {
        self.definition.as_deref()
    }
}
impl TopicNamedEntity {
    /// Creates a new builder-style object to manufacture [`TopicNamedEntity`](crate::types::TopicNamedEntity).
    pub fn builder() -> crate::types::builders::TopicNamedEntityBuilder {
        crate::types::builders::TopicNamedEntityBuilder::default()
    }
}

/// A builder for [`TopicNamedEntity`](crate::types::TopicNamedEntity).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TopicNamedEntityBuilder {
    pub(crate) entity_name: ::std::option::Option<::std::string::String>,
    pub(crate) entity_description: ::std::option::Option<::std::string::String>,
    pub(crate) entity_synonyms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) semantic_entity_type: ::std::option::Option<crate::types::SemanticEntityType>,
    pub(crate) definition:
        ::std::option::Option<::std::vec::Vec<crate::types::NamedEntityDefinition>>,
}
impl TopicNamedEntityBuilder {
    /// <p>The name of the named entity.</p>
    pub fn entity_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.entity_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the named entity.</p>
    pub fn set_entity_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.entity_name = input;
        self
    }
    /// <p>The description of the named entity.</p>
    pub fn entity_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.entity_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the named entity.</p>
    pub fn set_entity_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.entity_description = input;
        self
    }
    /// Appends an item to `entity_synonyms`.
    ///
    /// To override the contents of this collection use [`set_entity_synonyms`](Self::set_entity_synonyms).
    ///
    /// <p>The other names or aliases for the named entity.</p>
    pub fn entity_synonyms(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.entity_synonyms.unwrap_or_default();
        v.push(input.into());
        self.entity_synonyms = ::std::option::Option::Some(v);
        self
    }
    /// <p>The other names or aliases for the named entity.</p>
    pub fn set_entity_synonyms(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.entity_synonyms = input;
        self
    }
    /// <p>The type of named entity that a topic represents.</p>
    pub fn semantic_entity_type(mut self, input: crate::types::SemanticEntityType) -> Self {
        self.semantic_entity_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of named entity that a topic represents.</p>
    pub fn set_semantic_entity_type(
        mut self,
        input: ::std::option::Option<crate::types::SemanticEntityType>,
    ) -> Self {
        self.semantic_entity_type = input;
        self
    }
    /// Appends an item to `definition`.
    ///
    /// To override the contents of this collection use [`set_definition`](Self::set_definition).
    ///
    /// <p>The definition of a named entity.</p>
    pub fn definition(mut self, input: crate::types::NamedEntityDefinition) -> Self {
        let mut v = self.definition.unwrap_or_default();
        v.push(input);
        self.definition = ::std::option::Option::Some(v);
        self
    }
    /// <p>The definition of a named entity.</p>
    pub fn set_definition(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::NamedEntityDefinition>>,
    ) -> Self {
        self.definition = input;
        self
    }
    /// Consumes the builder and constructs a [`TopicNamedEntity`](crate::types::TopicNamedEntity).
    pub fn build(self) -> crate::types::TopicNamedEntity {
        crate::types::TopicNamedEntity {
            entity_name: self.entity_name,
            entity_description: self.entity_description,
            entity_synonyms: self.entity_synonyms,
            semantic_entity_type: self.semantic_entity_type,
            definition: self.definition,
        }
    }
}
