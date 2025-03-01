// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>The caller's account ID must be the same as the channel owner's account ID.</p>
    ChannelInsufficientPermission(crate::types::error::ChannelInsufficientPermission),
    /// <p>The channel could not be found.</p>
    ChannelNotFound(crate::types::error::ChannelNotFound),
    /// <p>The schema type of the event is not supported.</p>
    ChannelUnsupportedSchema(crate::types::error::ChannelUnsupportedSchema),
    /// <p>Two or more entries in the request have the same event ID.</p>
    DuplicatedAuditEventId(crate::types::error::DuplicatedAuditEventId),
    /// <p>The specified channel ARN is not a valid channel ARN.</p>
    InvalidChannelArn(crate::types::error::InvalidChannelArn),
    /// <p>The operation requested is not supported in this region or account.</p>
    UnsupportedOperationException(crate::types::error::UnsupportedOperationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ChannelInsufficientPermission(inner) => inner.fmt(f),
            Error::ChannelNotFound(inner) => inner.fmt(f),
            Error::ChannelUnsupportedSchema(inner) => inner.fmt(f),
            Error::DuplicatedAuditEventId(inner) => inner.fmt(f),
            Error::InvalidChannelArn(inner) => inner.fmt(f),
            Error::UnsupportedOperationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_audit_events::PutAuditEventsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_http::result::SdkError<
            crate::operation::put_audit_events::PutAuditEventsError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                ::aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err)
                            .clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::put_audit_events::PutAuditEventsError> for Error {
    fn from(err: crate::operation::put_audit_events::PutAuditEventsError) -> Self {
        match err {
            crate::operation::put_audit_events::PutAuditEventsError::ChannelInsufficientPermission(inner) => Error::ChannelInsufficientPermission(inner),
            crate::operation::put_audit_events::PutAuditEventsError::ChannelNotFound(inner) => Error::ChannelNotFound(inner),
            crate::operation::put_audit_events::PutAuditEventsError::ChannelUnsupportedSchema(inner) => Error::ChannelUnsupportedSchema(inner),
            crate::operation::put_audit_events::PutAuditEventsError::DuplicatedAuditEventId(inner) => Error::DuplicatedAuditEventId(inner),
            crate::operation::put_audit_events::PutAuditEventsError::InvalidChannelArn(inner) => Error::InvalidChannelArn(inner),
            crate::operation::put_audit_events::PutAuditEventsError::UnsupportedOperationException(inner) => Error::UnsupportedOperationException(inner),
            crate::operation::put_audit_events::PutAuditEventsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::ChannelInsufficientPermission(inner) => inner.source(),
            Error::ChannelNotFound(inner) => inner.source(),
            Error::ChannelUnsupportedSchema(inner) => inner.source(),
            Error::DuplicatedAuditEventId(inner) => inner.source(),
            Error::InvalidChannelArn(inner) => inner.source(),
            Error::UnsupportedOperationException(inner) => inner.source(),
            Error::Unhandled(inner) => inner.source(),
        }
    }
}
impl ::aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::ChannelInsufficientPermission(e) => e.request_id(),
            Self::ChannelNotFound(e) => e.request_id(),
            Self::ChannelUnsupportedSchema(e) => e.request_id(),
            Self::DuplicatedAuditEventId(e) => e.request_id(),
            Self::InvalidChannelArn(e) => e.request_id(),
            Self::UnsupportedOperationException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
