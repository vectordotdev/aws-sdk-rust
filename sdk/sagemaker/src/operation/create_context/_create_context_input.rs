// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateContextInput {
    /// <p>The name of the context. Must be unique to your account in an Amazon Web Services Region.</p>
    #[doc(hidden)]
    pub context_name: ::std::option::Option<::std::string::String>,
    /// <p>The source type, ID, and URI.</p>
    #[doc(hidden)]
    pub source: ::std::option::Option<crate::types::ContextSource>,
    /// <p>The context type.</p>
    #[doc(hidden)]
    pub context_type: ::std::option::Option<::std::string::String>,
    /// <p>The description of the context.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>A list of properties to add to the context.</p>
    #[doc(hidden)]
    pub properties: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>A list of tags to apply to the context.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateContextInput {
    /// <p>The name of the context. Must be unique to your account in an Amazon Web Services Region.</p>
    pub fn context_name(&self) -> ::std::option::Option<&str> {
        self.context_name.as_deref()
    }
    /// <p>The source type, ID, and URI.</p>
    pub fn source(&self) -> ::std::option::Option<&crate::types::ContextSource> {
        self.source.as_ref()
    }
    /// <p>The context type.</p>
    pub fn context_type(&self) -> ::std::option::Option<&str> {
        self.context_type.as_deref()
    }
    /// <p>The description of the context.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>A list of properties to add to the context.</p>
    pub fn properties(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.properties.as_ref()
    }
    /// <p>A list of tags to apply to the context.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl CreateContextInput {
    /// Creates a new builder-style object to manufacture [`CreateContextInput`](crate::operation::create_context::CreateContextInput).
    pub fn builder() -> crate::operation::create_context::builders::CreateContextInputBuilder {
        crate::operation::create_context::builders::CreateContextInputBuilder::default()
    }
}

/// A builder for [`CreateContextInput`](crate::operation::create_context::CreateContextInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateContextInputBuilder {
    pub(crate) context_name: ::std::option::Option<::std::string::String>,
    pub(crate) source: ::std::option::Option<crate::types::ContextSource>,
    pub(crate) context_type: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) properties: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateContextInputBuilder {
    /// <p>The name of the context. Must be unique to your account in an Amazon Web Services Region.</p>
    pub fn context_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.context_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the context. Must be unique to your account in an Amazon Web Services Region.</p>
    pub fn set_context_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.context_name = input;
        self
    }
    /// <p>The source type, ID, and URI.</p>
    pub fn source(mut self, input: crate::types::ContextSource) -> Self {
        self.source = ::std::option::Option::Some(input);
        self
    }
    /// <p>The source type, ID, and URI.</p>
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::ContextSource>) -> Self {
        self.source = input;
        self
    }
    /// <p>The context type.</p>
    pub fn context_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.context_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The context type.</p>
    pub fn set_context_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.context_type = input;
        self
    }
    /// <p>The description of the context.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the context.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Adds a key-value pair to `properties`.
    ///
    /// To override the contents of this collection use [`set_properties`](Self::set_properties).
    ///
    /// <p>A list of properties to add to the context.</p>
    pub fn properties(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.properties.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.properties = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A list of properties to add to the context.</p>
    pub fn set_properties(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.properties = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags to apply to the context.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of tags to apply to the context.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateContextInput`](crate::operation::create_context::CreateContextInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_context::CreateContextInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_context::CreateContextInput {
            context_name: self.context_name,
            source: self.source,
            context_type: self.context_type,
            description: self.description,
            properties: self.properties,
            tags: self.tags,
        })
    }
}
