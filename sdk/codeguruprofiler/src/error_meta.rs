// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    ConflictException(crate::error::ConflictException),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use <a href="https://docs.aws.amazon.com/servicequotas/latest/userguide/intro.html">Service Quotas</a> to request a service quota increase. </p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The parameter is not valid.</p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AddNotificationChannelsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AddNotificationChannelsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context
                .into_err()
                .kind
            {
                crate::error::AddNotificationChannelsErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::AddNotificationChannelsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::AddNotificationChannelsErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::AddNotificationChannelsErrorKind::ServiceQuotaExceededException(
                    inner,
                ) => Error::ServiceQuotaExceededException(inner),
                crate::error::AddNotificationChannelsErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::AddNotificationChannelsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::AddNotificationChannelsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BatchGetFrameMetricDataError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::BatchGetFrameMetricDataError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::BatchGetFrameMetricDataErrorKind::InternalServerException(
                        inner,
                    ) => Error::InternalServerException(inner),
                    crate::error::BatchGetFrameMetricDataErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::BatchGetFrameMetricDataErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::BatchGetFrameMetricDataErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::BatchGetFrameMetricDataErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ConfigureAgentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ConfigureAgentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ConfigureAgentErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::ConfigureAgentErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::ConfigureAgentErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::ConfigureAgentErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::ConfigureAgentErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateProfilingGroupError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateProfilingGroupError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::CreateProfilingGroupErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::CreateProfilingGroupErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::CreateProfilingGroupErrorKind::ServiceQuotaExceededException(
                        inner,
                    ) => Error::ServiceQuotaExceededException(inner),
                    crate::error::CreateProfilingGroupErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::CreateProfilingGroupErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::CreateProfilingGroupErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteProfilingGroupError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteProfilingGroupError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DeleteProfilingGroupErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::DeleteProfilingGroupErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::DeleteProfilingGroupErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::DeleteProfilingGroupErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::DeleteProfilingGroupErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DeleteProfilingGroupErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeProfilingGroupError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeProfilingGroupError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeProfilingGroupErrorKind::InternalServerException(
                        inner,
                    ) => Error::InternalServerException(inner),
                    crate::error::DescribeProfilingGroupErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::DescribeProfilingGroupErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::DescribeProfilingGroupErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DescribeProfilingGroupErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::GetFindingsReportAccountSummaryError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetFindingsReportAccountSummaryError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context
                .into_err()
                .kind
            {
                crate::error::GetFindingsReportAccountSummaryErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::GetFindingsReportAccountSummaryErrorKind::ThrottlingException(
                    inner,
                ) => Error::ThrottlingException(inner),
                crate::error::GetFindingsReportAccountSummaryErrorKind::ValidationException(
                    inner,
                ) => Error::ValidationException(inner),
                crate::error::GetFindingsReportAccountSummaryErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetNotificationConfigurationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetNotificationConfigurationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context
                .into_err()
                .kind
            {
                crate::error::GetNotificationConfigurationErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::GetNotificationConfigurationErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::GetNotificationConfigurationErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetNotificationConfigurationErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::GetNotificationConfigurationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPolicyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetPolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetPolicyErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::GetPolicyErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::GetPolicyErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::GetPolicyErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetProfileError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetProfileError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetProfileErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::GetProfileErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::GetProfileErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::GetProfileErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::GetProfileErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetRecommendationsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetRecommendationsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetRecommendationsErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::GetRecommendationsErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::GetRecommendationsErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::GetRecommendationsErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::GetRecommendationsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListFindingsReportsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListFindingsReportsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListFindingsReportsErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::ListFindingsReportsErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::ListFindingsReportsErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::ListFindingsReportsErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::ListFindingsReportsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListProfileTimesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListProfileTimesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListProfileTimesErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::ListProfileTimesErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::ListProfileTimesErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::ListProfileTimesErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::ListProfileTimesErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListProfilingGroupsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListProfilingGroupsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListProfilingGroupsErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::ListProfilingGroupsErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::ListProfilingGroupsErrorKind::Unhandled(inner) => {
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
                    crate::error::ListTagsForResourceErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::ListTagsForResourceErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
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
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PostAgentProfileError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::PostAgentProfileError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::PostAgentProfileErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::PostAgentProfileErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::PostAgentProfileErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::PostAgentProfileErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::PostAgentProfileErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutPermissionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutPermissionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::PutPermissionErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::PutPermissionErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::PutPermissionErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::PutPermissionErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::PutPermissionErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::PutPermissionErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RemoveNotificationChannelError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RemoveNotificationChannelError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::RemoveNotificationChannelErrorKind::InternalServerException(
                        inner,
                    ) => Error::InternalServerException(inner),
                    crate::error::RemoveNotificationChannelErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::RemoveNotificationChannelErrorKind::ThrottlingException(
                        inner,
                    ) => Error::ThrottlingException(inner),
                    crate::error::RemoveNotificationChannelErrorKind::ValidationException(
                        inner,
                    ) => Error::ValidationException(inner),
                    crate::error::RemoveNotificationChannelErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RemovePermissionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RemovePermissionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::RemovePermissionErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::RemovePermissionErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::RemovePermissionErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::RemovePermissionErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::RemovePermissionErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::RemovePermissionErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SubmitFeedbackError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::SubmitFeedbackError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::SubmitFeedbackErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::SubmitFeedbackErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::SubmitFeedbackErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::SubmitFeedbackErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::SubmitFeedbackErrorKind::Unhandled(inner) => {
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
                    crate::error::TagResourceErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::TagResourceErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
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
                    crate::error::UntagResourceErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::UntagResourceErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
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
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateProfilingGroupError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateProfilingGroupError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::UpdateProfilingGroupErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::UpdateProfilingGroupErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::UpdateProfilingGroupErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::UpdateProfilingGroupErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::UpdateProfilingGroupErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::UpdateProfilingGroupErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
