// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Updating or deleting a resource causes an inconsistent state.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ConflictException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: ::std::option::Option<::std::string::String>,
    /// Identifier of the resource in use
    #[doc(hidden)]
    pub resource_id: ::std::option::Option<::std::string::String>,
    /// Type of the resource in use
    #[doc(hidden)]
    pub resource_type: ::std::option::Option<::std::string::String>,
    /// List of dependent entities containing information on relation type and resourceArns linked to the resource in use
    #[doc(hidden)]
    pub dependent_entities: ::std::option::Option<::std::vec::Vec<crate::types::DependentEntity>>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl ConflictException {
    /// Identifier of the resource in use
    pub fn resource_id(&self) -> ::std::option::Option<&str> {
        self.resource_id.as_deref()
    }
    /// Type of the resource in use
    pub fn resource_type(&self) -> ::std::option::Option<&str> {
        self.resource_type.as_deref()
    }
    /// List of dependent entities containing information on relation type and resourceArns linked to the resource in use
    pub fn dependent_entities(&self) -> ::std::option::Option<&[crate::types::DependentEntity]> {
        self.dependent_entities.as_deref()
    }
}
impl ConflictException {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for ConflictException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "ConflictException")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for ConflictException {}
impl ::aws_http::request_id::RequestId for crate::types::error::ConflictException {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ConflictException {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl ConflictException {
    /// Creates a new builder-style object to manufacture [`ConflictException`](crate::types::error::ConflictException).
    pub fn builder() -> crate::types::error::builders::ConflictExceptionBuilder {
        crate::types::error::builders::ConflictExceptionBuilder::default()
    }
}

/// A builder for [`ConflictException`](crate::types::error::ConflictException).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ConflictExceptionBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    pub(crate) resource_id: ::std::option::Option<::std::string::String>,
    pub(crate) resource_type: ::std::option::Option<::std::string::String>,
    pub(crate) dependent_entities:
        ::std::option::Option<::std::vec::Vec<crate::types::DependentEntity>>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl ConflictExceptionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Identifier of the resource in use
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_id = ::std::option::Option::Some(input.into());
        self
    }
    /// Identifier of the resource in use
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_id = input;
        self
    }
    /// Type of the resource in use
    pub fn resource_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.resource_type = ::std::option::Option::Some(input.into());
        self
    }
    /// Type of the resource in use
    pub fn set_resource_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.resource_type = input;
        self
    }
    /// Appends an item to `dependent_entities`.
    ///
    /// To override the contents of this collection use [`set_dependent_entities`](Self::set_dependent_entities).
    ///
    /// List of dependent entities containing information on relation type and resourceArns linked to the resource in use
    pub fn dependent_entities(mut self, input: crate::types::DependentEntity) -> Self {
        let mut v = self.dependent_entities.unwrap_or_default();
        v.push(input);
        self.dependent_entities = ::std::option::Option::Some(v);
        self
    }
    /// List of dependent entities containing information on relation type and resourceArns linked to the resource in use
    pub fn set_dependent_entities(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DependentEntity>>,
    ) -> Self {
        self.dependent_entities = input;
        self
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(
        &mut self,
        meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`ConflictException`](crate::types::error::ConflictException).
    pub fn build(self) -> crate::types::error::ConflictException {
        crate::types::error::ConflictException {
            message: self.message,
            resource_id: self.resource_id,
            resource_type: self.resource_type,
            dependent_entities: self.dependent_entities,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
