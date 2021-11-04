// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `InvokeEndpoint` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct InvokeEndpointError {
    /// Kind of error that occurred.
    pub kind: InvokeEndpointErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `InvokeEndpoint` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum InvokeEndpointErrorKind {
    /// <p> An internal failure occurred. </p>
    InternalFailure(crate::error::InternalFailure),
    /// <p> Model (owned by the customer in the container) returned 4xx or 5xx error code.
    /// </p>
    ModelError(crate::error::ModelError),
    /// <p> The service is unavailable. Try your call again. </p>
    ServiceUnavailable(crate::error::ServiceUnavailable),
    /// <p> Inspect your request and try again. </p>
    ValidationError(crate::error::ValidationError),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for InvokeEndpointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            InvokeEndpointErrorKind::InternalFailure(_inner) => _inner.fmt(f),
            InvokeEndpointErrorKind::ModelError(_inner) => _inner.fmt(f),
            InvokeEndpointErrorKind::ServiceUnavailable(_inner) => _inner.fmt(f),
            InvokeEndpointErrorKind::ValidationError(_inner) => _inner.fmt(f),
            InvokeEndpointErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for InvokeEndpointError {
    fn code(&self) -> Option<&str> {
        InvokeEndpointError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl InvokeEndpointError {
    /// Creates a new `InvokeEndpointError`.
    pub fn new(kind: InvokeEndpointErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `InvokeEndpointError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: InvokeEndpointErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `InvokeEndpointError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: InvokeEndpointErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `InvokeEndpointErrorKind::InternalFailure`.
    pub fn is_internal_failure(&self) -> bool {
        matches!(&self.kind, InvokeEndpointErrorKind::InternalFailure(_))
    }
    /// Returns `true` if the error kind is `InvokeEndpointErrorKind::ModelError`.
    pub fn is_model_error(&self) -> bool {
        matches!(&self.kind, InvokeEndpointErrorKind::ModelError(_))
    }
    /// Returns `true` if the error kind is `InvokeEndpointErrorKind::ServiceUnavailable`.
    pub fn is_service_unavailable(&self) -> bool {
        matches!(&self.kind, InvokeEndpointErrorKind::ServiceUnavailable(_))
    }
    /// Returns `true` if the error kind is `InvokeEndpointErrorKind::ValidationError`.
    pub fn is_validation_error(&self) -> bool {
        matches!(&self.kind, InvokeEndpointErrorKind::ValidationError(_))
    }
}
impl std::error::Error for InvokeEndpointError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            InvokeEndpointErrorKind::InternalFailure(_inner) => Some(_inner),
            InvokeEndpointErrorKind::ModelError(_inner) => Some(_inner),
            InvokeEndpointErrorKind::ServiceUnavailable(_inner) => Some(_inner),
            InvokeEndpointErrorKind::ValidationError(_inner) => Some(_inner),
            InvokeEndpointErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `InvokeEndpointAsync` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct InvokeEndpointAsyncError {
    /// Kind of error that occurred.
    pub kind: InvokeEndpointAsyncErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `InvokeEndpointAsync` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum InvokeEndpointAsyncErrorKind {
    /// <p> An internal failure occurred. </p>
    InternalFailure(crate::error::InternalFailure),
    /// <p> The service is unavailable. Try your call again. </p>
    ServiceUnavailable(crate::error::ServiceUnavailable),
    /// <p> Inspect your request and try again. </p>
    ValidationError(crate::error::ValidationError),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for InvokeEndpointAsyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            InvokeEndpointAsyncErrorKind::InternalFailure(_inner) => _inner.fmt(f),
            InvokeEndpointAsyncErrorKind::ServiceUnavailable(_inner) => _inner.fmt(f),
            InvokeEndpointAsyncErrorKind::ValidationError(_inner) => _inner.fmt(f),
            InvokeEndpointAsyncErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for InvokeEndpointAsyncError {
    fn code(&self) -> Option<&str> {
        InvokeEndpointAsyncError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl InvokeEndpointAsyncError {
    /// Creates a new `InvokeEndpointAsyncError`.
    pub fn new(kind: InvokeEndpointAsyncErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `InvokeEndpointAsyncError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: InvokeEndpointAsyncErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `InvokeEndpointAsyncError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: InvokeEndpointAsyncErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `InvokeEndpointAsyncErrorKind::InternalFailure`.
    pub fn is_internal_failure(&self) -> bool {
        matches!(&self.kind, InvokeEndpointAsyncErrorKind::InternalFailure(_))
    }
    /// Returns `true` if the error kind is `InvokeEndpointAsyncErrorKind::ServiceUnavailable`.
    pub fn is_service_unavailable(&self) -> bool {
        matches!(
            &self.kind,
            InvokeEndpointAsyncErrorKind::ServiceUnavailable(_)
        )
    }
    /// Returns `true` if the error kind is `InvokeEndpointAsyncErrorKind::ValidationError`.
    pub fn is_validation_error(&self) -> bool {
        matches!(&self.kind, InvokeEndpointAsyncErrorKind::ValidationError(_))
    }
}
impl std::error::Error for InvokeEndpointAsyncError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            InvokeEndpointAsyncErrorKind::InternalFailure(_inner) => Some(_inner),
            InvokeEndpointAsyncErrorKind::ServiceUnavailable(_inner) => Some(_inner),
            InvokeEndpointAsyncErrorKind::ValidationError(_inner) => Some(_inner),
            InvokeEndpointAsyncErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p> Inspect your request and try again. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ValidationError {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ValidationError");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ValidationError {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidationError")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ValidationError {}
/// See [`ValidationError`](crate::error::ValidationError)
pub mod validation_error {
    /// A builder for [`ValidationError`](crate::error::ValidationError)
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
        /// Consumes the builder and constructs a [`ValidationError`](crate::error::ValidationError)
        pub fn build(self) -> crate::error::ValidationError {
            crate::error::ValidationError {
                message: self.message,
            }
        }
    }
}
impl ValidationError {
    /// Creates a new builder-style object to manufacture [`ValidationError`](crate::error::ValidationError)
    pub fn builder() -> crate::error::validation_error::Builder {
        crate::error::validation_error::Builder::default()
    }
}

/// <p> The service is unavailable. Try your call again. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ServiceUnavailable {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ServiceUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ServiceUnavailable");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ServiceUnavailable {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ServiceUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceUnavailable")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for ServiceUnavailable {}
/// See [`ServiceUnavailable`](crate::error::ServiceUnavailable)
pub mod service_unavailable {
    /// A builder for [`ServiceUnavailable`](crate::error::ServiceUnavailable)
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
        /// Consumes the builder and constructs a [`ServiceUnavailable`](crate::error::ServiceUnavailable)
        pub fn build(self) -> crate::error::ServiceUnavailable {
            crate::error::ServiceUnavailable {
                message: self.message,
            }
        }
    }
}
impl ServiceUnavailable {
    /// Creates a new builder-style object to manufacture [`ServiceUnavailable`](crate::error::ServiceUnavailable)
    pub fn builder() -> crate::error::service_unavailable::Builder {
        crate::error::service_unavailable::Builder::default()
    }
}

/// <p> An internal failure occurred. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalFailure {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalFailure");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalFailure {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalFailure")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalFailure {}
/// See [`InternalFailure`](crate::error::InternalFailure)
pub mod internal_failure {
    /// A builder for [`InternalFailure`](crate::error::InternalFailure)
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
        /// Consumes the builder and constructs a [`InternalFailure`](crate::error::InternalFailure)
        pub fn build(self) -> crate::error::InternalFailure {
            crate::error::InternalFailure {
                message: self.message,
            }
        }
    }
}
impl InternalFailure {
    /// Creates a new builder-style object to manufacture [`InternalFailure`](crate::error::InternalFailure)
    pub fn builder() -> crate::error::internal_failure::Builder {
        crate::error::internal_failure::Builder::default()
    }
}

/// <p> Model (owned by the customer in the container) returned 4xx or 5xx error code.
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ModelError {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
    /// <p> Original status code. </p>
    pub original_status_code: std::option::Option<i32>,
    /// <p> Original message. </p>
    pub original_message: std::option::Option<std::string::String>,
    /// <p> The Amazon Resource Name (ARN) of the log stream. </p>
    pub log_stream_arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ModelError");
        formatter.field("message", &self.message);
        formatter.field("original_status_code", &self.original_status_code);
        formatter.field("original_message", &self.original_message);
        formatter.field("log_stream_arn", &self.log_stream_arn);
        formatter.finish()
    }
}
impl ModelError {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModelError")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for ModelError {}
/// See [`ModelError`](crate::error::ModelError)
pub mod model_error {
    /// A builder for [`ModelError`](crate::error::ModelError)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        pub(crate) original_status_code: std::option::Option<i32>,
        pub(crate) original_message: std::option::Option<std::string::String>,
        pub(crate) log_stream_arn: std::option::Option<std::string::String>,
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
        /// <p> Original status code. </p>
        pub fn original_status_code(mut self, input: i32) -> Self {
            self.original_status_code = Some(input);
            self
        }
        /// <p> Original status code. </p>
        pub fn set_original_status_code(mut self, input: std::option::Option<i32>) -> Self {
            self.original_status_code = input;
            self
        }
        /// <p> Original message. </p>
        pub fn original_message(mut self, input: impl Into<std::string::String>) -> Self {
            self.original_message = Some(input.into());
            self
        }
        /// <p> Original message. </p>
        pub fn set_original_message(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.original_message = input;
            self
        }
        /// <p> The Amazon Resource Name (ARN) of the log stream. </p>
        pub fn log_stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.log_stream_arn = Some(input.into());
            self
        }
        /// <p> The Amazon Resource Name (ARN) of the log stream. </p>
        pub fn set_log_stream_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.log_stream_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`ModelError`](crate::error::ModelError)
        pub fn build(self) -> crate::error::ModelError {
            crate::error::ModelError {
                message: self.message,
                original_status_code: self.original_status_code,
                original_message: self.original_message,
                log_stream_arn: self.log_stream_arn,
            }
        }
    }
}
impl ModelError {
    /// Creates a new builder-style object to manufacture [`ModelError`](crate::error::ModelError)
    pub fn builder() -> crate::error::model_error::Builder {
        crate::error::model_error::Builder::default()
    }
}
