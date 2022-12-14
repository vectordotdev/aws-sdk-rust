// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Exception thrown when the customer does not have a valid subscription for the product.</p>
    CustomerNotEntitledException(crate::error::CustomerNotEntitledException),
    /// <p>The API is disabled in the Region.</p>
    DisabledApiException(crate::error::DisabledApiException),
    /// <p>A metering record has already been emitted by the same EC2 instance, ECS task, or EKS pod for the given {<code>usageDimension</code>, <code>timestamp</code>} with a different <code>usageQuantity</code>.</p>
    DuplicateRequestException(crate::error::DuplicateRequestException),
    /// <p>The submitted registration token has expired. This can happen if the buyer's browser takes too long to redirect to your page, the buyer has resubmitted the registration token, or your application has held on to the registration token for too long. Your SaaS registration website should redeem this token as soon as it is submitted by the buyer's browser.</p>
    ExpiredTokenException(crate::error::ExpiredTokenException),
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceErrorException(crate::error::InternalServiceErrorException),
    /// <p>You have metered usage for a <code>CustomerIdentifier</code> that does not exist.</p>
    InvalidCustomerIdentifierException(crate::error::InvalidCustomerIdentifierException),
    /// <p>The endpoint being called is in a AWS Region different from your EC2 instance, ECS task, or EKS pod. The Region of the Metering Service endpoint and the AWS Region of the resource must match.</p>
    InvalidEndpointRegionException(crate::error::InvalidEndpointRegionException),
    /// <p>The product code passed does not match the product code used for publishing the product.</p>
    InvalidProductCodeException(crate::error::InvalidProductCodeException),
    /// <p>Public Key version is invalid.</p>
    InvalidPublicKeyVersionException(crate::error::InvalidPublicKeyVersionException),
    /// <p> <code>RegisterUsage</code> must be called in the same AWS Region the ECS task was launched in. This prevents a container from hardcoding a Region (e.g. withRegion(“us-east-1”) when calling <code>RegisterUsage</code>.</p>
    InvalidRegionException(crate::error::InvalidRegionException),
    /// <p>The tag is invalid, or the number of tags is greater than 5.</p>
    InvalidTagException(crate::error::InvalidTagException),
    /// <p>Registration token is invalid.</p>
    InvalidTokenException(crate::error::InvalidTokenException),
    /// <p>The usage allocation objects are invalid, or the number of allocations is greater than 500 for a single usage record.</p>
    InvalidUsageAllocationsException(crate::error::InvalidUsageAllocationsException),
    /// <p>The usage dimension does not match one of the <code>UsageDimensions</code> associated with products.</p>
    InvalidUsageDimensionException(crate::error::InvalidUsageDimensionException),
    /// <p>AWS Marketplace does not support metering usage from the underlying platform. Currently, Amazon ECS, Amazon EKS, and AWS Fargate are supported.</p>
    PlatformNotSupportedException(crate::error::PlatformNotSupportedException),
    /// <p>The calls to the API are throttled.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The <code>timestamp</code> value passed in the <code>UsageRecord</code> is out of allowed range.</p>
    /// <p>For <code>BatchMeterUsage</code>, if any of the records are outside of the allowed range, the entire batch is not processed. You must remove invalid records and try again.</p>
    TimestampOutOfBoundsException(crate::error::TimestampOutOfBoundsException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CustomerNotEntitledException(inner) => inner.fmt(f),
            Error::DisabledApiException(inner) => inner.fmt(f),
            Error::DuplicateRequestException(inner) => inner.fmt(f),
            Error::ExpiredTokenException(inner) => inner.fmt(f),
            Error::InternalServiceErrorException(inner) => inner.fmt(f),
            Error::InvalidCustomerIdentifierException(inner) => inner.fmt(f),
            Error::InvalidEndpointRegionException(inner) => inner.fmt(f),
            Error::InvalidProductCodeException(inner) => inner.fmt(f),
            Error::InvalidPublicKeyVersionException(inner) => inner.fmt(f),
            Error::InvalidRegionException(inner) => inner.fmt(f),
            Error::InvalidTagException(inner) => inner.fmt(f),
            Error::InvalidTokenException(inner) => inner.fmt(f),
            Error::InvalidUsageAllocationsException(inner) => inner.fmt(f),
            Error::InvalidUsageDimensionException(inner) => inner.fmt(f),
            Error::PlatformNotSupportedException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::TimestampOutOfBoundsException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BatchMeterUsageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::BatchMeterUsageError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::BatchMeterUsageErrorKind::DisabledApiException(inner) => {
                        Error::DisabledApiException(inner)
                    }
                    crate::error::BatchMeterUsageErrorKind::InternalServiceErrorException(
                        inner,
                    ) => Error::InternalServiceErrorException(inner),
                    crate::error::BatchMeterUsageErrorKind::InvalidCustomerIdentifierException(
                        inner,
                    ) => Error::InvalidCustomerIdentifierException(inner),
                    crate::error::BatchMeterUsageErrorKind::InvalidProductCodeException(inner) => {
                        Error::InvalidProductCodeException(inner)
                    }
                    crate::error::BatchMeterUsageErrorKind::InvalidTagException(inner) => {
                        Error::InvalidTagException(inner)
                    }
                    crate::error::BatchMeterUsageErrorKind::InvalidUsageAllocationsException(
                        inner,
                    ) => Error::InvalidUsageAllocationsException(inner),
                    crate::error::BatchMeterUsageErrorKind::InvalidUsageDimensionException(
                        inner,
                    ) => Error::InvalidUsageDimensionException(inner),
                    crate::error::BatchMeterUsageErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::BatchMeterUsageErrorKind::TimestampOutOfBoundsException(
                        inner,
                    ) => Error::TimestampOutOfBoundsException(inner),
                    crate::error::BatchMeterUsageErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::MeterUsageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::MeterUsageError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::MeterUsageErrorKind::CustomerNotEntitledException(inner) => {
                        Error::CustomerNotEntitledException(inner)
                    }
                    crate::error::MeterUsageErrorKind::DuplicateRequestException(inner) => {
                        Error::DuplicateRequestException(inner)
                    }
                    crate::error::MeterUsageErrorKind::InternalServiceErrorException(inner) => {
                        Error::InternalServiceErrorException(inner)
                    }
                    crate::error::MeterUsageErrorKind::InvalidEndpointRegionException(inner) => {
                        Error::InvalidEndpointRegionException(inner)
                    }
                    crate::error::MeterUsageErrorKind::InvalidProductCodeException(inner) => {
                        Error::InvalidProductCodeException(inner)
                    }
                    crate::error::MeterUsageErrorKind::InvalidTagException(inner) => {
                        Error::InvalidTagException(inner)
                    }
                    crate::error::MeterUsageErrorKind::InvalidUsageAllocationsException(inner) => {
                        Error::InvalidUsageAllocationsException(inner)
                    }
                    crate::error::MeterUsageErrorKind::InvalidUsageDimensionException(inner) => {
                        Error::InvalidUsageDimensionException(inner)
                    }
                    crate::error::MeterUsageErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::MeterUsageErrorKind::TimestampOutOfBoundsException(inner) => {
                        Error::TimestampOutOfBoundsException(inner)
                    }
                    crate::error::MeterUsageErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RegisterUsageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::RegisterUsageError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::RegisterUsageErrorKind::CustomerNotEntitledException(inner) => {
                        Error::CustomerNotEntitledException(inner)
                    }
                    crate::error::RegisterUsageErrorKind::DisabledApiException(inner) => {
                        Error::DisabledApiException(inner)
                    }
                    crate::error::RegisterUsageErrorKind::InternalServiceErrorException(inner) => {
                        Error::InternalServiceErrorException(inner)
                    }
                    crate::error::RegisterUsageErrorKind::InvalidProductCodeException(inner) => {
                        Error::InvalidProductCodeException(inner)
                    }
                    crate::error::RegisterUsageErrorKind::InvalidPublicKeyVersionException(
                        inner,
                    ) => Error::InvalidPublicKeyVersionException(inner),
                    crate::error::RegisterUsageErrorKind::InvalidRegionException(inner) => {
                        Error::InvalidRegionException(inner)
                    }
                    crate::error::RegisterUsageErrorKind::PlatformNotSupportedException(inner) => {
                        Error::PlatformNotSupportedException(inner)
                    }
                    crate::error::RegisterUsageErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::RegisterUsageErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ResolveCustomerError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ResolveCustomerError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                match context.into_err().kind {
                    crate::error::ResolveCustomerErrorKind::DisabledApiException(inner) => {
                        Error::DisabledApiException(inner)
                    }
                    crate::error::ResolveCustomerErrorKind::ExpiredTokenException(inner) => {
                        Error::ExpiredTokenException(inner)
                    }
                    crate::error::ResolveCustomerErrorKind::InternalServiceErrorException(
                        inner,
                    ) => Error::InternalServiceErrorException(inner),
                    crate::error::ResolveCustomerErrorKind::InvalidTokenException(inner) => {
                        Error::InvalidTokenException(inner)
                    }
                    crate::error::ResolveCustomerErrorKind::ThrottlingException(inner) => {
                        Error::ThrottlingException(inner)
                    }
                    crate::error::ResolveCustomerErrorKind::Unhandled(inner) => {
                        Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                    }
                }
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
