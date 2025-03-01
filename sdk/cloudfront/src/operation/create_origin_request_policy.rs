// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl CreateOriginRequestPolicyInput {
    /// Consumes the builder and constructs an Operation<[`CreateOriginRequestPolicy`](crate::operation::create_origin_request_policy::CreateOriginRequestPolicy)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::create_origin_request_policy::CreateOriginRequestPolicy,
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
                _input: &crate::operation::create_origin_request_policy::CreateOriginRequestPolicyInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/2020-05-31/origin-request-policy")
                    .expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_origin_request_policy::CreateOriginRequestPolicyInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<
                ::http::request::Builder,
                ::aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, ::http::request::Builder::new())?;
            builder = ::aws_smithy_http::header::set_request_header_if_absent(
                builder,
                ::http::header::CONTENT_TYPE,
                "application/xml",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_create_origin_request_policy_input::ser_origin_request_policy_config_http_payload(& self.origin_request_policy_config)?
        );
        if let ::std::option::Option::Some(content_length) = body.content_length() {
            request = ::aws_smithy_http::header::set_request_header_if_absent(
                request,
                ::http::header::CONTENT_LENGTH,
                content_length,
            );
        }
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
            crate::operation::create_origin_request_policy::CreateOriginRequestPolicy::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "CreateOriginRequestPolicy",
            "cloudfront",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `CreateOriginRequestPolicy`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct CreateOriginRequestPolicy;
impl CreateOriginRequestPolicy {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for CreateOriginRequestPolicy {
    type Output = ::std::result::Result<
        crate::operation::create_origin_request_policy::CreateOriginRequestPolicyOutput,
        crate::operation::create_origin_request_policy::CreateOriginRequestPolicyError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 201 {
            crate::protocol_serde::shape_create_origin_request_policy::de_create_origin_request_policy_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_create_origin_request_policy::de_create_origin_request_policy_http_response_with_props(status, headers, body)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type CreateOriginRequestPolicyErrorKind = CreateOriginRequestPolicyError;
/// Error type for the `CreateOriginRequestPolicyError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum CreateOriginRequestPolicyError {
    /// <p>Access denied.</p>
    AccessDenied(crate::types::error::AccessDenied),
    /// <p>The value of <code>Quantity</code> and the size of <code>Items</code> don't match.</p>
    InconsistentQuantities(crate::types::error::InconsistentQuantities),
    /// <p>An argument is invalid.</p>
    InvalidArgument(crate::types::error::InvalidArgument),
    /// <p>An origin request policy with this name already exists. You must provide a unique name. To modify an existing origin request policy, use <code>UpdateOriginRequestPolicy</code>.</p>
    OriginRequestPolicyAlreadyExists(crate::types::error::OriginRequestPolicyAlreadyExists),
    /// <p>The number of cookies in the origin request policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyCookiesInOriginRequestPolicy(crate::types::error::TooManyCookiesInOriginRequestPolicy),
    /// <p>The number of headers in the origin request policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyHeadersInOriginRequestPolicy(crate::types::error::TooManyHeadersInOriginRequestPolicy),
    /// <p>You have reached the maximum number of origin request policies for this Amazon Web Services account. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyOriginRequestPolicies(crate::types::error::TooManyOriginRequestPolicies),
    /// <p>The number of query strings in the origin request policy exceeds the maximum. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-limits.html">Quotas</a> (formerly known as limits) in the <i>Amazon CloudFront Developer Guide</i>.</p>
    TooManyQueryStringsInOriginRequestPolicy(
        crate::types::error::TooManyQueryStringsInOriginRequestPolicy,
    ),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for CreateOriginRequestPolicyError {
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
impl ::std::fmt::Display for CreateOriginRequestPolicyError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::AccessDenied(_inner) => _inner.fmt(f),
            Self::InconsistentQuantities(_inner) => _inner.fmt(f),
            Self::InvalidArgument(_inner) => _inner.fmt(f),
            Self::OriginRequestPolicyAlreadyExists(_inner) => _inner.fmt(f),
            Self::TooManyCookiesInOriginRequestPolicy(_inner) => _inner.fmt(f),
            Self::TooManyHeadersInOriginRequestPolicy(_inner) => _inner.fmt(f),
            Self::TooManyOriginRequestPolicies(_inner) => _inner.fmt(f),
            Self::TooManyQueryStringsInOriginRequestPolicy(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateOriginRequestPolicyError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::AccessDenied(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InconsistentQuantities(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidArgument(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::OriginRequestPolicyAlreadyExists(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TooManyCookiesInOriginRequestPolicy(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TooManyHeadersInOriginRequestPolicy(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TooManyOriginRequestPolicies(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TooManyQueryStringsInOriginRequestPolicy(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId
    for crate::operation::create_origin_request_policy::CreateOriginRequestPolicyError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for CreateOriginRequestPolicyError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl CreateOriginRequestPolicyError {
    /// Creates the `CreateOriginRequestPolicyError::Unhandled` variant from any error type.
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

    /// Creates the `CreateOriginRequestPolicyError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::AccessDenied(e) => e.meta(),
            Self::InconsistentQuantities(e) => e.meta(),
            Self::InvalidArgument(e) => e.meta(),
            Self::OriginRequestPolicyAlreadyExists(e) => e.meta(),
            Self::TooManyCookiesInOriginRequestPolicy(e) => e.meta(),
            Self::TooManyHeadersInOriginRequestPolicy(e) => e.meta(),
            Self::TooManyOriginRequestPolicies(e) => e.meta(),
            Self::TooManyQueryStringsInOriginRequestPolicy(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `CreateOriginRequestPolicyError::AccessDenied`.
    pub fn is_access_denied(&self) -> bool {
        matches!(self, Self::AccessDenied(_))
    }
    /// Returns `true` if the error kind is `CreateOriginRequestPolicyError::InconsistentQuantities`.
    pub fn is_inconsistent_quantities(&self) -> bool {
        matches!(self, Self::InconsistentQuantities(_))
    }
    /// Returns `true` if the error kind is `CreateOriginRequestPolicyError::InvalidArgument`.
    pub fn is_invalid_argument(&self) -> bool {
        matches!(self, Self::InvalidArgument(_))
    }
    /// Returns `true` if the error kind is `CreateOriginRequestPolicyError::OriginRequestPolicyAlreadyExists`.
    pub fn is_origin_request_policy_already_exists(&self) -> bool {
        matches!(self, Self::OriginRequestPolicyAlreadyExists(_))
    }
    /// Returns `true` if the error kind is `CreateOriginRequestPolicyError::TooManyCookiesInOriginRequestPolicy`.
    pub fn is_too_many_cookies_in_origin_request_policy(&self) -> bool {
        matches!(self, Self::TooManyCookiesInOriginRequestPolicy(_))
    }
    /// Returns `true` if the error kind is `CreateOriginRequestPolicyError::TooManyHeadersInOriginRequestPolicy`.
    pub fn is_too_many_headers_in_origin_request_policy(&self) -> bool {
        matches!(self, Self::TooManyHeadersInOriginRequestPolicy(_))
    }
    /// Returns `true` if the error kind is `CreateOriginRequestPolicyError::TooManyOriginRequestPolicies`.
    pub fn is_too_many_origin_request_policies(&self) -> bool {
        matches!(self, Self::TooManyOriginRequestPolicies(_))
    }
    /// Returns `true` if the error kind is `CreateOriginRequestPolicyError::TooManyQueryStringsInOriginRequestPolicy`.
    pub fn is_too_many_query_strings_in_origin_request_policy(&self) -> bool {
        matches!(self, Self::TooManyQueryStringsInOriginRequestPolicy(_))
    }
}
impl ::std::error::Error for CreateOriginRequestPolicyError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::AccessDenied(_inner) => ::std::option::Option::Some(_inner),
            Self::InconsistentQuantities(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidArgument(_inner) => ::std::option::Option::Some(_inner),
            Self::OriginRequestPolicyAlreadyExists(_inner) => ::std::option::Option::Some(_inner),
            Self::TooManyCookiesInOriginRequestPolicy(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::TooManyHeadersInOriginRequestPolicy(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::TooManyOriginRequestPolicies(_inner) => ::std::option::Option::Some(_inner),
            Self::TooManyQueryStringsInOriginRequestPolicy(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::create_origin_request_policy::_create_origin_request_policy_output::CreateOriginRequestPolicyOutput;

pub use crate::operation::create_origin_request_policy::_create_origin_request_policy_input::CreateOriginRequestPolicyInput;

mod _create_origin_request_policy_input;

mod _create_origin_request_policy_output;

/// Builders
pub mod builders;
