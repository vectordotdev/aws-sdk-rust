// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The operation failed because the calling identity doesn't have the minimum required permissions.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>The operation failed because of an error internal to Amazon Web Services. Try your operation again later.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The operation failed because it specified a resource that can't be found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The operation failed because it was called too frequently and exceeded a throttle limit.</p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// <p>The operation failed because one of the input parameters was invalid.</p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::TooManyRequestsException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteAlternateContactError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteAlternateContactError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DeleteAlternateContactErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::DeleteAlternateContactErrorKind::InternalServerException(
                        inner,
                    ) => Error::InternalServerException(inner),
                    crate::error::DeleteAlternateContactErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::DeleteAlternateContactErrorKind::TooManyRequestsException(
                        inner,
                    ) => Error::TooManyRequestsException(inner),
                    crate::error::DeleteAlternateContactErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DeleteAlternateContactErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetAlternateContactError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetAlternateContactError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetAlternateContactErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::GetAlternateContactErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::GetAlternateContactErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::GetAlternateContactErrorKind::TooManyRequestsException(inner) => {
                        Error::TooManyRequestsException(inner)
                    }
                    crate::error::GetAlternateContactErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::GetAlternateContactErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetContactInformationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetContactInformationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetContactInformationErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::GetContactInformationErrorKind::InternalServerException(
                        inner,
                    ) => Error::InternalServerException(inner),
                    crate::error::GetContactInformationErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::GetContactInformationErrorKind::TooManyRequestsException(
                        inner,
                    ) => Error::TooManyRequestsException(inner),
                    crate::error::GetContactInformationErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::GetContactInformationErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutAlternateContactError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::PutAlternateContactError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::PutAlternateContactErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::PutAlternateContactErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::PutAlternateContactErrorKind::TooManyRequestsException(inner) => {
                        Error::TooManyRequestsException(inner)
                    }
                    crate::error::PutAlternateContactErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::PutAlternateContactErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutContactInformationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::PutContactInformationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::PutContactInformationErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::PutContactInformationErrorKind::InternalServerException(
                        inner,
                    ) => Error::InternalServerException(inner),
                    crate::error::PutContactInformationErrorKind::TooManyRequestsException(
                        inner,
                    ) => Error::TooManyRequestsException(inner),
                    crate::error::PutContactInformationErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::PutContactInformationErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
