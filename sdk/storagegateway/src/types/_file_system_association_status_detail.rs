// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Detailed information on file system association status.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FileSystemAssociationStatusDetail {
    /// <p>The error code for a given file system association status.</p>
    #[doc(hidden)]
    pub error_code: ::std::option::Option<::std::string::String>,
}
impl FileSystemAssociationStatusDetail {
    /// <p>The error code for a given file system association status.</p>
    pub fn error_code(&self) -> ::std::option::Option<&str> {
        self.error_code.as_deref()
    }
}
impl FileSystemAssociationStatusDetail {
    /// Creates a new builder-style object to manufacture [`FileSystemAssociationStatusDetail`](crate::types::FileSystemAssociationStatusDetail).
    pub fn builder() -> crate::types::builders::FileSystemAssociationStatusDetailBuilder {
        crate::types::builders::FileSystemAssociationStatusDetailBuilder::default()
    }
}

/// A builder for [`FileSystemAssociationStatusDetail`](crate::types::FileSystemAssociationStatusDetail).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FileSystemAssociationStatusDetailBuilder {
    pub(crate) error_code: ::std::option::Option<::std::string::String>,
}
impl FileSystemAssociationStatusDetailBuilder {
    /// <p>The error code for a given file system association status.</p>
    pub fn error_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.error_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The error code for a given file system association status.</p>
    pub fn set_error_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.error_code = input;
        self
    }
    /// Consumes the builder and constructs a [`FileSystemAssociationStatusDetail`](crate::types::FileSystemAssociationStatusDetail).
    pub fn build(self) -> crate::types::FileSystemAssociationStatusDetail {
        crate::types::FileSystemAssociationStatusDetail {
            error_code: self.error_code,
        }
    }
}
