// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// You do not have sufficient access to perform this action.
    AccessDeniedException(crate::error::AccessDeniedException),
    /// The request could not be processed because of conflict in the current state of the resource.
    ConflictException(crate::error::ConflictException),
    /// Request processing failed because of an error or failure with the service.
    InternalServerException(crate::error::InternalServerException),
    /// The request could not be processed because of conflict in the current state of the campaign.
    InvalidCampaignStateException(crate::error::InvalidCampaignStateException),
    /// The request could not be processed because of conflict in the current state.
    InvalidStateException(crate::error::InvalidStateException),
    /// The specified resource was not found.
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// Request would cause a service quota to be exceeded.
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// The request was denied due to request throttling.
    ThrottlingException(crate::error::ThrottlingException),
    /// The input fails to satisfy the constraints specified by an AWS service.
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::InvalidCampaignStateException(inner) => inner.fmt(f),
            Error::InvalidStateException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateCampaignError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateCampaignError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateCampaignErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CreateCampaignErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateCampaignErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateCampaignErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateCampaignErrorKind::ServiceQuotaExceededException(inner) => {
                    Error::ServiceQuotaExceededException(inner)
                }
                crate::error::CreateCampaignErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::CreateCampaignErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateCampaignErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteCampaignError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteCampaignError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteCampaignErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DeleteCampaignErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DeleteCampaignErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteCampaignErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteCampaignErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteConnectInstanceConfigError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteConnectInstanceConfigError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteConnectInstanceConfigErrorKind::AccessDeniedException(
                    inner,
                ) => Error::AccessDeniedException(inner),
                crate::error::DeleteConnectInstanceConfigErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::DeleteConnectInstanceConfigErrorKind::InvalidStateException(
                    inner,
                ) => Error::InvalidStateException(inner),
                crate::error::DeleteConnectInstanceConfigErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteConnectInstanceConfigErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DeleteConnectInstanceConfigErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteConnectInstanceConfigErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteInstanceOnboardingJobError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteInstanceOnboardingJobError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteInstanceOnboardingJobErrorKind::AccessDeniedException(
                    inner,
                ) => Error::AccessDeniedException(inner),
                crate::error::DeleteInstanceOnboardingJobErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::DeleteInstanceOnboardingJobErrorKind::InvalidStateException(
                    inner,
                ) => Error::InvalidStateException(inner),
                crate::error::DeleteInstanceOnboardingJobErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteInstanceOnboardingJobErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteInstanceOnboardingJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeCampaignError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeCampaignError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeCampaignErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DescribeCampaignErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DescribeCampaignErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeCampaignErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeCampaignErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetCampaignStateError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetCampaignStateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetCampaignStateErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetCampaignStateErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetCampaignStateErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetCampaignStateErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetCampaignStateErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::GetCampaignStateErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetCampaignStateBatchError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetCampaignStateBatchError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetCampaignStateBatchErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetCampaignStateBatchErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetCampaignStateBatchErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetCampaignStateBatchErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::GetCampaignStateBatchErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetConnectInstanceConfigError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetConnectInstanceConfigError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetConnectInstanceConfigErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetConnectInstanceConfigErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetConnectInstanceConfigErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::GetConnectInstanceConfigErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::GetConnectInstanceConfigErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::GetInstanceOnboardingJobStatusError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetInstanceOnboardingJobStatusError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetInstanceOnboardingJobStatusErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetInstanceOnboardingJobStatusErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetInstanceOnboardingJobStatusErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetInstanceOnboardingJobStatusErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetInstanceOnboardingJobStatusErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListCampaignsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListCampaignsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListCampaignsErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListCampaignsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListCampaignsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListCampaignsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
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
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForResourceErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PauseCampaignError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PauseCampaignError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PauseCampaignErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::PauseCampaignErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::PauseCampaignErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::PauseCampaignErrorKind::InvalidCampaignStateException(inner) => {
                    Error::InvalidCampaignStateException(inner)
                }
                crate::error::PauseCampaignErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::PauseCampaignErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::PauseCampaignErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::PauseCampaignErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutDialRequestBatchError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::PutDialRequestBatchError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutDialRequestBatchErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::PutDialRequestBatchErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::PutDialRequestBatchErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::PutDialRequestBatchErrorKind::InvalidCampaignStateException(
                    inner,
                ) => Error::InvalidCampaignStateException(inner),
                crate::error::PutDialRequestBatchErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::PutDialRequestBatchErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::PutDialRequestBatchErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::PutDialRequestBatchErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ResumeCampaignError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ResumeCampaignError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ResumeCampaignErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ResumeCampaignErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::ResumeCampaignErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ResumeCampaignErrorKind::InvalidCampaignStateException(inner) => {
                    Error::InvalidCampaignStateException(inner)
                }
                crate::error::ResumeCampaignErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ResumeCampaignErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ResumeCampaignErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ResumeCampaignErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartCampaignError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StartCampaignError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartCampaignErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::StartCampaignErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::StartCampaignErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StartCampaignErrorKind::InvalidCampaignStateException(inner) => {
                    Error::InvalidCampaignStateException(inner)
                }
                crate::error::StartCampaignErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::StartCampaignErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::StartCampaignErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::StartCampaignErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartInstanceOnboardingJobError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartInstanceOnboardingJobError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartInstanceOnboardingJobErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::StartInstanceOnboardingJobErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::StartInstanceOnboardingJobErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::StartInstanceOnboardingJobErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::StartInstanceOnboardingJobErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::StartInstanceOnboardingJobErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::StartInstanceOnboardingJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StopCampaignError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StopCampaignError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StopCampaignErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::StopCampaignErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::StopCampaignErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StopCampaignErrorKind::InvalidCampaignStateException(inner) => {
                    Error::InvalidCampaignStateException(inner)
                }
                crate::error::StopCampaignErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::StopCampaignErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::StopCampaignErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::StopCampaignErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagResourceErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::TagResourceErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::TagResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::TagResourceErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagResourceErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UntagResourceErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UntagResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::UntagResourceErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateCampaignDialerConfigError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateCampaignDialerConfigError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateCampaignDialerConfigErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UpdateCampaignDialerConfigErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::UpdateCampaignDialerConfigErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::UpdateCampaignDialerConfigErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateCampaignDialerConfigErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateCampaignDialerConfigErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateCampaignNameError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateCampaignNameError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateCampaignNameErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UpdateCampaignNameErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::UpdateCampaignNameErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UpdateCampaignNameErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateCampaignNameErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateCampaignNameErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::UpdateCampaignOutboundCallConfigError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::UpdateCampaignOutboundCallConfigError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateCampaignOutboundCallConfigErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::UpdateCampaignOutboundCallConfigErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::UpdateCampaignOutboundCallConfigErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::UpdateCampaignOutboundCallConfigErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateCampaignOutboundCallConfigErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::UpdateCampaignOutboundCallConfigErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UpdateCampaignOutboundCallConfigErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
