// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>You can't delete a backup while it's being used to restore a file system.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BackupRestoring {
    /// <p>A detailed error message.</p>
    #[doc(hidden)]
    pub message: ::std::option::Option<::std::string::String>,
    /// <p>The ID of a file system being restored from the backup.</p>
    #[doc(hidden)]
    pub file_system_id: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl BackupRestoring {
    /// <p>The ID of a file system being restored from the backup.</p>
    pub fn file_system_id(&self) -> ::std::option::Option<&str> {
        self.file_system_id.as_deref()
    }
}
impl BackupRestoring {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for BackupRestoring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "BackupRestoring")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for BackupRestoring {}
impl ::aws_http::request_id::RequestId for crate::types::error::BackupRestoring {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for BackupRestoring {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl BackupRestoring {
    /// Creates a new builder-style object to manufacture [`BackupRestoring`](crate::types::error::BackupRestoring).
    pub fn builder() -> crate::types::error::builders::BackupRestoringBuilder {
        crate::types::error::builders::BackupRestoringBuilder::default()
    }
}

/// A builder for [`BackupRestoring`](crate::types::error::BackupRestoring).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BackupRestoringBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    pub(crate) file_system_id: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl BackupRestoringBuilder {
    /// <p>A detailed error message.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A detailed error message.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>The ID of a file system being restored from the backup.</p>
    pub fn file_system_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.file_system_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of a file system being restored from the backup.</p>
    pub fn set_file_system_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.file_system_id = input;
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
    /// Consumes the builder and constructs a [`BackupRestoring`](crate::types::error::BackupRestoring).
    pub fn build(self) -> crate::types::error::BackupRestoring {
        crate::types::error::BackupRestoring {
            message: self.message,
            file_system_id: self.file_system_id,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
