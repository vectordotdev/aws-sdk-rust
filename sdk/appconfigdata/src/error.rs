// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request was denied due to request throttling.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ThrottlingException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ThrottlingException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ThrottlingException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThrottlingException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ThrottlingException {}
/// See [`ThrottlingException`](crate::error::ThrottlingException).
pub mod throttling_exception {

    /// A builder for [`ThrottlingException`](crate::error::ThrottlingException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ThrottlingException`](crate::error::ThrottlingException).
        pub fn build(self) -> crate::error::ThrottlingException {
            crate::error::ThrottlingException {
                message: self.message,
            }
        }
    }
}
impl ThrottlingException {
    /// Creates a new builder-style object to manufacture [`ThrottlingException`](crate::error::ThrottlingException).
    pub fn builder() -> crate::error::throttling_exception::Builder {
        crate::error::throttling_exception::Builder::default()
    }
}

/// <p>The requested resource could not be found.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceNotFoundException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    /// <p>The type of resource that was not found.</p>
    #[doc(hidden)]
    pub resource_type: std::option::Option<crate::model::ResourceType>,
    /// <p>A map indicating which parameters in the request reference the resource that was not found.</p>
    #[doc(hidden)]
    pub referenced_by:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl ResourceNotFoundException {
    /// <p>The type of resource that was not found.</p>
    pub fn resource_type(&self) -> std::option::Option<&crate::model::ResourceType> {
        self.resource_type.as_ref()
    }
    /// <p>A map indicating which parameters in the request reference the resource that was not found.</p>
    pub fn referenced_by(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.referenced_by.as_ref()
    }
}
impl std::fmt::Debug for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceNotFoundException");
        formatter.field("message", &self.message);
        formatter.field("resource_type", &self.resource_type);
        formatter.field("referenced_by", &self.referenced_by);
        formatter.finish()
    }
}
impl ResourceNotFoundException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_2) = &self.message {
            {
                write!(f, ": {}", inner_2)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFoundException {}
/// See [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
pub mod resource_not_found_exception {

    /// A builder for [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        pub(crate) resource_type: std::option::Option<crate::model::ResourceType>,
        pub(crate) referenced_by: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// <p>The type of resource that was not found.</p>
        pub fn resource_type(mut self, input: crate::model::ResourceType) -> Self {
            self.resource_type = Some(input);
            self
        }
        /// <p>The type of resource that was not found.</p>
        pub fn set_resource_type(
            mut self,
            input: std::option::Option<crate::model::ResourceType>,
        ) -> Self {
            self.resource_type = input;
            self
        }
        /// Adds a key-value pair to `referenced_by`.
        ///
        /// To override the contents of this collection use [`set_referenced_by`](Self::set_referenced_by).
        ///
        /// <p>A map indicating which parameters in the request reference the resource that was not found.</p>
        pub fn referenced_by(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.referenced_by.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.referenced_by = Some(hash_map);
            self
        }
        /// <p>A map indicating which parameters in the request reference the resource that was not found.</p>
        pub fn set_referenced_by(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.referenced_by = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message,
                resource_type: self.resource_type,
                referenced_by: self.referenced_by,
            }
        }
    }
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
    pub fn builder() -> crate::error::resource_not_found_exception::Builder {
        crate::error::resource_not_found_exception::Builder::default()
    }
}

/// <p>There was an internal failure in the service.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalServerException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalServerException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalServerException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServerException")?;
        if let Some(inner_3) = &self.message {
            {
                write!(f, ": {}", inner_3)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InternalServerException {}
/// See [`InternalServerException`](crate::error::InternalServerException).
pub mod internal_server_exception {

    /// A builder for [`InternalServerException`](crate::error::InternalServerException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InternalServerException`](crate::error::InternalServerException).
        pub fn build(self) -> crate::error::InternalServerException {
            crate::error::InternalServerException {
                message: self.message,
            }
        }
    }
}
impl InternalServerException {
    /// Creates a new builder-style object to manufacture [`InternalServerException`](crate::error::InternalServerException).
    pub fn builder() -> crate::error::internal_server_exception::Builder {
        crate::error::internal_server_exception::Builder::default()
    }
}

/// <p>The input fails to satisfy the constraints specified by the service.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct BadRequestException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    /// <p>Code indicating the reason the request was invalid.</p>
    #[doc(hidden)]
    pub reason: std::option::Option<crate::model::BadRequestReason>,
    /// <p>Details describing why the request was invalid.</p>
    #[doc(hidden)]
    pub details: std::option::Option<crate::model::BadRequestDetails>,
}
impl BadRequestException {
    /// <p>Code indicating the reason the request was invalid.</p>
    pub fn reason(&self) -> std::option::Option<&crate::model::BadRequestReason> {
        self.reason.as_ref()
    }
    /// <p>Details describing why the request was invalid.</p>
    pub fn details(&self) -> std::option::Option<&crate::model::BadRequestDetails> {
        self.details.as_ref()
    }
}
impl std::fmt::Debug for BadRequestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BadRequestException");
        formatter.field("message", &self.message);
        formatter.field("reason", &self.reason);
        formatter.field("details", &self.details);
        formatter.finish()
    }
}
impl BadRequestException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for BadRequestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BadRequestException")?;
        if let Some(inner_4) = &self.message {
            {
                write!(f, ": {}", inner_4)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for BadRequestException {}
/// See [`BadRequestException`](crate::error::BadRequestException).
pub mod bad_request_exception {

    /// A builder for [`BadRequestException`](crate::error::BadRequestException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        pub(crate) reason: std::option::Option<crate::model::BadRequestReason>,
        pub(crate) details: std::option::Option<crate::model::BadRequestDetails>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// <p>Code indicating the reason the request was invalid.</p>
        pub fn reason(mut self, input: crate::model::BadRequestReason) -> Self {
            self.reason = Some(input);
            self
        }
        /// <p>Code indicating the reason the request was invalid.</p>
        pub fn set_reason(
            mut self,
            input: std::option::Option<crate::model::BadRequestReason>,
        ) -> Self {
            self.reason = input;
            self
        }
        /// <p>Details describing why the request was invalid.</p>
        pub fn details(mut self, input: crate::model::BadRequestDetails) -> Self {
            self.details = Some(input);
            self
        }
        /// <p>Details describing why the request was invalid.</p>
        pub fn set_details(
            mut self,
            input: std::option::Option<crate::model::BadRequestDetails>,
        ) -> Self {
            self.details = input;
            self
        }
        /// Consumes the builder and constructs a [`BadRequestException`](crate::error::BadRequestException).
        pub fn build(self) -> crate::error::BadRequestException {
            crate::error::BadRequestException {
                message: self.message,
                reason: self.reason,
                details: self.details,
            }
        }
    }
}
impl BadRequestException {
    /// Creates a new builder-style object to manufacture [`BadRequestException`](crate::error::BadRequestException).
    pub fn builder() -> crate::error::bad_request_exception::Builder {
        crate::error::bad_request_exception::Builder::default()
    }
}

/// Error type for the `GetLatestConfiguration` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetLatestConfigurationError {
    /// Kind of error that occurred.
    pub kind: GetLatestConfigurationErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for GetLatestConfigurationError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: GetLatestConfigurationErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `GetLatestConfiguration` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetLatestConfigurationErrorKind {
    /// <p>The input fails to satisfy the constraints specified by the service.</p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>There was an internal failure in the service.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for GetLatestConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetLatestConfigurationErrorKind::BadRequestException(_inner) => _inner.fmt(f),
            GetLatestConfigurationErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            GetLatestConfigurationErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            GetLatestConfigurationErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            GetLatestConfigurationErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetLatestConfigurationError {
    fn code(&self) -> Option<&str> {
        GetLatestConfigurationError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetLatestConfigurationError {
    /// Creates a new `GetLatestConfigurationError`.
    pub fn new(kind: GetLatestConfigurationErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetLatestConfigurationError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetLatestConfigurationErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
            meta: Default::default(),
        }
    }

    /// Creates the `GetLatestConfigurationError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetLatestConfigurationErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `GetLatestConfigurationErrorKind::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetLatestConfigurationErrorKind::BadRequestException(_)
        )
    }
    /// Returns `true` if the error kind is `GetLatestConfigurationErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetLatestConfigurationErrorKind::InternalServerException(_)
        )
    }
    /// Returns `true` if the error kind is `GetLatestConfigurationErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetLatestConfigurationErrorKind::ResourceNotFoundException(_)
        )
    }
    /// Returns `true` if the error kind is `GetLatestConfigurationErrorKind::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetLatestConfigurationErrorKind::ThrottlingException(_)
        )
    }
}
impl std::error::Error for GetLatestConfigurationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetLatestConfigurationErrorKind::BadRequestException(_inner) => Some(_inner),
            GetLatestConfigurationErrorKind::InternalServerException(_inner) => Some(_inner),
            GetLatestConfigurationErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            GetLatestConfigurationErrorKind::ThrottlingException(_inner) => Some(_inner),
            GetLatestConfigurationErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `StartConfigurationSession` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct StartConfigurationSessionError {
    /// Kind of error that occurred.
    pub kind: StartConfigurationSessionErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for StartConfigurationSessionError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: StartConfigurationSessionErrorKind::Unhandled(crate::error::Unhandled::new(
                source,
            )),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `StartConfigurationSession` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum StartConfigurationSessionErrorKind {
    /// <p>The input fails to satisfy the constraints specified by the service.</p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>There was an internal failure in the service.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The requested resource could not be found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for StartConfigurationSessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StartConfigurationSessionErrorKind::BadRequestException(_inner) => _inner.fmt(f),
            StartConfigurationSessionErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            StartConfigurationSessionErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            StartConfigurationSessionErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            StartConfigurationSessionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for StartConfigurationSessionError {
    fn code(&self) -> Option<&str> {
        StartConfigurationSessionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl StartConfigurationSessionError {
    /// Creates a new `StartConfigurationSessionError`.
    pub fn new(kind: StartConfigurationSessionErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `StartConfigurationSessionError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: StartConfigurationSessionErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
            meta: Default::default(),
        }
    }

    /// Creates the `StartConfigurationSessionError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: StartConfigurationSessionErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `StartConfigurationSessionErrorKind::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartConfigurationSessionErrorKind::BadRequestException(_)
        )
    }
    /// Returns `true` if the error kind is `StartConfigurationSessionErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartConfigurationSessionErrorKind::InternalServerException(_)
        )
    }
    /// Returns `true` if the error kind is `StartConfigurationSessionErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartConfigurationSessionErrorKind::ResourceNotFoundException(_)
        )
    }
    /// Returns `true` if the error kind is `StartConfigurationSessionErrorKind::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            StartConfigurationSessionErrorKind::ThrottlingException(_)
        )
    }
}
impl std::error::Error for StartConfigurationSessionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            StartConfigurationSessionErrorKind::BadRequestException(_inner) => Some(_inner),
            StartConfigurationSessionErrorKind::InternalServerException(_inner) => Some(_inner),
            StartConfigurationSessionErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            StartConfigurationSessionErrorKind::ThrottlingException(_inner) => Some(_inner),
            StartConfigurationSessionErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

///
/// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
///
/// When logging an error from the SDK, it is recommended that you either wrap the error in
/// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
/// error reporter library that visits the error's cause/source chain, or call
/// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
///
#[derive(Debug)]
pub struct Unhandled {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
}
impl Unhandled {
    pub(crate) fn new(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self { source }
    }
}
impl std::fmt::Display for Unhandled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "unhandled error")
    }
}
impl std::error::Error for Unhandled {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref() as _)
    }
}
