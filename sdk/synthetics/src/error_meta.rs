// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The request was not valid.</p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>A conflicting operation is already in progress.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>An internal failure occurred. Try the operation again.</p>
    InternalFailureException(crate::error::InternalFailureException),
    /// <p>An unknown internal error occurred.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The specified resource was not found.</p>
    NotFoundException(crate::error::NotFoundException),
    /// <p>One of the input resources is larger than is allowed.</p>
    RequestEntityTooLargeException(crate::error::RequestEntityTooLargeException),
    /// <p>One of the specified resources was not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The request exceeded a service quota value.</p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p>There were too many simultaneous requests. Try the operation again.</p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// <p>A parameter could not be validated.</p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalFailureException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::RequestEntityTooLargeException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::TooManyRequestsException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AssociateResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AssociateResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::AssociateResourceErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::AssociateResourceErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::AssociateResourceErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::AssociateResourceErrorKind::ServiceQuotaExceededException(
                        inner,
                    ) => Error::ServiceQuotaExceededException(inner),
                    crate::error::AssociateResourceErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::AssociateResourceErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateCanaryError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateCanaryError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::CreateCanaryErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::CreateCanaryErrorKind::RequestEntityTooLargeException(inner) => {
                        Error::RequestEntityTooLargeException(inner)
                    }
                    crate::error::CreateCanaryErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::CreateCanaryErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateGroupError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateGroupError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::CreateGroupErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::CreateGroupErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::CreateGroupErrorKind::ServiceQuotaExceededException(inner) => {
                        Error::ServiceQuotaExceededException(inner)
                    }
                    crate::error::CreateGroupErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::CreateGroupErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteCanaryError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteCanaryError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DeleteCanaryErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::DeleteCanaryErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::DeleteCanaryErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::DeleteCanaryErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DeleteCanaryErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteGroupError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteGroupError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DeleteGroupErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::DeleteGroupErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::DeleteGroupErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::DeleteGroupErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DeleteGroupErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeCanariesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeCanariesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeCanariesErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::DescribeCanariesErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DescribeCanariesErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeCanariesLastRunError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeCanariesLastRunError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeCanariesLastRunErrorKind::InternalServerException(
                        inner,
                    ) => Error::InternalServerException(inner),
                    crate::error::DescribeCanariesLastRunErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DescribeCanariesLastRunErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeRuntimeVersionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeRuntimeVersionsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeRuntimeVersionsErrorKind::InternalServerException(
                        inner,
                    ) => Error::InternalServerException(inner),
                    crate::error::DescribeRuntimeVersionsErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DescribeRuntimeVersionsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisassociateResourceError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DisassociateResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DisassociateResourceErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::DisassociateResourceErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::DisassociateResourceErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::DisassociateResourceErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DisassociateResourceErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetCanaryError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetCanaryError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetCanaryErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::GetCanaryErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::GetCanaryErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetCanaryRunsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetCanaryRunsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetCanaryRunsErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::GetCanaryRunsErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::GetCanaryRunsErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::GetCanaryRunsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetGroupError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetGroupError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetGroupErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::GetGroupErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::GetGroupErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::GetGroupErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::GetGroupErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListAssociatedGroupsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListAssociatedGroupsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListAssociatedGroupsErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::ListAssociatedGroupsErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::ListAssociatedGroupsErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::ListAssociatedGroupsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListGroupResourcesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListGroupResourcesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListGroupResourcesErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::ListGroupResourcesErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::ListGroupResourcesErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::ListGroupResourcesErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::ListGroupResourcesErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListGroupsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListGroupsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListGroupsErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::ListGroupsErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::ListGroupsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListTagsForResourceErrorKind::BadRequestException(inner) => {
                        Error::BadRequestException(inner)
                    }
                    crate::error::ListTagsForResourceErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::ListTagsForResourceErrorKind::InternalFailureException(inner) => {
                        Error::InternalFailureException(inner)
                    }
                    crate::error::ListTagsForResourceErrorKind::NotFoundException(inner) => {
                        Error::NotFoundException(inner)
                    }
                    crate::error::ListTagsForResourceErrorKind::TooManyRequestsException(inner) => {
                        Error::TooManyRequestsException(inner)
                    }
                    crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartCanaryError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StartCanaryError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::StartCanaryErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::StartCanaryErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::StartCanaryErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::StartCanaryErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::StartCanaryErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StopCanaryError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StopCanaryError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::StopCanaryErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::StopCanaryErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::StopCanaryErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::StopCanaryErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::StopCanaryErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::TagResourceErrorKind::BadRequestException(inner) => {
                        Error::BadRequestException(inner)
                    }
                    crate::error::TagResourceErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::TagResourceErrorKind::InternalFailureException(inner) => {
                        Error::InternalFailureException(inner)
                    }
                    crate::error::TagResourceErrorKind::NotFoundException(inner) => {
                        Error::NotFoundException(inner)
                    }
                    crate::error::TagResourceErrorKind::TooManyRequestsException(inner) => {
                        Error::TooManyRequestsException(inner)
                    }
                    crate::error::TagResourceErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::UntagResourceErrorKind::BadRequestException(inner) => {
                        Error::BadRequestException(inner)
                    }
                    crate::error::UntagResourceErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::UntagResourceErrorKind::InternalFailureException(inner) => {
                        Error::InternalFailureException(inner)
                    }
                    crate::error::UntagResourceErrorKind::NotFoundException(inner) => {
                        Error::NotFoundException(inner)
                    }
                    crate::error::UntagResourceErrorKind::TooManyRequestsException(inner) => {
                        Error::TooManyRequestsException(inner)
                    }
                    crate::error::UntagResourceErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateCanaryError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateCanaryError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::UpdateCanaryErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::UpdateCanaryErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::UpdateCanaryErrorKind::RequestEntityTooLargeException(inner) => {
                        Error::RequestEntityTooLargeException(inner)
                    }
                    crate::error::UpdateCanaryErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::UpdateCanaryErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::UpdateCanaryErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
