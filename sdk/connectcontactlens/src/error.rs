// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `ListRealtimeContactAnalysisSegments` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ListRealtimeContactAnalysisSegmentsError {
    /// Kind of error that occurred.
    pub kind: ListRealtimeContactAnalysisSegmentsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `ListRealtimeContactAnalysisSegments` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListRealtimeContactAnalysisSegmentsErrorKind {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalServiceException(crate::error::InternalServiceException),
    /// <p>The request is not valid.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>The specified resource was not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The throttling limit has been exceeded.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for ListRealtimeContactAnalysisSegmentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListRealtimeContactAnalysisSegmentsErrorKind::AccessDeniedException(_inner) => {
                _inner.fmt(f)
            }
            ListRealtimeContactAnalysisSegmentsErrorKind::InternalServiceException(_inner) => {
                _inner.fmt(f)
            }
            ListRealtimeContactAnalysisSegmentsErrorKind::InvalidRequestException(_inner) => {
                _inner.fmt(f)
            }
            ListRealtimeContactAnalysisSegmentsErrorKind::ResourceNotFoundException(_inner) => {
                _inner.fmt(f)
            }
            ListRealtimeContactAnalysisSegmentsErrorKind::ThrottlingException(_inner) => {
                _inner.fmt(f)
            }
            ListRealtimeContactAnalysisSegmentsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListRealtimeContactAnalysisSegmentsError {
    fn code(&self) -> Option<&str> {
        ListRealtimeContactAnalysisSegmentsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListRealtimeContactAnalysisSegmentsError {
    /// Creates a new `ListRealtimeContactAnalysisSegmentsError`.
    pub fn new(
        kind: ListRealtimeContactAnalysisSegmentsErrorKind,
        meta: aws_smithy_types::Error,
    ) -> Self {
        Self { kind, meta }
    }

    /// Creates the `ListRealtimeContactAnalysisSegmentsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: ListRealtimeContactAnalysisSegmentsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `ListRealtimeContactAnalysisSegmentsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: ListRealtimeContactAnalysisSegmentsErrorKind::Unhandled(err.into()),
        }
    }

    // TODO: Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display
    // as implemented by std::Error to generate a message in that case.
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
    /// Returns `true` if the error kind is `ListRealtimeContactAnalysisSegmentsErrorKind::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListRealtimeContactAnalysisSegmentsErrorKind::AccessDeniedException(_)
        )
    }
    /// Returns `true` if the error kind is `ListRealtimeContactAnalysisSegmentsErrorKind::InternalServiceException`.
    pub fn is_internal_service_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListRealtimeContactAnalysisSegmentsErrorKind::InternalServiceException(_)
        )
    }
    /// Returns `true` if the error kind is `ListRealtimeContactAnalysisSegmentsErrorKind::InvalidRequestException`.
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListRealtimeContactAnalysisSegmentsErrorKind::InvalidRequestException(_)
        )
    }
    /// Returns `true` if the error kind is `ListRealtimeContactAnalysisSegmentsErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListRealtimeContactAnalysisSegmentsErrorKind::ResourceNotFoundException(_)
        )
    }
    /// Returns `true` if the error kind is `ListRealtimeContactAnalysisSegmentsErrorKind::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListRealtimeContactAnalysisSegmentsErrorKind::ThrottlingException(_)
        )
    }
}
impl std::error::Error for ListRealtimeContactAnalysisSegmentsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListRealtimeContactAnalysisSegmentsErrorKind::AccessDeniedException(_inner) => {
                Some(_inner)
            }
            ListRealtimeContactAnalysisSegmentsErrorKind::InternalServiceException(_inner) => {
                Some(_inner)
            }
            ListRealtimeContactAnalysisSegmentsErrorKind::InvalidRequestException(_inner) => {
                Some(_inner)
            }
            ListRealtimeContactAnalysisSegmentsErrorKind::ResourceNotFoundException(_inner) => {
                Some(_inner)
            }
            ListRealtimeContactAnalysisSegmentsErrorKind::ThrottlingException(_inner) => {
                Some(_inner)
            }
            ListRealtimeContactAnalysisSegmentsErrorKind::Unhandled(_inner) => {
                Some(_inner.as_ref())
            }
        }
    }
}

/// <p>The throttling limit has been exceeded.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ThrottlingException {
    #[allow(missing_docs)] // documentation missing in model
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
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThrottlingException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ThrottlingException {}
/// See [`ThrottlingException`](crate::error::ThrottlingException)
pub mod throttling_exception {
    /// A builder for [`ThrottlingException`](crate::error::ThrottlingException)
    #[non_exhaustive]
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
        /// Consumes the builder and constructs a [`ThrottlingException`](crate::error::ThrottlingException)
        pub fn build(self) -> crate::error::ThrottlingException {
            crate::error::ThrottlingException {
                message: self.message,
            }
        }
    }
}
impl ThrottlingException {
    /// Creates a new builder-style object to manufacture [`ThrottlingException`](crate::error::ThrottlingException)
    pub fn builder() -> crate::error::throttling_exception::Builder {
        crate::error::throttling_exception::Builder::default()
    }
}

/// <p>The specified resource was not found.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceNotFoundException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceNotFoundException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ResourceNotFoundException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFoundException {}
/// See [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
pub mod resource_not_found_exception {
    /// A builder for [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
    #[non_exhaustive]
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
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message,
            }
        }
    }
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
    pub fn builder() -> crate::error::resource_not_found_exception::Builder {
        crate::error::resource_not_found_exception::Builder::default()
    }
}

/// <p>The request is not valid.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidRequestException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidRequestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidRequestException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InvalidRequestException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidRequestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidRequestException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidRequestException {}
/// See [`InvalidRequestException`](crate::error::InvalidRequestException)
pub mod invalid_request_exception {
    /// A builder for [`InvalidRequestException`](crate::error::InvalidRequestException)
    #[non_exhaustive]
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
        /// Consumes the builder and constructs a [`InvalidRequestException`](crate::error::InvalidRequestException)
        pub fn build(self) -> crate::error::InvalidRequestException {
            crate::error::InvalidRequestException {
                message: self.message,
            }
        }
    }
}
impl InvalidRequestException {
    /// Creates a new builder-style object to manufacture [`InvalidRequestException`](crate::error::InvalidRequestException)
    pub fn builder() -> crate::error::invalid_request_exception::Builder {
        crate::error::invalid_request_exception::Builder::default()
    }
}

/// <p>Request processing failed due to an error or failure with the service.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalServiceException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalServiceException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalServiceException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalServiceException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalServiceException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServiceException")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalServiceException {}
/// See [`InternalServiceException`](crate::error::InternalServiceException)
pub mod internal_service_exception {
    /// A builder for [`InternalServiceException`](crate::error::InternalServiceException)
    #[non_exhaustive]
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
        /// Consumes the builder and constructs a [`InternalServiceException`](crate::error::InternalServiceException)
        pub fn build(self) -> crate::error::InternalServiceException {
            crate::error::InternalServiceException {
                message: self.message,
            }
        }
    }
}
impl InternalServiceException {
    /// Creates a new builder-style object to manufacture [`InternalServiceException`](crate::error::InternalServiceException)
    pub fn builder() -> crate::error::internal_service_exception::Builder {
        crate::error::internal_service_exception::Builder::default()
    }
}

/// <p>You do not have sufficient access to perform this action.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AccessDeniedException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for AccessDeniedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AccessDeniedException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl AccessDeniedException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for AccessDeniedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccessDeniedException")?;
        if let Some(inner_5) = &self.message {
            write!(f, ": {}", inner_5)?;
        }
        Ok(())
    }
}
impl std::error::Error for AccessDeniedException {}
/// See [`AccessDeniedException`](crate::error::AccessDeniedException)
pub mod access_denied_exception {
    /// A builder for [`AccessDeniedException`](crate::error::AccessDeniedException)
    #[non_exhaustive]
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
        /// Consumes the builder and constructs a [`AccessDeniedException`](crate::error::AccessDeniedException)
        pub fn build(self) -> crate::error::AccessDeniedException {
            crate::error::AccessDeniedException {
                message: self.message,
            }
        }
    }
}
impl AccessDeniedException {
    /// Creates a new builder-style object to manufacture [`AccessDeniedException`](crate::error::AccessDeniedException)
    pub fn builder() -> crate::error::access_denied_exception::Builder {
        crate::error::access_denied_exception::Builder::default()
    }
}
