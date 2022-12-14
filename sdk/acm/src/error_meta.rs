// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have access required to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>You are trying to update a resource or configuration that is already being created or updated. Wait for the previous operation to finish and try again.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>One or more of of request parameters specified is not valid.</p>
    InvalidArgsException(crate::error::InvalidArgsException),
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidArnException(crate::error::InvalidArnException),
    /// <p>One or more values in the <code>DomainValidationOption</code> structure is incorrect.</p>
    InvalidDomainValidationOptionsException(crate::error::InvalidDomainValidationOptionsException),
    /// <p>An input parameter was invalid.</p>
    InvalidParameterException(crate::error::InvalidParameterException),
    /// <p>Processing has reached an invalid state.</p>
    InvalidStateException(crate::error::InvalidStateException),
    /// <p>One or both of the values that make up the key-value pair is not valid. For example, you cannot specify a tag value that begins with <code>aws:</code>.</p>
    InvalidTagException(crate::error::InvalidTagException),
    /// <p>An ACM quota has been exceeded.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The certificate request is in process and the certificate in your account has not yet been issued.</p>
    RequestInProgressException(crate::error::RequestInProgressException),
    /// <p>The certificate is in use by another Amazon Web Services service in the caller's account. Remove the association and try again.</p>
    ResourceInUseException(crate::error::ResourceInUseException),
    /// <p>The specified certificate cannot be found in the caller's account or the caller's account cannot be found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>A specified tag did not comply with an existing tag policy and was rejected.</p>
    TagPolicyException(crate::error::TagPolicyException),
    /// <p>The request was denied because it exceeded a quota.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The request contains too many tags. Try the request again with fewer tags.</p>
    TooManyTagsException(crate::error::TooManyTagsException),
    /// <p>The supplied input failed to satisfy constraints of an Amazon Web Services service.</p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InvalidArgsException(inner) => inner.fmt(f),
            Error::InvalidArnException(inner) => inner.fmt(f),
            Error::InvalidDomainValidationOptionsException(inner) => inner.fmt(f),
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::InvalidStateException(inner) => inner.fmt(f),
            Error::InvalidTagException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::RequestInProgressException(inner) => inner.fmt(f),
            Error::ResourceInUseException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::TagPolicyException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::TooManyTagsException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AddTagsToCertificateError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AddTagsToCertificateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::AddTagsToCertificateErrorKind::InvalidArnException(inner) => {
                        Error::InvalidArnException(inner)
                    }
                    crate::error::AddTagsToCertificateErrorKind::InvalidParameterException(
                        inner,
                    ) => Error::InvalidParameterException(inner),
                    crate::error::AddTagsToCertificateErrorKind::InvalidTagException(inner) => {
                        Error::InvalidTagException(inner)
                    }
                    crate::error::AddTagsToCertificateErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::AddTagsToCertificateErrorKind::TagPolicyException(inner) => {
                        Error::TagPolicyException(inner)
                    }
                    crate::error::AddTagsToCertificateErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::AddTagsToCertificateErrorKind::TooManyTagsException(inner) => {
                        Error::TooManyTagsException(inner)
                    }
                    crate::error::AddTagsToCertificateErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteCertificateError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteCertificateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DeleteCertificateErrorKind::InvalidArnException(inner) => {
                        Error::InvalidArnException(inner)
                    }
                    crate::error::DeleteCertificateErrorKind::ResourceInUseException(inner) => {
                        Error::ResourceInUseException(inner)
                    }
                    crate::error::DeleteCertificateErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::DeleteCertificateErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeCertificateError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeCertificateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DescribeCertificateErrorKind::InvalidArnException(inner) => {
                        Error::InvalidArnException(inner)
                    }
                    crate::error::DescribeCertificateErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::DescribeCertificateErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ExportCertificateError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ExportCertificateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ExportCertificateErrorKind::InvalidArnException(inner) => {
                        Error::InvalidArnException(inner)
                    }
                    crate::error::ExportCertificateErrorKind::RequestInProgressException(inner) => {
                        Error::RequestInProgressException(inner)
                    }
                    crate::error::ExportCertificateErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::ExportCertificateErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetAccountConfigurationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetAccountConfigurationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetAccountConfigurationErrorKind::AccessDeniedException(
                        inner,
                    ) => Error::AccessDeniedException(inner),
                    crate::error::GetAccountConfigurationErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::GetAccountConfigurationErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetCertificateError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetCertificateError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetCertificateErrorKind::InvalidArnException(inner) => {
                        Error::InvalidArnException(inner)
                    }
                    crate::error::GetCertificateErrorKind::RequestInProgressException(inner) => {
                        Error::RequestInProgressException(inner)
                    }
                    crate::error::GetCertificateErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::GetCertificateErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ImportCertificateError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ImportCertificateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ImportCertificateErrorKind::InvalidArnException(inner) => {
                        Error::InvalidArnException(inner)
                    }
                    crate::error::ImportCertificateErrorKind::InvalidParameterException(inner) => {
                        Error::InvalidParameterException(inner)
                    }
                    crate::error::ImportCertificateErrorKind::InvalidTagException(inner) => {
                        Error::InvalidTagException(inner)
                    }
                    crate::error::ImportCertificateErrorKind::LimitExceededException(inner) => {
                        Error::LimitExceededException(inner)
                    }
                    crate::error::ImportCertificateErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::ImportCertificateErrorKind::TagPolicyException(inner) => {
                        Error::TagPolicyException(inner)
                    }
                    crate::error::ImportCertificateErrorKind::TooManyTagsException(inner) => {
                        Error::TooManyTagsException(inner)
                    }
                    crate::error::ImportCertificateErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListCertificatesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListCertificatesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListCertificatesErrorKind::InvalidArgsException(inner) => {
                        Error::InvalidArgsException(inner)
                    }
                    crate::error::ListCertificatesErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForCertificateError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTagsForCertificateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListTagsForCertificateErrorKind::InvalidArnException(inner) => {
                        Error::InvalidArnException(inner)
                    }
                    crate::error::ListTagsForCertificateErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::ListTagsForCertificateErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutAccountConfigurationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::PutAccountConfigurationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::PutAccountConfigurationErrorKind::AccessDeniedException(
                        inner,
                    ) => Error::AccessDeniedException(inner),
                    crate::error::PutAccountConfigurationErrorKind::ConflictException(inner) => {
                        Error::ConflictException(inner)
                    }
                    crate::error::PutAccountConfigurationErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::PutAccountConfigurationErrorKind::ValidationException(inner) => {
                        Error::ValidationException(inner)
                    }
                    crate::error::PutAccountConfigurationErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RemoveTagsFromCertificateError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RemoveTagsFromCertificateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::RemoveTagsFromCertificateErrorKind::InvalidArnException(
                        inner,
                    ) => Error::InvalidArnException(inner),
                    crate::error::RemoveTagsFromCertificateErrorKind::InvalidParameterException(
                        inner,
                    ) => Error::InvalidParameterException(inner),
                    crate::error::RemoveTagsFromCertificateErrorKind::InvalidTagException(
                        inner,
                    ) => Error::InvalidTagException(inner),
                    crate::error::RemoveTagsFromCertificateErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::RemoveTagsFromCertificateErrorKind::TagPolicyException(inner) => {
                        Error::TagPolicyException(inner)
                    }
                    crate::error::RemoveTagsFromCertificateErrorKind::ThrottlingException(
                        inner,
                    ) => Error::ThrottlingException(inner),
                    crate::error::RemoveTagsFromCertificateErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RenewCertificateError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RenewCertificateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::RenewCertificateErrorKind::InvalidArnException(inner) => {
                        Error::InvalidArnException(inner)
                    }
                    crate::error::RenewCertificateErrorKind::ResourceNotFoundException(inner) => {
                        Error::ResourceNotFoundException(inner)
                    }
                    crate::error::RenewCertificateErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RequestCertificateError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RequestCertificateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context.into_err().kind {
                crate::error::RequestCertificateErrorKind::InvalidArnException(inner) => Error::InvalidArnException(inner),
                crate::error::RequestCertificateErrorKind::InvalidDomainValidationOptionsException(inner) => Error::InvalidDomainValidationOptionsException(inner),
                crate::error::RequestCertificateErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::RequestCertificateErrorKind::InvalidTagException(inner) => Error::InvalidTagException(inner),
                crate::error::RequestCertificateErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::RequestCertificateErrorKind::TagPolicyException(inner) => Error::TagPolicyException(inner),
                crate::error::RequestCertificateErrorKind::TooManyTagsException(inner) => Error::TooManyTagsException(inner),
                crate::error::RequestCertificateErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ResendValidationEmailError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ResendValidationEmailError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context.into_err().kind {
                crate::error::ResendValidationEmailErrorKind::InvalidArnException(inner) => Error::InvalidArnException(inner),
                crate::error::ResendValidationEmailErrorKind::InvalidDomainValidationOptionsException(inner) => Error::InvalidDomainValidationOptionsException(inner),
                crate::error::ResendValidationEmailErrorKind::InvalidStateException(inner) => Error::InvalidStateException(inner),
                crate::error::ResendValidationEmailErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ResendValidationEmailErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateCertificateOptionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateCertificateOptionsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::UpdateCertificateOptionsErrorKind::InvalidArnException(inner) => {
                        Error::InvalidArnException(inner)
                    }
                    crate::error::UpdateCertificateOptionsErrorKind::InvalidStateException(
                        inner,
                    ) => Error::InvalidStateException(inner),
                    crate::error::UpdateCertificateOptionsErrorKind::LimitExceededException(
                        inner,
                    ) => Error::LimitExceededException(inner),
                    crate::error::UpdateCertificateOptionsErrorKind::ResourceNotFoundException(
                        inner,
                    ) => Error::ResourceNotFoundException(inner),
                    crate::error::UpdateCertificateOptionsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
