// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl UpdateCertificateInput {
    /// Consumes the builder and constructs an Operation<[`UpdateCertificate`](crate::operation::update_certificate::UpdateCertificate)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::update_certificate::UpdateCertificate,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(_config.use_dual_stack)
            .set_use_fips(_config.use_fips)
            .set_endpoint(_config.endpoint_url.clone())
            .build()
            .map_err(|err| {
                ::aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            ::std::result::Result::Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                ::std::option::Option::Some(params),
            ),
            ::std::result::Result::Err(e) => {
                (::std::result::Result::Err(e), ::std::option::Option::None)
            }
        };
        let mut request = {
            fn uri_base(
                _input: &crate::operation::update_certificate::UpdateCertificateInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                let input_1 = &_input.certificate_id;
                let input_1 = input_1.as_ref().ok_or_else(|| {
                    ::aws_smithy_http::operation::error::BuildError::missing_field(
                        "certificate_id",
                        "cannot be empty or unset",
                    )
                })?;
                let certificate_id = ::aws_smithy_http::label::fmt_string(
                    input_1,
                    ::aws_smithy_http::label::EncodingStrategy::Default,
                );
                if certificate_id.is_empty() {
                    return ::std::result::Result::Err(
                        ::aws_smithy_http::operation::error::BuildError::missing_field(
                            "certificate_id",
                            "cannot be empty or unset",
                        ),
                    );
                }
                ::std::write!(
                    output,
                    "/certificates/{certificateId}",
                    certificateId = certificate_id
                )
                .expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            fn uri_query(
                _input: &crate::operation::update_certificate::UpdateCertificateInput,
                mut output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                let mut query = ::aws_smithy_http::query::Writer::new(output);
                let inner_2 = &_input.new_status;
                let inner_2 = inner_2.as_ref().ok_or_else(|| {
                    ::aws_smithy_http::operation::error::BuildError::missing_field(
                        "new_status",
                        "cannot be empty or unset",
                    )
                })?;
                query.push_kv("newStatus", &::aws_smithy_http::query::fmt_string(&inner_2));
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::update_certificate::UpdateCertificateInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<
                ::http::request::Builder,
                ::aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("PUT").uri(uri))
            }
            let mut builder = update_http_builder(&self, ::http::request::Builder::new())?;
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from("");
        let request = request.body(body).expect("should be valid request");
        let mut request = ::aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let ::std::option::Option::Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(::aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = ::aws_http::user_agent::AwsUserAgent::new_from_environment(
            ::aws_types::os_shim_internal::Env::real(),
            crate::meta::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = ::aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(::aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(::aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        ::aws_http::auth::set_credentials_cache(
            &mut request.properties_mut(),
            _config.credentials_cache.clone(),
        );
        let op = ::aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::update_certificate::UpdateCertificate::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "UpdateCertificate",
            "iot",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `UpdateCertificate`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct UpdateCertificate;
impl UpdateCertificate {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for UpdateCertificate {
    type Output = ::std::result::Result<
        crate::operation::update_certificate::UpdateCertificateOutput,
        crate::operation::update_certificate::UpdateCertificateError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_update_certificate::de_update_certificate_http_error(
                status, headers, body,
            )
        } else {
            crate::protocol_serde::shape_update_certificate::de_update_certificate_http_response_with_props(status, headers, body)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type UpdateCertificateErrorKind = UpdateCertificateError;
/// Error type for the `UpdateCertificateError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum UpdateCertificateError {
    /// <p>The certificate operation is not allowed.</p>
    CertificateStateException(crate::types::error::CertificateStateException),
    /// <p>An unexpected error has occurred.</p>
    InternalFailureException(crate::types::error::InternalFailureException),
    /// <p>The request is not valid.</p>
    InvalidRequestException(crate::types::error::InvalidRequestException),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailableException(crate::types::error::ServiceUnavailableException),
    /// <p>The rate exceeds the limit.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>You are not authorized to perform this operation.</p>
    UnauthorizedException(crate::types::error::UnauthorizedException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for UpdateCertificateError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<
            dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
        >,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled({
            let mut builder = ::aws_smithy_types::error::Unhandled::builder().source(source);
            builder.set_meta(meta);
            builder.build()
        })
    }
}
impl ::std::fmt::Display for UpdateCertificateError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::CertificateStateException(_inner) => _inner.fmt(f),
            Self::InternalFailureException(_inner) => _inner.fmt(f),
            Self::InvalidRequestException(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::ServiceUnavailableException(_inner) => _inner.fmt(f),
            Self::ThrottlingException(_inner) => _inner.fmt(f),
            Self::UnauthorizedException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for UpdateCertificateError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::CertificateStateException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalFailureException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidRequestException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ResourceNotFoundException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ServiceUnavailableException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ThrottlingException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UnauthorizedException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId
    for crate::operation::update_certificate::UpdateCertificateError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for UpdateCertificateError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl UpdateCertificateError {
    /// Creates the `UpdateCertificateError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<
            ::std::boxed::Box<
                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
            >,
        >,
    ) -> Self {
        Self::Unhandled(
            ::aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `UpdateCertificateError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(
            ::aws_smithy_types::error::Unhandled::builder()
                .source(err.clone())
                .meta(err)
                .build(),
        )
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::CertificateStateException(e) => e.meta(),
            Self::InternalFailureException(e) => e.meta(),
            Self::InvalidRequestException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::ServiceUnavailableException(e) => e.meta(),
            Self::ThrottlingException(e) => e.meta(),
            Self::UnauthorizedException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `UpdateCertificateError::CertificateStateException`.
    pub fn is_certificate_state_exception(&self) -> bool {
        matches!(self, Self::CertificateStateException(_))
    }
    /// Returns `true` if the error kind is `UpdateCertificateError::InternalFailureException`.
    pub fn is_internal_failure_exception(&self) -> bool {
        matches!(self, Self::InternalFailureException(_))
    }
    /// Returns `true` if the error kind is `UpdateCertificateError::InvalidRequestException`.
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(self, Self::InvalidRequestException(_))
    }
    /// Returns `true` if the error kind is `UpdateCertificateError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `UpdateCertificateError::ServiceUnavailableException`.
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(self, Self::ServiceUnavailableException(_))
    }
    /// Returns `true` if the error kind is `UpdateCertificateError::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(self, Self::ThrottlingException(_))
    }
    /// Returns `true` if the error kind is `UpdateCertificateError::UnauthorizedException`.
    pub fn is_unauthorized_exception(&self) -> bool {
        matches!(self, Self::UnauthorizedException(_))
    }
}
impl ::std::error::Error for UpdateCertificateError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::CertificateStateException(_inner) => ::std::option::Option::Some(_inner),
            Self::InternalFailureException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidRequestException(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::ServiceUnavailableException(_inner) => ::std::option::Option::Some(_inner),
            Self::ThrottlingException(_inner) => ::std::option::Option::Some(_inner),
            Self::UnauthorizedException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::update_certificate::_update_certificate_output::UpdateCertificateOutput;

pub use crate::operation::update_certificate::_update_certificate_input::UpdateCertificateInput;

mod _update_certificate_input;

mod _update_certificate_output;

/// Builders
pub mod builders;
