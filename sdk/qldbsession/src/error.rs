// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Returned when the rate of requests exceeds the allowed throughput.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RateExceededException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for RateExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RateExceededException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl RateExceededException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for RateExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RateExceededException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for RateExceededException {}
/// See [`RateExceededException`](crate::error::RateExceededException).
pub mod rate_exceeded_exception {

    /// A builder for [`RateExceededException`](crate::error::RateExceededException).
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
        /// Consumes the builder and constructs a [`RateExceededException`](crate::error::RateExceededException).
        pub fn build(self) -> crate::error::RateExceededException {
            crate::error::RateExceededException {
                message: self.message,
            }
        }
    }
}
impl RateExceededException {
    /// Creates a new builder-style object to manufacture [`RateExceededException`](crate::error::RateExceededException).
    pub fn builder() -> crate::error::rate_exceeded_exception::Builder {
        crate::error::rate_exceeded_exception::Builder::default()
    }
}

/// <p>Returned when a transaction cannot be written to the journal due to a failure in the verification phase of <i>optimistic concurrency control</i> (OCC).</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct OccConflictException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for OccConflictException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("OccConflictException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl OccConflictException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for OccConflictException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OccConflictException")?;
        if let Some(inner_2) = &self.message {
            {
                write!(f, ": {}", inner_2)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for OccConflictException {}
/// See [`OccConflictException`](crate::error::OccConflictException).
pub mod occ_conflict_exception {

    /// A builder for [`OccConflictException`](crate::error::OccConflictException).
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
        /// Consumes the builder and constructs a [`OccConflictException`](crate::error::OccConflictException).
        pub fn build(self) -> crate::error::OccConflictException {
            crate::error::OccConflictException {
                message: self.message,
            }
        }
    }
}
impl OccConflictException {
    /// Creates a new builder-style object to manufacture [`OccConflictException`](crate::error::OccConflictException).
    pub fn builder() -> crate::error::occ_conflict_exception::Builder {
        crate::error::occ_conflict_exception::Builder::default()
    }
}

/// <p>Returned if a resource limit such as number of active sessions is exceeded.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct LimitExceededException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for LimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("LimitExceededException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl LimitExceededException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for LimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LimitExceededException")?;
        if let Some(inner_3) = &self.message {
            {
                write!(f, ": {}", inner_3)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for LimitExceededException {}
/// See [`LimitExceededException`](crate::error::LimitExceededException).
pub mod limit_exceeded_exception {

    /// A builder for [`LimitExceededException`](crate::error::LimitExceededException).
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
        /// Consumes the builder and constructs a [`LimitExceededException`](crate::error::LimitExceededException).
        pub fn build(self) -> crate::error::LimitExceededException {
            crate::error::LimitExceededException {
                message: self.message,
            }
        }
    }
}
impl LimitExceededException {
    /// Creates a new builder-style object to manufacture [`LimitExceededException`](crate::error::LimitExceededException).
    pub fn builder() -> crate::error::limit_exceeded_exception::Builder {
        crate::error::limit_exceeded_exception::Builder::default()
    }
}

/// <p>Returned if the session doesn't exist anymore because it timed out or expired.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidSessionException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub code: std::option::Option<std::string::String>,
}
impl InvalidSessionException {
    #[allow(missing_docs)] // documentation missing in model
    pub fn code(&self) -> std::option::Option<&str> {
        self.code.as_deref()
    }
}
impl std::fmt::Debug for InvalidSessionException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidSessionException");
        formatter.field("message", &self.message);
        formatter.field("code", &self.code);
        formatter.finish()
    }
}
impl InvalidSessionException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidSessionException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidSessionException")?;
        if let Some(inner_4) = &self.message {
            {
                write!(f, ": {}", inner_4)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InvalidSessionException {}
/// See [`InvalidSessionException`](crate::error::InvalidSessionException).
pub mod invalid_session_exception {

    /// A builder for [`InvalidSessionException`](crate::error::InvalidSessionException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        pub(crate) code: std::option::Option<std::string::String>,
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
        #[allow(missing_docs)] // documentation missing in model
        pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
            self.code = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.code = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidSessionException`](crate::error::InvalidSessionException).
        pub fn build(self) -> crate::error::InvalidSessionException {
            crate::error::InvalidSessionException {
                message: self.message,
                code: self.code,
            }
        }
    }
}
impl InvalidSessionException {
    /// Creates a new builder-style object to manufacture [`InvalidSessionException`](crate::error::InvalidSessionException).
    pub fn builder() -> crate::error::invalid_session_exception::Builder {
        crate::error::invalid_session_exception::Builder::default()
    }
}

/// <p>Returned when the request exceeds the processing capacity of the ledger.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CapacityExceededException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for CapacityExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CapacityExceededException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl CapacityExceededException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for CapacityExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CapacityExceededException")?;
        if let Some(inner_5) = &self.message {
            {
                write!(f, ": {}", inner_5)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for CapacityExceededException {}
/// See [`CapacityExceededException`](crate::error::CapacityExceededException).
pub mod capacity_exceeded_exception {

    /// A builder for [`CapacityExceededException`](crate::error::CapacityExceededException).
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
        /// Consumes the builder and constructs a [`CapacityExceededException`](crate::error::CapacityExceededException).
        pub fn build(self) -> crate::error::CapacityExceededException {
            crate::error::CapacityExceededException {
                message: self.message,
            }
        }
    }
}
impl CapacityExceededException {
    /// Creates a new builder-style object to manufacture [`CapacityExceededException`](crate::error::CapacityExceededException).
    pub fn builder() -> crate::error::capacity_exceeded_exception::Builder {
        crate::error::capacity_exceeded_exception::Builder::default()
    }
}

/// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct BadRequestException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub code: std::option::Option<std::string::String>,
}
impl BadRequestException {
    #[allow(missing_docs)] // documentation missing in model
    pub fn code(&self) -> std::option::Option<&str> {
        self.code.as_deref()
    }
}
impl std::fmt::Debug for BadRequestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BadRequestException");
        formatter.field("message", &self.message);
        formatter.field("code", &self.code);
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
        if let Some(inner_6) = &self.message {
            {
                write!(f, ": {}", inner_6)?;
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
        pub(crate) code: std::option::Option<std::string::String>,
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
        #[allow(missing_docs)] // documentation missing in model
        pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
            self.code = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.code = input;
            self
        }
        /// Consumes the builder and constructs a [`BadRequestException`](crate::error::BadRequestException).
        pub fn build(self) -> crate::error::BadRequestException {
            crate::error::BadRequestException {
                message: self.message,
                code: self.code,
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

/// Error type for the `SendCommand` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct SendCommandError {
    /// Kind of error that occurred.
    pub kind: SendCommandErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for SendCommandError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: SendCommandErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `SendCommand` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum SendCommandErrorKind {
    /// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>Returned when the request exceeds the processing capacity of the ledger.</p>
    CapacityExceededException(crate::error::CapacityExceededException),
    /// <p>Returned if the session doesn't exist anymore because it timed out or expired.</p>
    InvalidSessionException(crate::error::InvalidSessionException),
    /// <p>Returned if a resource limit such as number of active sessions is exceeded.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>Returned when a transaction cannot be written to the journal due to a failure in the verification phase of <i>optimistic concurrency control</i> (OCC).</p>
    OccConflictException(crate::error::OccConflictException),
    /// <p>Returned when the rate of requests exceeds the allowed throughput.</p>
    RateExceededException(crate::error::RateExceededException),
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
impl std::fmt::Display for SendCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            SendCommandErrorKind::BadRequestException(_inner) => _inner.fmt(f),
            SendCommandErrorKind::CapacityExceededException(_inner) => _inner.fmt(f),
            SendCommandErrorKind::InvalidSessionException(_inner) => _inner.fmt(f),
            SendCommandErrorKind::LimitExceededException(_inner) => _inner.fmt(f),
            SendCommandErrorKind::OccConflictException(_inner) => _inner.fmt(f),
            SendCommandErrorKind::RateExceededException(_inner) => _inner.fmt(f),
            SendCommandErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for SendCommandError {
    fn code(&self) -> Option<&str> {
        SendCommandError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl SendCommandError {
    /// Creates a new `SendCommandError`.
    pub fn new(kind: SendCommandErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `SendCommandError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: SendCommandErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `SendCommandError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: SendCommandErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
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
    /// Returns `true` if the error kind is `SendCommandErrorKind::BadRequestException`.
    pub fn is_bad_request_exception(&self) -> bool {
        matches!(&self.kind, SendCommandErrorKind::BadRequestException(_))
    }
    /// Returns `true` if the error kind is `SendCommandErrorKind::CapacityExceededException`.
    pub fn is_capacity_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            SendCommandErrorKind::CapacityExceededException(_)
        )
    }
    /// Returns `true` if the error kind is `SendCommandErrorKind::InvalidSessionException`.
    pub fn is_invalid_session_exception(&self) -> bool {
        matches!(&self.kind, SendCommandErrorKind::InvalidSessionException(_))
    }
    /// Returns `true` if the error kind is `SendCommandErrorKind::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(&self.kind, SendCommandErrorKind::LimitExceededException(_))
    }
    /// Returns `true` if the error kind is `SendCommandErrorKind::OccConflictException`.
    pub fn is_occ_conflict_exception(&self) -> bool {
        matches!(&self.kind, SendCommandErrorKind::OccConflictException(_))
    }
    /// Returns `true` if the error kind is `SendCommandErrorKind::RateExceededException`.
    pub fn is_rate_exceeded_exception(&self) -> bool {
        matches!(&self.kind, SendCommandErrorKind::RateExceededException(_))
    }
}
impl std::error::Error for SendCommandError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            SendCommandErrorKind::BadRequestException(_inner) => Some(_inner),
            SendCommandErrorKind::CapacityExceededException(_inner) => Some(_inner),
            SendCommandErrorKind::InvalidSessionException(_inner) => Some(_inner),
            SendCommandErrorKind::LimitExceededException(_inner) => Some(_inner),
            SendCommandErrorKind::OccConflictException(_inner) => Some(_inner),
            SendCommandErrorKind::RateExceededException(_inner) => Some(_inner),
            SendCommandErrorKind::Unhandled(_inner) => Some(_inner),
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
