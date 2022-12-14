// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>403 response - You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>409 response - ConflictException. You might be using a predefined variable.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>500 response - InternalServiceError. Temporary service error. Retry the request.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>404 response - MalformedQueryString. The query string contains a syntax error or resource not found..</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>402 response - You attempted to create more resources than the service allows based on service quotas.</p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p>429 response - LimitExceededException or TooManyRequestsException.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>400 response - Multiple causes. For example, you might have a malformed query string and input parameter might be out of range, or you might have used parameters together incorrectly.</p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
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
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateClusterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateClusterError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::CreateClusterErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::CreateClusterErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::CreateClusterErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::CreateClusterErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::CreateClusterErrorKind::ServiceQuotaExceededException(inner) => {
                        Error::ServiceQuotaExceededException(inner)
                    }
                    crate::error::CreateClusterErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::CreateClusterErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
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
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateControlPanelError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateControlPanelError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::CreateControlPanelErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::CreateControlPanelErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::CreateControlPanelErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::CreateControlPanelErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::CreateControlPanelErrorKind::ServiceQuotaExceededException(
                        inner,
                    ) => Error::ServiceQuotaExceededException(inner),
                    crate::error::CreateControlPanelErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::CreateControlPanelErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::CreateControlPanelErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateRoutingControlError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateRoutingControlError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::CreateRoutingControlErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::CreateRoutingControlErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::CreateRoutingControlErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::CreateRoutingControlErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::CreateRoutingControlErrorKind::ServiceQuotaExceededException(
                        inner,
                    ) => Error::ServiceQuotaExceededException(inner),
                    crate::error::CreateRoutingControlErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::CreateRoutingControlErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::CreateRoutingControlErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateSafetyRuleError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateSafetyRuleError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::CreateSafetyRuleErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::CreateSafetyRuleErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::CreateSafetyRuleErrorKind::Unhandled(inner) => {
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
                    crate::error::DeleteClusterErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::DeleteClusterErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::DeleteClusterErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::DeleteClusterErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::DeleteClusterErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::DeleteClusterErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
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
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteControlPanelError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteControlPanelError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DeleteControlPanelErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::DeleteControlPanelErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::DeleteControlPanelErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::DeleteControlPanelErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::DeleteControlPanelErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::DeleteControlPanelErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DeleteControlPanelErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteRoutingControlError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteRoutingControlError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DeleteRoutingControlErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::DeleteRoutingControlErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::DeleteRoutingControlErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::DeleteRoutingControlErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::DeleteRoutingControlErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::DeleteRoutingControlErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DeleteRoutingControlErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteSafetyRuleError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteSafetyRuleError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DeleteSafetyRuleErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::DeleteSafetyRuleErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::DeleteSafetyRuleErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DeleteSafetyRuleErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeClusterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeClusterError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeClusterErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::DescribeClusterErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::DescribeClusterErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::DescribeClusterErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::DescribeClusterErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::DescribeClusterErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DescribeClusterErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeControlPanelError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeControlPanelError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeControlPanelErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::DescribeControlPanelErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::DescribeControlPanelErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::DescribeControlPanelErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::DescribeControlPanelErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::DescribeControlPanelErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DescribeControlPanelErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeRoutingControlError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeRoutingControlError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeRoutingControlErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::DescribeRoutingControlErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::DescribeRoutingControlErrorKind::InternalServerException(
                        inner,
                    ) => Error::InternalServerException(inner),
                    crate::error::DescribeRoutingControlErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::DescribeRoutingControlErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::DescribeRoutingControlErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DescribeRoutingControlErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeSafetyRuleError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeSafetyRuleError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeSafetyRuleErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::DescribeSafetyRuleErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::DescribeSafetyRuleErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::ListAssociatedRoute53HealthChecksError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::ListAssociatedRoute53HealthChecksError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context.into_err().kind {
                crate::error::ListAssociatedRoute53HealthChecksErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListAssociatedRoute53HealthChecksErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ListAssociatedRoute53HealthChecksErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListAssociatedRoute53HealthChecksErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListClustersError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListClustersError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListClustersErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::ListClustersErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::ListClustersErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::ListClustersErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::ListClustersErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::ListClustersErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListControlPanelsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListControlPanelsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListControlPanelsErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::ListControlPanelsErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::ListControlPanelsErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::ListControlPanelsErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::ListControlPanelsErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::ListControlPanelsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListRoutingControlsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListRoutingControlsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListRoutingControlsErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::ListRoutingControlsErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::ListRoutingControlsErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::ListRoutingControlsErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::ListRoutingControlsErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::ListRoutingControlsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListSafetyRulesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListSafetyRulesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListSafetyRulesErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::ListSafetyRulesErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::ListSafetyRulesErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::ListSafetyRulesErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::ListSafetyRulesErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::ListSafetyRulesErrorKind::Unhandled(inner) => {
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
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateControlPanelError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateControlPanelError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::UpdateControlPanelErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::UpdateControlPanelErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::UpdateControlPanelErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::UpdateControlPanelErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::UpdateControlPanelErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::UpdateControlPanelErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::UpdateControlPanelErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateRoutingControlError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateRoutingControlError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::UpdateRoutingControlErrorKind::AccessDeniedException(inner) => {
                        Error::AccessDeniedException(inner)
                    }
                    crate::error::UpdateRoutingControlErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::UpdateRoutingControlErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::UpdateRoutingControlErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::UpdateRoutingControlErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::UpdateRoutingControlErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::UpdateRoutingControlErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateSafetyRuleError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateSafetyRuleError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::UpdateSafetyRuleErrorKind::InternalServerException(inner) => {
                        Error::InternalServerException(inner)
                    }
                    crate::error::UpdateSafetyRuleErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::UpdateSafetyRuleErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::UpdateSafetyRuleErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
