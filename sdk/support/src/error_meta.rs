// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>An attachment with the specified ID could not be found.</p>
    AttachmentIdNotFound(crate::error::AttachmentIdNotFound),
    /// <p>The limit for the number of attachment sets created in a short period of time has been exceeded.</p>
    AttachmentLimitExceeded(crate::error::AttachmentLimitExceeded),
    /// <p>The expiration time of the attachment set has passed. The set expires 1 hour after it is created.</p>
    AttachmentSetExpired(crate::error::AttachmentSetExpired),
    /// <p>An attachment set with the specified ID could not be found.</p>
    AttachmentSetIdNotFound(crate::error::AttachmentSetIdNotFound),
    /// <p>A limit for the size of an attachment set has been exceeded. The limits are three attachments and 5 MB per attachment.</p>
    AttachmentSetSizeLimitExceeded(crate::error::AttachmentSetSizeLimitExceeded),
    /// <p>The case creation limit for the account has been exceeded.</p>
    CaseCreationLimitExceeded(crate::error::CaseCreationLimitExceeded),
    /// <p>The requested <code>caseId</code> couldn't be located.</p>
    CaseIdNotFound(crate::error::CaseIdNotFound),
    /// <p>The limit for the number of <code>DescribeAttachment</code> requests in a short period of time has been exceeded.</p>
    DescribeAttachmentLimitExceeded(crate::error::DescribeAttachmentLimitExceeded),
    /// <p>An internal server error occurred.</p>
    InternalServerError(crate::error::InternalServerError),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AttachmentIdNotFound(inner) => inner.fmt(f),
            Error::AttachmentLimitExceeded(inner) => inner.fmt(f),
            Error::AttachmentSetExpired(inner) => inner.fmt(f),
            Error::AttachmentSetIdNotFound(inner) => inner.fmt(f),
            Error::AttachmentSetSizeLimitExceeded(inner) => inner.fmt(f),
            Error::CaseCreationLimitExceeded(inner) => inner.fmt(f),
            Error::CaseIdNotFound(inner) => inner.fmt(f),
            Error::DescribeAttachmentLimitExceeded(inner) => inner.fmt(f),
            Error::InternalServerError(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AddAttachmentsToSetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AddAttachmentsToSetError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::AddAttachmentsToSetErrorKind::AttachmentLimitExceeded(inner) => {
                        Error::AttachmentLimitExceeded(inner)
                    }
                    crate::error::AddAttachmentsToSetErrorKind::AttachmentSetExpired(inner) => {
                        Error::AttachmentSetExpired(inner)
                    }
                    crate::error::AddAttachmentsToSetErrorKind::AttachmentSetIdNotFound(inner) => {
                        Error::AttachmentSetIdNotFound(inner)
                    }
                    crate::error::AddAttachmentsToSetErrorKind::AttachmentSetSizeLimitExceeded(
                        inner,
                    ) => Error::AttachmentSetSizeLimitExceeded(inner),
                    crate::error::AddAttachmentsToSetErrorKind::InternalServerError(inner) => {
                        Error::InternalServerError(inner)
                    }
                    crate::error::AddAttachmentsToSetErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AddCommunicationToCaseError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AddCommunicationToCaseError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::AddCommunicationToCaseErrorKind::AttachmentSetExpired(inner) => {
                        Error::AttachmentSetExpired(inner)
                    }
                    crate::error::AddCommunicationToCaseErrorKind::AttachmentSetIdNotFound(
                        inner,
                    ) => Error::AttachmentSetIdNotFound(inner),
                    crate::error::AddCommunicationToCaseErrorKind::CaseIdNotFound(inner) => {
                        Error::CaseIdNotFound(inner)
                    }
                    crate::error::AddCommunicationToCaseErrorKind::InternalServerError(inner) => {
                        Error::InternalServerError(inner)
                    }
                    crate::error::AddCommunicationToCaseErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateCaseError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateCaseError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::CreateCaseErrorKind::AttachmentSetExpired(inner) => {
                        Error::AttachmentSetExpired(inner)
                    }
                    crate::error::CreateCaseErrorKind::AttachmentSetIdNotFound(inner) => {
                        Error::AttachmentSetIdNotFound(inner)
                    }
                    crate::error::CreateCaseErrorKind::CaseCreationLimitExceeded(inner) => {
                        Error::CaseCreationLimitExceeded(inner)
                    }
                    crate::error::CreateCaseErrorKind::InternalServerError(inner) => {
                        Error::InternalServerError(inner)
                    }
                    crate::error::CreateCaseErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeAttachmentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeAttachmentError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeAttachmentErrorKind::AttachmentIdNotFound(inner) => {
                        Error::AttachmentIdNotFound(inner)
                    }
                    crate::error::DescribeAttachmentErrorKind::DescribeAttachmentLimitExceeded(
                        inner,
                    ) => Error::DescribeAttachmentLimitExceeded(inner),
                    crate::error::DescribeAttachmentErrorKind::InternalServerError(inner) => {
                        Error::InternalServerError(inner)
                    }
                    crate::error::DescribeAttachmentErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeCasesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeCasesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeCasesErrorKind::CaseIdNotFound(inner) => {
                        Error::CaseIdNotFound(inner)
                    }
                    crate::error::DescribeCasesErrorKind::InternalServerError(inner) => {
                        Error::InternalServerError(inner)
                    }
                    crate::error::DescribeCasesErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeCommunicationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeCommunicationsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeCommunicationsErrorKind::CaseIdNotFound(inner) => {
                        Error::CaseIdNotFound(inner)
                    }
                    crate::error::DescribeCommunicationsErrorKind::InternalServerError(inner) => {
                        Error::InternalServerError(inner)
                    }
                    crate::error::DescribeCommunicationsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeServicesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeServicesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeServicesErrorKind::InternalServerError(inner) => {
                        Error::InternalServerError(inner)
                    }
                    crate::error::DescribeServicesErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeSeverityLevelsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeSeverityLevelsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeSeverityLevelsErrorKind::InternalServerError(inner) => {
                        Error::InternalServerError(inner)
                    }
                    crate::error::DescribeSeverityLevelsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::DescribeTrustedAdvisorCheckRefreshStatusesError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DescribeTrustedAdvisorCheckRefreshStatusesError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context.into_err().kind {
                crate::error::DescribeTrustedAdvisorCheckRefreshStatusesErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::DescribeTrustedAdvisorCheckRefreshStatusesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::DescribeTrustedAdvisorCheckResultError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DescribeTrustedAdvisorCheckResultError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context
                .into_err()
                .kind
            {
                crate::error::DescribeTrustedAdvisorCheckResultErrorKind::InternalServerError(
                    inner,
                ) => Error::InternalServerError(inner),
                crate::error::DescribeTrustedAdvisorCheckResultErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeTrustedAdvisorChecksError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeTrustedAdvisorChecksError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeTrustedAdvisorChecksErrorKind::InternalServerError(
                        inner,
                    ) => Error::InternalServerError(inner),
                    crate::error::DescribeTrustedAdvisorChecksErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::DescribeTrustedAdvisorCheckSummariesError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DescribeTrustedAdvisorCheckSummariesError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context.into_err().kind {
                crate::error::DescribeTrustedAdvisorCheckSummariesErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::DescribeTrustedAdvisorCheckSummariesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RefreshTrustedAdvisorCheckError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RefreshTrustedAdvisorCheckError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::RefreshTrustedAdvisorCheckErrorKind::InternalServerError(
                        inner,
                    ) => Error::InternalServerError(inner),
                    crate::error::RefreshTrustedAdvisorCheckErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ResolveCaseError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ResolveCaseError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ResolveCaseErrorKind::CaseIdNotFound(inner) => {
                        Error::CaseIdNotFound(inner)
                    }
                    crate::error::ResolveCaseErrorKind::InternalServerError(inner) => {
                        Error::InternalServerError(inner)
                    }
                    crate::error::ResolveCaseErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
