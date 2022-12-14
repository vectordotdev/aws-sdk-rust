// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDeniedException(crate::error::CloudHsmAccessDeniedException),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailureException(crate::error::CloudHsmInternalFailureException),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequestException(crate::error::CloudHsmInvalidRequestException),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFoundException(crate::error::CloudHsmResourceNotFoundException),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmServiceException(crate::error::CloudHsmServiceException),
    /// <p>The request was rejected because of a tagging failure. Verify the tag conditions in all applicable policies, and then retry the request.</p>
    CloudHsmTagException(crate::error::CloudHsmTagException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CloudHsmAccessDeniedException(inner) => inner.fmt(f),
            Error::CloudHsmInternalFailureException(inner) => inner.fmt(f),
            Error::CloudHsmInvalidRequestException(inner) => inner.fmt(f),
            Error::CloudHsmResourceNotFoundException(inner) => inner.fmt(f),
            Error::CloudHsmServiceException(inner) => inner.fmt(f),
            Error::CloudHsmTagException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CopyBackupToRegionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CopyBackupToRegionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context
                .into_err()
                .kind
            {
                crate::error::CopyBackupToRegionErrorKind::CloudHsmAccessDeniedException(inner) => {
                    Error::CloudHsmAccessDeniedException(inner)
                }
                crate::error::CopyBackupToRegionErrorKind::CloudHsmInternalFailureException(
                    inner,
                ) => Error::CloudHsmInternalFailureException(inner),
                crate::error::CopyBackupToRegionErrorKind::CloudHsmInvalidRequestException(
                    inner,
                ) => Error::CloudHsmInvalidRequestException(inner),
                crate::error::CopyBackupToRegionErrorKind::CloudHsmResourceNotFoundException(
                    inner,
                ) => Error::CloudHsmResourceNotFoundException(inner),
                crate::error::CopyBackupToRegionErrorKind::CloudHsmServiceException(inner) => {
                    Error::CloudHsmServiceException(inner)
                }
                crate::error::CopyBackupToRegionErrorKind::CloudHsmTagException(inner) => {
                    Error::CloudHsmTagException(inner)
                }
                crate::error::CopyBackupToRegionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateClusterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateClusterError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::CreateClusterErrorKind::CloudHsmAccessDeniedException(inner) => {
                        Error::CloudHsmAccessDeniedException(inner)
                    }
                    crate::error::CreateClusterErrorKind::CloudHsmInternalFailureException(
                        inner,
                    ) => Error::CloudHsmInternalFailureException(inner),
                    crate::error::CreateClusterErrorKind::CloudHsmInvalidRequestException(
                        inner,
                    ) => Error::CloudHsmInvalidRequestException(inner),
                    crate::error::CreateClusterErrorKind::CloudHsmResourceNotFoundException(
                        inner,
                    ) => Error::CloudHsmResourceNotFoundException(inner),
                    crate::error::CreateClusterErrorKind::CloudHsmServiceException(inner) => {
                        Error::CloudHsmServiceException(inner)
                    }
                    crate::error::CreateClusterErrorKind::CloudHsmTagException(inner) => {
                        Error::CloudHsmTagException(inner)
                    }
                    crate::error::CreateClusterErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateHsmError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateHsmError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::CreateHsmErrorKind::CloudHsmAccessDeniedException(inner) => {
                        Error::CloudHsmAccessDeniedException(inner)
                    }
                    crate::error::CreateHsmErrorKind::CloudHsmInternalFailureException(inner) => {
                        Error::CloudHsmInternalFailureException(inner)
                    }
                    crate::error::CreateHsmErrorKind::CloudHsmInvalidRequestException(inner) => {
                        Error::CloudHsmInvalidRequestException(inner)
                    }
                    crate::error::CreateHsmErrorKind::CloudHsmResourceNotFoundException(inner) => {
                        Error::CloudHsmResourceNotFoundException(inner)
                    }
                    crate::error::CreateHsmErrorKind::CloudHsmServiceException(inner) => {
                        Error::CloudHsmServiceException(inner)
                    }
                    crate::error::CreateHsmErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteBackupError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteBackupError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DeleteBackupErrorKind::CloudHsmAccessDeniedException(inner) => {
                        Error::CloudHsmAccessDeniedException(inner)
                    }
                    crate::error::DeleteBackupErrorKind::CloudHsmInternalFailureException(
                        inner,
                    ) => Error::CloudHsmInternalFailureException(inner),
                    crate::error::DeleteBackupErrorKind::CloudHsmInvalidRequestException(inner) => {
                        Error::CloudHsmInvalidRequestException(inner)
                    }
                    crate::error::DeleteBackupErrorKind::CloudHsmResourceNotFoundException(
                        inner,
                    ) => Error::CloudHsmResourceNotFoundException(inner),
                    crate::error::DeleteBackupErrorKind::CloudHsmServiceException(inner) => {
                        Error::CloudHsmServiceException(inner)
                    }
                    crate::error::DeleteBackupErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteClusterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteClusterError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DeleteClusterErrorKind::CloudHsmAccessDeniedException(inner) => {
                        Error::CloudHsmAccessDeniedException(inner)
                    }
                    crate::error::DeleteClusterErrorKind::CloudHsmInternalFailureException(
                        inner,
                    ) => Error::CloudHsmInternalFailureException(inner),
                    crate::error::DeleteClusterErrorKind::CloudHsmInvalidRequestException(
                        inner,
                    ) => Error::CloudHsmInvalidRequestException(inner),
                    crate::error::DeleteClusterErrorKind::CloudHsmResourceNotFoundException(
                        inner,
                    ) => Error::CloudHsmResourceNotFoundException(inner),
                    crate::error::DeleteClusterErrorKind::CloudHsmServiceException(inner) => {
                        Error::CloudHsmServiceException(inner)
                    }
                    crate::error::DeleteClusterErrorKind::CloudHsmTagException(inner) => {
                        Error::CloudHsmTagException(inner)
                    }
                    crate::error::DeleteClusterErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteHsmError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteHsmError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DeleteHsmErrorKind::CloudHsmAccessDeniedException(inner) => {
                        Error::CloudHsmAccessDeniedException(inner)
                    }
                    crate::error::DeleteHsmErrorKind::CloudHsmInternalFailureException(inner) => {
                        Error::CloudHsmInternalFailureException(inner)
                    }
                    crate::error::DeleteHsmErrorKind::CloudHsmInvalidRequestException(inner) => {
                        Error::CloudHsmInvalidRequestException(inner)
                    }
                    crate::error::DeleteHsmErrorKind::CloudHsmResourceNotFoundException(inner) => {
                        Error::CloudHsmResourceNotFoundException(inner)
                    }
                    crate::error::DeleteHsmErrorKind::CloudHsmServiceException(inner) => {
                        Error::CloudHsmServiceException(inner)
                    }
                    crate::error::DeleteHsmErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeBackupsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeBackupsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeBackupsErrorKind::CloudHsmAccessDeniedException(
                        inner,
                    ) => Error::CloudHsmAccessDeniedException(inner),
                    crate::error::DescribeBackupsErrorKind::CloudHsmInternalFailureException(
                        inner,
                    ) => Error::CloudHsmInternalFailureException(inner),
                    crate::error::DescribeBackupsErrorKind::CloudHsmInvalidRequestException(
                        inner,
                    ) => Error::CloudHsmInvalidRequestException(inner),
                    crate::error::DescribeBackupsErrorKind::CloudHsmResourceNotFoundException(
                        inner,
                    ) => Error::CloudHsmResourceNotFoundException(inner),
                    crate::error::DescribeBackupsErrorKind::CloudHsmServiceException(inner) => {
                        Error::CloudHsmServiceException(inner)
                    }
                    crate::error::DescribeBackupsErrorKind::CloudHsmTagException(inner) => {
                        Error::CloudHsmTagException(inner)
                    }
                    crate::error::DescribeBackupsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeClustersError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeClustersError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeClustersErrorKind::CloudHsmAccessDeniedException(
                        inner,
                    ) => Error::CloudHsmAccessDeniedException(inner),
                    crate::error::DescribeClustersErrorKind::CloudHsmInternalFailureException(
                        inner,
                    ) => Error::CloudHsmInternalFailureException(inner),
                    crate::error::DescribeClustersErrorKind::CloudHsmInvalidRequestException(
                        inner,
                    ) => Error::CloudHsmInvalidRequestException(inner),
                    crate::error::DescribeClustersErrorKind::CloudHsmServiceException(inner) => {
                        Error::CloudHsmServiceException(inner)
                    }
                    crate::error::DescribeClustersErrorKind::CloudHsmTagException(inner) => {
                        Error::CloudHsmTagException(inner)
                    }
                    crate::error::DescribeClustersErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::InitializeClusterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::InitializeClusterError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::InitializeClusterErrorKind::CloudHsmAccessDeniedException(
                        inner,
                    ) => Error::CloudHsmAccessDeniedException(inner),
                    crate::error::InitializeClusterErrorKind::CloudHsmInternalFailureException(
                        inner,
                    ) => Error::CloudHsmInternalFailureException(inner),
                    crate::error::InitializeClusterErrorKind::CloudHsmInvalidRequestException(
                        inner,
                    ) => Error::CloudHsmInvalidRequestException(inner),
                    crate::error::InitializeClusterErrorKind::CloudHsmResourceNotFoundException(
                        inner,
                    ) => Error::CloudHsmResourceNotFoundException(inner),
                    crate::error::InitializeClusterErrorKind::CloudHsmServiceException(inner) => {
                        Error::CloudHsmServiceException(inner)
                    }
                    crate::error::InitializeClusterErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListTagsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListTagsErrorKind::CloudHsmAccessDeniedException(inner) => {
                        Error::CloudHsmAccessDeniedException(inner)
                    }
                    crate::error::ListTagsErrorKind::CloudHsmInternalFailureException(inner) => {
                        Error::CloudHsmInternalFailureException(inner)
                    }
                    crate::error::ListTagsErrorKind::CloudHsmInvalidRequestException(inner) => {
                        Error::CloudHsmInvalidRequestException(inner)
                    }
                    crate::error::ListTagsErrorKind::CloudHsmResourceNotFoundException(inner) => {
                        Error::CloudHsmResourceNotFoundException(inner)
                    }
                    crate::error::ListTagsErrorKind::CloudHsmServiceException(inner) => {
                        Error::CloudHsmServiceException(inner)
                    }
                    crate::error::ListTagsErrorKind::CloudHsmTagException(inner) => {
                        Error::CloudHsmTagException(inner)
                    }
                    crate::error::ListTagsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ModifyBackupAttributesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ModifyBackupAttributesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context.into_err().kind {
                crate::error::ModifyBackupAttributesErrorKind::CloudHsmAccessDeniedException(inner) => Error::CloudHsmAccessDeniedException(inner),
                crate::error::ModifyBackupAttributesErrorKind::CloudHsmInternalFailureException(inner) => Error::CloudHsmInternalFailureException(inner),
                crate::error::ModifyBackupAttributesErrorKind::CloudHsmInvalidRequestException(inner) => Error::CloudHsmInvalidRequestException(inner),
                crate::error::ModifyBackupAttributesErrorKind::CloudHsmResourceNotFoundException(inner) => Error::CloudHsmResourceNotFoundException(inner),
                crate::error::ModifyBackupAttributesErrorKind::CloudHsmServiceException(inner) => Error::CloudHsmServiceException(inner),
                crate::error::ModifyBackupAttributesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ModifyClusterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ModifyClusterError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ModifyClusterErrorKind::CloudHsmAccessDeniedException(inner) => {
                        Error::CloudHsmAccessDeniedException(inner)
                    }
                    crate::error::ModifyClusterErrorKind::CloudHsmInternalFailureException(
                        inner,
                    ) => Error::CloudHsmInternalFailureException(inner),
                    crate::error::ModifyClusterErrorKind::CloudHsmInvalidRequestException(
                        inner,
                    ) => Error::CloudHsmInvalidRequestException(inner),
                    crate::error::ModifyClusterErrorKind::CloudHsmResourceNotFoundException(
                        inner,
                    ) => Error::CloudHsmResourceNotFoundException(inner),
                    crate::error::ModifyClusterErrorKind::CloudHsmServiceException(inner) => {
                        Error::CloudHsmServiceException(inner)
                    }
                    crate::error::ModifyClusterErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RestoreBackupError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::RestoreBackupError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::RestoreBackupErrorKind::CloudHsmAccessDeniedException(inner) => {
                        Error::CloudHsmAccessDeniedException(inner)
                    }
                    crate::error::RestoreBackupErrorKind::CloudHsmInternalFailureException(
                        inner,
                    ) => Error::CloudHsmInternalFailureException(inner),
                    crate::error::RestoreBackupErrorKind::CloudHsmInvalidRequestException(
                        inner,
                    ) => Error::CloudHsmInvalidRequestException(inner),
                    crate::error::RestoreBackupErrorKind::CloudHsmResourceNotFoundException(
                        inner,
                    ) => Error::CloudHsmResourceNotFoundException(inner),
                    crate::error::RestoreBackupErrorKind::CloudHsmServiceException(inner) => {
                        Error::CloudHsmServiceException(inner)
                    }
                    crate::error::RestoreBackupErrorKind::Unhandled(inner) => {
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
                    crate::error::TagResourceErrorKind::CloudHsmAccessDeniedException(inner) => {
                        Error::CloudHsmAccessDeniedException(inner)
                    }
                    crate::error::TagResourceErrorKind::CloudHsmInternalFailureException(inner) => {
                        Error::CloudHsmInternalFailureException(inner)
                    }
                    crate::error::TagResourceErrorKind::CloudHsmInvalidRequestException(inner) => {
                        Error::CloudHsmInvalidRequestException(inner)
                    }
                    crate::error::TagResourceErrorKind::CloudHsmResourceNotFoundException(
                        inner,
                    ) => Error::CloudHsmResourceNotFoundException(inner),
                    crate::error::TagResourceErrorKind::CloudHsmServiceException(inner) => {
                        Error::CloudHsmServiceException(inner)
                    }
                    crate::error::TagResourceErrorKind::CloudHsmTagException(inner) => {
                        Error::CloudHsmTagException(inner)
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
                    crate::error::UntagResourceErrorKind::CloudHsmAccessDeniedException(inner) => {
                        Error::CloudHsmAccessDeniedException(inner)
                    }
                    crate::error::UntagResourceErrorKind::CloudHsmInternalFailureException(
                        inner,
                    ) => Error::CloudHsmInternalFailureException(inner),
                    crate::error::UntagResourceErrorKind::CloudHsmInvalidRequestException(
                        inner,
                    ) => Error::CloudHsmInvalidRequestException(inner),
                    crate::error::UntagResourceErrorKind::CloudHsmResourceNotFoundException(
                        inner,
                    ) => Error::CloudHsmResourceNotFoundException(inner),
                    crate::error::UntagResourceErrorKind::CloudHsmServiceException(inner) => {
                        Error::CloudHsmServiceException(inner)
                    }
                    crate::error::UntagResourceErrorKind::CloudHsmTagException(inner) => {
                        Error::CloudHsmTagException(inner)
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
impl std::error::Error for Error {}
