// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The number of domains has exceeded the allowed threshold for the account.</p>
    DomainLimitExceeded(crate::error::DomainLimitExceeded),
    /// <p>The request is already in progress for the domain.</p>
    DuplicateRequest(crate::error::DuplicateRequest),
    /// <p>The requested item is not acceptable. For example, for APIs that accept a domain name, the request might specify a domain name that doesn't belong to the account that submitted the request. For <code>AcceptDomainTransferFromAnotherAwsAccount</code>, the password might be invalid.</p>
    InvalidInput(crate::error::InvalidInput),
    /// <p>The number of operations or jobs running exceeded the allowed threshold for the account.</p>
    OperationLimitExceeded(crate::error::OperationLimitExceeded),
    /// <p>The top-level domain does not support this operation.</p>
    TldRulesViolation(crate::error::TldRulesViolation),
    /// <p>Amazon Route 53 does not support this top-level domain (TLD).</p>
    UnsupportedTld(crate::error::UnsupportedTld),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DomainLimitExceeded(inner) => inner.fmt(f),
            Error::DuplicateRequest(inner) => inner.fmt(f),
            Error::InvalidInput(inner) => inner.fmt(f),
            Error::OperationLimitExceeded(inner) => inner.fmt(f),
            Error::TldRulesViolation(inner) => inner.fmt(f),
            Error::UnsupportedTld(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::AcceptDomainTransferFromAnotherAwsAccountError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::AcceptDomainTransferFromAnotherAwsAccountError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context.into_err().kind {
                crate::error::AcceptDomainTransferFromAnotherAwsAccountErrorKind::DomainLimitExceeded(inner) => Error::DomainLimitExceeded(inner),
                crate::error::AcceptDomainTransferFromAnotherAwsAccountErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
                crate::error::AcceptDomainTransferFromAnotherAwsAccountErrorKind::OperationLimitExceeded(inner) => Error::OperationLimitExceeded(inner),
                crate::error::AcceptDomainTransferFromAnotherAwsAccountErrorKind::UnsupportedTld(inner) => Error::UnsupportedTld(inner),
                crate::error::AcceptDomainTransferFromAnotherAwsAccountErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::CancelDomainTransferToAnotherAwsAccountError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::CancelDomainTransferToAnotherAwsAccountError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context.into_err().kind {
                crate::error::CancelDomainTransferToAnotherAwsAccountErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
                crate::error::CancelDomainTransferToAnotherAwsAccountErrorKind::OperationLimitExceeded(inner) => Error::OperationLimitExceeded(inner),
                crate::error::CancelDomainTransferToAnotherAwsAccountErrorKind::UnsupportedTld(inner) => Error::UnsupportedTld(inner),
                crate::error::CancelDomainTransferToAnotherAwsAccountErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CheckDomainAvailabilityError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CheckDomainAvailabilityError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::CheckDomainAvailabilityErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::CheckDomainAvailabilityErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::CheckDomainAvailabilityErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CheckDomainTransferabilityError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CheckDomainTransferabilityError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::CheckDomainTransferabilityErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::CheckDomainTransferabilityErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::CheckDomainTransferabilityErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteDomainError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteDomainError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DeleteDomainErrorKind::DuplicateRequest(inner) => {
                        Error::DuplicateRequest(inner)
                    }
                    crate::error::DeleteDomainErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::DeleteDomainErrorKind::TldRulesViolation(inner) => {
                        Error::TldRulesViolation(inner)
                    }
                    crate::error::DeleteDomainErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::DeleteDomainErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteTagsForDomainError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteTagsForDomainError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DeleteTagsForDomainErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::DeleteTagsForDomainErrorKind::OperationLimitExceeded(inner) => {
                        Error::OperationLimitExceeded(inner)
                    }
                    crate::error::DeleteTagsForDomainErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::DeleteTagsForDomainErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisableDomainAutoRenewError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DisableDomainAutoRenewError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DisableDomainAutoRenewErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::DisableDomainAutoRenewErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::DisableDomainAutoRenewErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisableDomainTransferLockError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DisableDomainTransferLockError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::DisableDomainTransferLockErrorKind::DuplicateRequest(inner) => {
                        Error::DuplicateRequest(inner)
                    }
                    crate::error::DisableDomainTransferLockErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::DisableDomainTransferLockErrorKind::OperationLimitExceeded(
                        inner,
                    ) => Error::OperationLimitExceeded(inner),
                    crate::error::DisableDomainTransferLockErrorKind::TldRulesViolation(inner) => {
                        Error::TldRulesViolation(inner)
                    }
                    crate::error::DisableDomainTransferLockErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::DisableDomainTransferLockErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::EnableDomainAutoRenewError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::EnableDomainAutoRenewError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::EnableDomainAutoRenewErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::EnableDomainAutoRenewErrorKind::TldRulesViolation(inner) => {
                        Error::TldRulesViolation(inner)
                    }
                    crate::error::EnableDomainAutoRenewErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::EnableDomainAutoRenewErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::EnableDomainTransferLockError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::EnableDomainTransferLockError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::EnableDomainTransferLockErrorKind::DuplicateRequest(inner) => {
                        Error::DuplicateRequest(inner)
                    }
                    crate::error::EnableDomainTransferLockErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::EnableDomainTransferLockErrorKind::OperationLimitExceeded(
                        inner,
                    ) => Error::OperationLimitExceeded(inner),
                    crate::error::EnableDomainTransferLockErrorKind::TldRulesViolation(inner) => {
                        Error::TldRulesViolation(inner)
                    }
                    crate::error::EnableDomainTransferLockErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::EnableDomainTransferLockErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetContactReachabilityStatusError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetContactReachabilityStatusError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetContactReachabilityStatusErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::GetContactReachabilityStatusErrorKind::OperationLimitExceeded(
                        inner,
                    ) => Error::OperationLimitExceeded(inner),
                    crate::error::GetContactReachabilityStatusErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::GetContactReachabilityStatusErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetDomainDetailError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetDomainDetailError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetDomainDetailErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::GetDomainDetailErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::GetDomainDetailErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetDomainSuggestionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetDomainSuggestionsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetDomainSuggestionsErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::GetDomainSuggestionsErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::GetDomainSuggestionsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetOperationDetailError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetOperationDetailError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::GetOperationDetailErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::GetOperationDetailErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListDomainsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListDomainsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListDomainsErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::ListDomainsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListOperationsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListOperationsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListOperationsErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::ListOperationsErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListPricesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListPricesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListPricesErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::ListPricesErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::ListPricesErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForDomainError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTagsForDomainError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ListTagsForDomainErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::ListTagsForDomainErrorKind::OperationLimitExceeded(inner) => {
                        Error::OperationLimitExceeded(inner)
                    }
                    crate::error::ListTagsForDomainErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::ListTagsForDomainErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RegisterDomainError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::RegisterDomainError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::RegisterDomainErrorKind::DomainLimitExceeded(inner) => {
                        Error::DomainLimitExceeded(inner)
                    }
                    crate::error::RegisterDomainErrorKind::DuplicateRequest(inner) => {
                        Error::DuplicateRequest(inner)
                    }
                    crate::error::RegisterDomainErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::RegisterDomainErrorKind::OperationLimitExceeded(inner) => {
                        Error::OperationLimitExceeded(inner)
                    }
                    crate::error::RegisterDomainErrorKind::TldRulesViolation(inner) => {
                        Error::TldRulesViolation(inner)
                    }
                    crate::error::RegisterDomainErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::RegisterDomainErrorKind::Unhandled(inner) => {
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
            crate::error::RejectDomainTransferFromAnotherAwsAccountError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::RejectDomainTransferFromAnotherAwsAccountError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context.into_err().kind {
                crate::error::RejectDomainTransferFromAnotherAwsAccountErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
                crate::error::RejectDomainTransferFromAnotherAwsAccountErrorKind::OperationLimitExceeded(inner) => Error::OperationLimitExceeded(inner),
                crate::error::RejectDomainTransferFromAnotherAwsAccountErrorKind::UnsupportedTld(inner) => Error::UnsupportedTld(inner),
                crate::error::RejectDomainTransferFromAnotherAwsAccountErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RenewDomainError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::RenewDomainError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::RenewDomainErrorKind::DuplicateRequest(inner) => {
                        Error::DuplicateRequest(inner)
                    }
                    crate::error::RenewDomainErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::RenewDomainErrorKind::OperationLimitExceeded(inner) => {
                        Error::OperationLimitExceeded(inner)
                    }
                    crate::error::RenewDomainErrorKind::TldRulesViolation(inner) => {
                        Error::TldRulesViolation(inner)
                    }
                    crate::error::RenewDomainErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::RenewDomainErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::ResendContactReachabilityEmailError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::ResendContactReachabilityEmailError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context
                .into_err()
                .kind
            {
                crate::error::ResendContactReachabilityEmailErrorKind::InvalidInput(inner) => {
                    Error::InvalidInput(inner)
                }
                crate::error::ResendContactReachabilityEmailErrorKind::OperationLimitExceeded(
                    inner,
                ) => Error::OperationLimitExceeded(inner),
                crate::error::ResendContactReachabilityEmailErrorKind::UnsupportedTld(inner) => {
                    Error::UnsupportedTld(inner)
                }
                crate::error::ResendContactReachabilityEmailErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RetrieveDomainAuthCodeError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RetrieveDomainAuthCodeError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::RetrieveDomainAuthCodeErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::RetrieveDomainAuthCodeErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::RetrieveDomainAuthCodeErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TransferDomainError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TransferDomainError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::TransferDomainErrorKind::DomainLimitExceeded(inner) => {
                        Error::DomainLimitExceeded(inner)
                    }
                    crate::error::TransferDomainErrorKind::DuplicateRequest(inner) => {
                        Error::DuplicateRequest(inner)
                    }
                    crate::error::TransferDomainErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::TransferDomainErrorKind::OperationLimitExceeded(inner) => {
                        Error::OperationLimitExceeded(inner)
                    }
                    crate::error::TransferDomainErrorKind::TldRulesViolation(inner) => {
                        Error::TldRulesViolation(inner)
                    }
                    crate::error::TransferDomainErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::TransferDomainErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::TransferDomainToAnotherAwsAccountError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::TransferDomainToAnotherAwsAccountError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => match context.into_err().kind {
                crate::error::TransferDomainToAnotherAwsAccountErrorKind::DuplicateRequest(inner) => Error::DuplicateRequest(inner),
                crate::error::TransferDomainToAnotherAwsAccountErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
                crate::error::TransferDomainToAnotherAwsAccountErrorKind::OperationLimitExceeded(inner) => Error::OperationLimitExceeded(inner),
                crate::error::TransferDomainToAnotherAwsAccountErrorKind::UnsupportedTld(inner) => Error::UnsupportedTld(inner),
                crate::error::TransferDomainToAnotherAwsAccountErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateDomainContactError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateDomainContactError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::UpdateDomainContactErrorKind::DuplicateRequest(inner) => {
                        Error::DuplicateRequest(inner)
                    }
                    crate::error::UpdateDomainContactErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::UpdateDomainContactErrorKind::OperationLimitExceeded(inner) => {
                        Error::OperationLimitExceeded(inner)
                    }
                    crate::error::UpdateDomainContactErrorKind::TldRulesViolation(inner) => {
                        Error::TldRulesViolation(inner)
                    }
                    crate::error::UpdateDomainContactErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::UpdateDomainContactErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateDomainContactPrivacyError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateDomainContactPrivacyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::UpdateDomainContactPrivacyErrorKind::DuplicateRequest(inner) => {
                        Error::DuplicateRequest(inner)
                    }
                    crate::error::UpdateDomainContactPrivacyErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::UpdateDomainContactPrivacyErrorKind::OperationLimitExceeded(
                        inner,
                    ) => Error::OperationLimitExceeded(inner),
                    crate::error::UpdateDomainContactPrivacyErrorKind::TldRulesViolation(inner) => {
                        Error::TldRulesViolation(inner)
                    }
                    crate::error::UpdateDomainContactPrivacyErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::UpdateDomainContactPrivacyErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateDomainNameserversError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateDomainNameserversError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::UpdateDomainNameserversErrorKind::DuplicateRequest(inner) => {
                        Error::DuplicateRequest(inner)
                    }
                    crate::error::UpdateDomainNameserversErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::UpdateDomainNameserversErrorKind::OperationLimitExceeded(
                        inner,
                    ) => Error::OperationLimitExceeded(inner),
                    crate::error::UpdateDomainNameserversErrorKind::TldRulesViolation(inner) => {
                        Error::TldRulesViolation(inner)
                    }
                    crate::error::UpdateDomainNameserversErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::UpdateDomainNameserversErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateTagsForDomainError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateTagsForDomainError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::UpdateTagsForDomainErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::UpdateTagsForDomainErrorKind::OperationLimitExceeded(inner) => {
                        Error::OperationLimitExceeded(inner)
                    }
                    crate::error::UpdateTagsForDomainErrorKind::UnsupportedTld(inner) => {
                        Error::UnsupportedTld(inner)
                    }
                    crate::error::UpdateTagsForDomainErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ViewBillingError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ViewBillingError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ViewBillingErrorKind::InvalidInput(inner) => {
                        Error::InvalidInput(inner)
                    }
                    crate::error::ViewBillingErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
