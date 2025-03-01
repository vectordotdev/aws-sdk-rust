// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A structure containing an LF-tag key and values for a resource.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LfTagKeyResource {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p>
    #[doc(hidden)]
    pub catalog_id: ::std::option::Option<::std::string::String>,
    /// <p>The key-name for the LF-tag.</p>
    #[doc(hidden)]
    pub tag_key: ::std::option::Option<::std::string::String>,
    /// <p>A list of possible values an attribute can take.</p>
    #[doc(hidden)]
    pub tag_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl LfTagKeyResource {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p>
    pub fn catalog_id(&self) -> ::std::option::Option<&str> {
        self.catalog_id.as_deref()
    }
    /// <p>The key-name for the LF-tag.</p>
    pub fn tag_key(&self) -> ::std::option::Option<&str> {
        self.tag_key.as_deref()
    }
    /// <p>A list of possible values an attribute can take.</p>
    pub fn tag_values(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.tag_values.as_deref()
    }
}
impl LfTagKeyResource {
    /// Creates a new builder-style object to manufacture [`LfTagKeyResource`](crate::types::LfTagKeyResource).
    pub fn builder() -> crate::types::builders::LfTagKeyResourceBuilder {
        crate::types::builders::LfTagKeyResourceBuilder::default()
    }
}

/// A builder for [`LfTagKeyResource`](crate::types::LfTagKeyResource).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LfTagKeyResourceBuilder {
    pub(crate) catalog_id: ::std::option::Option<::std::string::String>,
    pub(crate) tag_key: ::std::option::Option<::std::string::String>,
    pub(crate) tag_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl LfTagKeyResourceBuilder {
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p>
    pub fn catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.catalog_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p>
    pub fn set_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.catalog_id = input;
        self
    }
    /// <p>The key-name for the LF-tag.</p>
    pub fn tag_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.tag_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The key-name for the LF-tag.</p>
    pub fn set_tag_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.tag_key = input;
        self
    }
    /// Appends an item to `tag_values`.
    ///
    /// To override the contents of this collection use [`set_tag_values`](Self::set_tag_values).
    ///
    /// <p>A list of possible values an attribute can take.</p>
    pub fn tag_values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.tag_values.unwrap_or_default();
        v.push(input.into());
        self.tag_values = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of possible values an attribute can take.</p>
    pub fn set_tag_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.tag_values = input;
        self
    }
    /// Consumes the builder and constructs a [`LfTagKeyResource`](crate::types::LfTagKeyResource).
    pub fn build(self) -> crate::types::LfTagKeyResource {
        crate::types::LfTagKeyResource {
            catalog_id: self.catalog_id,
            tag_key: self.tag_key,
            tag_values: self.tag_values,
        }
    }
}
