// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about a parameter object.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ParameterObject {
    /// <p>The ID of the parameter object. </p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The attributes of the parameter object.</p>
    #[doc(hidden)]
    pub attributes: ::std::option::Option<::std::vec::Vec<crate::types::ParameterAttribute>>,
}
impl ParameterObject {
    /// <p>The ID of the parameter object. </p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The attributes of the parameter object.</p>
    pub fn attributes(&self) -> ::std::option::Option<&[crate::types::ParameterAttribute]> {
        self.attributes.as_deref()
    }
}
impl ParameterObject {
    /// Creates a new builder-style object to manufacture [`ParameterObject`](crate::types::ParameterObject).
    pub fn builder() -> crate::types::builders::ParameterObjectBuilder {
        crate::types::builders::ParameterObjectBuilder::default()
    }
}

/// A builder for [`ParameterObject`](crate::types::ParameterObject).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ParameterObjectBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) attributes: ::std::option::Option<::std::vec::Vec<crate::types::ParameterAttribute>>,
}
impl ParameterObjectBuilder {
    /// <p>The ID of the parameter object. </p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the parameter object. </p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Appends an item to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>The attributes of the parameter object.</p>
    pub fn attributes(mut self, input: crate::types::ParameterAttribute) -> Self {
        let mut v = self.attributes.unwrap_or_default();
        v.push(input);
        self.attributes = ::std::option::Option::Some(v);
        self
    }
    /// <p>The attributes of the parameter object.</p>
    pub fn set_attributes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ParameterAttribute>>,
    ) -> Self {
        self.attributes = input;
        self
    }
    /// Consumes the builder and constructs a [`ParameterObject`](crate::types::ParameterObject).
    pub fn build(self) -> crate::types::ParameterObject {
        crate::types::ParameterObject {
            id: self.id,
            attributes: self.attributes,
        }
    }
}
