// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl CreatePullRequestInput {
    /// Consumes the builder and constructs an Operation<[`CreatePullRequest`](crate::operation::create_pull_request::CreatePullRequest)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        mut self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::create_pull_request::CreatePullRequest,
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
        if self.client_request_token.is_none() {
            self.client_request_token =
                ::std::option::Option::Some(_config.make_token.make_idempotency_token());
        }
        let mut request = {
            fn uri_base(
                _input: &crate::operation::create_pull_request::CreatePullRequestInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_pull_request::CreatePullRequestInput,
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
                "application/x-amz-json-1.1",
            );
            builder = ::aws_smithy_http::header::set_request_header_if_absent(
                builder,
                ::http::header::HeaderName::from_static("x-amz-target"),
                "CodeCommit_20150413.CreatePullRequest",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_create_pull_request::ser_create_pull_request_input(&self)?,
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
            crate::operation::create_pull_request::CreatePullRequest::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "CreatePullRequest",
            "codecommit",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `CreatePullRequest`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct CreatePullRequest;
impl CreatePullRequest {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for CreatePullRequest {
    type Output = ::std::result::Result<
        crate::operation::create_pull_request::CreatePullRequestOutput,
        crate::operation::create_pull_request::CreatePullRequestError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_create_pull_request::de_create_pull_request_http_error(
                status, headers, body,
            )
        } else {
            crate::protocol_serde::shape_create_pull_request::de_create_pull_request_http_response_with_props(status, headers, body)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type CreatePullRequestErrorKind = CreatePullRequestError;
/// Error type for the `CreatePullRequestError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum CreatePullRequestError {
    /// <p>A client request token is required. A client request token is an unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request returns information about the initial request that used that token.</p>
    ClientRequestTokenRequiredException(crate::types::error::ClientRequestTokenRequiredException),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailedException(
        crate::types::error::EncryptionIntegrityChecksFailedException,
    ),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDeniedException(crate::types::error::EncryptionKeyAccessDeniedException),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabledException(crate::types::error::EncryptionKeyDisabledException),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFoundException(crate::types::error::EncryptionKeyNotFoundException),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailableException(crate::types::error::EncryptionKeyUnavailableException),
    /// <p>The client request token is not valid. Either the token is not in a valid format, or the token has been used in a previous request and cannot be reused.</p>
    IdempotencyParameterMismatchException(
        crate::types::error::IdempotencyParameterMismatchException,
    ),
    /// <p>The client request token is not valid.</p>
    InvalidClientRequestTokenException(crate::types::error::InvalidClientRequestTokenException),
    /// <p>The pull request description is not valid. Descriptions cannot be more than 1,000 characters.</p>
    InvalidDescriptionException(crate::types::error::InvalidDescriptionException),
    /// <p>The specified reference name format is not valid. Reference names must conform to the Git references format (for example, refs/heads/master). For more information, see <a href="https://git-scm.com/book/en/v2/Git-Internals-Git-References">Git Internals - Git References</a> or consult your Git documentation.</p>
    InvalidReferenceNameException(crate::types::error::InvalidReferenceNameException),
    /// <p>A specified repository name is not valid.</p> <note>
    /// <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p>
    /// </note>
    InvalidRepositoryNameException(crate::types::error::InvalidRepositoryNameException),
    /// <p>The target for the pull request is not valid. A target must contain the full values for the repository name, source branch, and destination branch for the pull request.</p>
    InvalidTargetException(crate::types::error::InvalidTargetException),
    /// <p>The targets for the pull request is not valid or not in a valid format. Targets are a list of target objects. Each target object must contain the full values for the repository name, source branch, and destination branch for a pull request.</p>
    InvalidTargetsException(crate::types::error::InvalidTargetsException),
    /// <p>The title of the pull request is not valid. Pull request titles cannot exceed 100 characters in length.</p>
    InvalidTitleException(crate::types::error::InvalidTitleException),
    /// <p>You cannot create the pull request because the repository has too many open pull requests. The maximum number of open pull requests for a repository is 1,000. Close one or more open pull requests, and then try again.</p>
    MaximumOpenPullRequestsExceededException(
        crate::types::error::MaximumOpenPullRequestsExceededException,
    ),
    /// <p>You cannot include more than one repository in a pull request. Make sure you have specified only one repository name in your request, and then try again.</p>
    MultipleRepositoriesInPullRequestException(
        crate::types::error::MultipleRepositoriesInPullRequestException,
    ),
    /// <p>The specified reference does not exist. You must provide a full commit ID.</p>
    ReferenceDoesNotExistException(crate::types::error::ReferenceDoesNotExistException),
    /// <p>A reference name is required, but none was provided.</p>
    ReferenceNameRequiredException(crate::types::error::ReferenceNameRequiredException),
    /// <p>The specified reference is not a supported type. </p>
    ReferenceTypeNotSupportedException(crate::types::error::ReferenceTypeNotSupportedException),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExistException(crate::types::error::RepositoryDoesNotExistException),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequiredException(crate::types::error::RepositoryNameRequiredException),
    /// <p>The source branch and destination branch for the pull request are the same. You must specify different branches for the source and destination.</p>
    SourceAndDestinationAreSameException(crate::types::error::SourceAndDestinationAreSameException),
    /// <p>A pull request target is required. It cannot be empty or null. A pull request target must contain the full values for the repository name, source branch, and destination branch for the pull request.</p>
    TargetRequiredException(crate::types::error::TargetRequiredException),
    /// <p>An array of target objects is required. It cannot be empty or null.</p>
    TargetsRequiredException(crate::types::error::TargetsRequiredException),
    /// <p>A pull request title is required. It cannot be empty or null.</p>
    TitleRequiredException(crate::types::error::TitleRequiredException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for CreatePullRequestError {
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
impl ::std::fmt::Display for CreatePullRequestError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::ClientRequestTokenRequiredException(_inner) => _inner.fmt(f),
            Self::EncryptionIntegrityChecksFailedException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyAccessDeniedException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyDisabledException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyNotFoundException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyUnavailableException(_inner) => _inner.fmt(f),
            Self::IdempotencyParameterMismatchException(_inner) => _inner.fmt(f),
            Self::InvalidClientRequestTokenException(_inner) => _inner.fmt(f),
            Self::InvalidDescriptionException(_inner) => _inner.fmt(f),
            Self::InvalidReferenceNameException(_inner) => _inner.fmt(f),
            Self::InvalidRepositoryNameException(_inner) => _inner.fmt(f),
            Self::InvalidTargetException(_inner) => _inner.fmt(f),
            Self::InvalidTargetsException(_inner) => _inner.fmt(f),
            Self::InvalidTitleException(_inner) => _inner.fmt(f),
            Self::MaximumOpenPullRequestsExceededException(_inner) => _inner.fmt(f),
            Self::MultipleRepositoriesInPullRequestException(_inner) => _inner.fmt(f),
            Self::ReferenceDoesNotExistException(_inner) => _inner.fmt(f),
            Self::ReferenceNameRequiredException(_inner) => _inner.fmt(f),
            Self::ReferenceTypeNotSupportedException(_inner) => _inner.fmt(f),
            Self::RepositoryDoesNotExistException(_inner) => _inner.fmt(f),
            Self::RepositoryNameRequiredException(_inner) => _inner.fmt(f),
            Self::SourceAndDestinationAreSameException(_inner) => _inner.fmt(f),
            Self::TargetRequiredException(_inner) => _inner.fmt(f),
            Self::TargetsRequiredException(_inner) => _inner.fmt(f),
            Self::TitleRequiredException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CreatePullRequestError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ClientRequestTokenRequiredException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionIntegrityChecksFailedException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyAccessDeniedException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyDisabledException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyNotFoundException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyUnavailableException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::IdempotencyParameterMismatchException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidClientRequestTokenException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidDescriptionException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidReferenceNameException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidRepositoryNameException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidTargetException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidTargetsException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidTitleException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::MaximumOpenPullRequestsExceededException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::MultipleRepositoriesInPullRequestException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ReferenceDoesNotExistException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ReferenceNameRequiredException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ReferenceTypeNotSupportedException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::RepositoryDoesNotExistException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::RepositoryNameRequiredException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::SourceAndDestinationAreSameException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TargetRequiredException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TargetsRequiredException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TitleRequiredException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId
    for crate::operation::create_pull_request::CreatePullRequestError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for CreatePullRequestError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl CreatePullRequestError {
    /// Creates the `CreatePullRequestError::Unhandled` variant from any error type.
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

    /// Creates the `CreatePullRequestError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::ClientRequestTokenRequiredException(e) => e.meta(),
            Self::EncryptionIntegrityChecksFailedException(e) => e.meta(),
            Self::EncryptionKeyAccessDeniedException(e) => e.meta(),
            Self::EncryptionKeyDisabledException(e) => e.meta(),
            Self::EncryptionKeyNotFoundException(e) => e.meta(),
            Self::EncryptionKeyUnavailableException(e) => e.meta(),
            Self::IdempotencyParameterMismatchException(e) => e.meta(),
            Self::InvalidClientRequestTokenException(e) => e.meta(),
            Self::InvalidDescriptionException(e) => e.meta(),
            Self::InvalidReferenceNameException(e) => e.meta(),
            Self::InvalidRepositoryNameException(e) => e.meta(),
            Self::InvalidTargetException(e) => e.meta(),
            Self::InvalidTargetsException(e) => e.meta(),
            Self::InvalidTitleException(e) => e.meta(),
            Self::MaximumOpenPullRequestsExceededException(e) => e.meta(),
            Self::MultipleRepositoriesInPullRequestException(e) => e.meta(),
            Self::ReferenceDoesNotExistException(e) => e.meta(),
            Self::ReferenceNameRequiredException(e) => e.meta(),
            Self::ReferenceTypeNotSupportedException(e) => e.meta(),
            Self::RepositoryDoesNotExistException(e) => e.meta(),
            Self::RepositoryNameRequiredException(e) => e.meta(),
            Self::SourceAndDestinationAreSameException(e) => e.meta(),
            Self::TargetRequiredException(e) => e.meta(),
            Self::TargetsRequiredException(e) => e.meta(),
            Self::TitleRequiredException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::ClientRequestTokenRequiredException`.
    pub fn is_client_request_token_required_exception(&self) -> bool {
        matches!(self, Self::ClientRequestTokenRequiredException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::EncryptionIntegrityChecksFailedException`.
    pub fn is_encryption_integrity_checks_failed_exception(&self) -> bool {
        matches!(self, Self::EncryptionIntegrityChecksFailedException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::EncryptionKeyAccessDeniedException`.
    pub fn is_encryption_key_access_denied_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyAccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::EncryptionKeyDisabledException`.
    pub fn is_encryption_key_disabled_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyDisabledException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::EncryptionKeyNotFoundException`.
    pub fn is_encryption_key_not_found_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyNotFoundException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::EncryptionKeyUnavailableException`.
    pub fn is_encryption_key_unavailable_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyUnavailableException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::IdempotencyParameterMismatchException`.
    pub fn is_idempotency_parameter_mismatch_exception(&self) -> bool {
        matches!(self, Self::IdempotencyParameterMismatchException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::InvalidClientRequestTokenException`.
    pub fn is_invalid_client_request_token_exception(&self) -> bool {
        matches!(self, Self::InvalidClientRequestTokenException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::InvalidDescriptionException`.
    pub fn is_invalid_description_exception(&self) -> bool {
        matches!(self, Self::InvalidDescriptionException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::InvalidReferenceNameException`.
    pub fn is_invalid_reference_name_exception(&self) -> bool {
        matches!(self, Self::InvalidReferenceNameException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::InvalidRepositoryNameException`.
    pub fn is_invalid_repository_name_exception(&self) -> bool {
        matches!(self, Self::InvalidRepositoryNameException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::InvalidTargetException`.
    pub fn is_invalid_target_exception(&self) -> bool {
        matches!(self, Self::InvalidTargetException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::InvalidTargetsException`.
    pub fn is_invalid_targets_exception(&self) -> bool {
        matches!(self, Self::InvalidTargetsException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::InvalidTitleException`.
    pub fn is_invalid_title_exception(&self) -> bool {
        matches!(self, Self::InvalidTitleException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::MaximumOpenPullRequestsExceededException`.
    pub fn is_maximum_open_pull_requests_exceeded_exception(&self) -> bool {
        matches!(self, Self::MaximumOpenPullRequestsExceededException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::MultipleRepositoriesInPullRequestException`.
    pub fn is_multiple_repositories_in_pull_request_exception(&self) -> bool {
        matches!(self, Self::MultipleRepositoriesInPullRequestException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::ReferenceDoesNotExistException`.
    pub fn is_reference_does_not_exist_exception(&self) -> bool {
        matches!(self, Self::ReferenceDoesNotExistException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::ReferenceNameRequiredException`.
    pub fn is_reference_name_required_exception(&self) -> bool {
        matches!(self, Self::ReferenceNameRequiredException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::ReferenceTypeNotSupportedException`.
    pub fn is_reference_type_not_supported_exception(&self) -> bool {
        matches!(self, Self::ReferenceTypeNotSupportedException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::RepositoryDoesNotExistException`.
    pub fn is_repository_does_not_exist_exception(&self) -> bool {
        matches!(self, Self::RepositoryDoesNotExistException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::RepositoryNameRequiredException`.
    pub fn is_repository_name_required_exception(&self) -> bool {
        matches!(self, Self::RepositoryNameRequiredException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::SourceAndDestinationAreSameException`.
    pub fn is_source_and_destination_are_same_exception(&self) -> bool {
        matches!(self, Self::SourceAndDestinationAreSameException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::TargetRequiredException`.
    pub fn is_target_required_exception(&self) -> bool {
        matches!(self, Self::TargetRequiredException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::TargetsRequiredException`.
    pub fn is_targets_required_exception(&self) -> bool {
        matches!(self, Self::TargetsRequiredException(_))
    }
    /// Returns `true` if the error kind is `CreatePullRequestError::TitleRequiredException`.
    pub fn is_title_required_exception(&self) -> bool {
        matches!(self, Self::TitleRequiredException(_))
    }
}
impl ::std::error::Error for CreatePullRequestError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::ClientRequestTokenRequiredException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::EncryptionIntegrityChecksFailedException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::EncryptionKeyAccessDeniedException(_inner) => ::std::option::Option::Some(_inner),
            Self::EncryptionKeyDisabledException(_inner) => ::std::option::Option::Some(_inner),
            Self::EncryptionKeyNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::EncryptionKeyUnavailableException(_inner) => ::std::option::Option::Some(_inner),
            Self::IdempotencyParameterMismatchException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::InvalidClientRequestTokenException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidDescriptionException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidReferenceNameException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidRepositoryNameException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidTargetException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidTargetsException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidTitleException(_inner) => ::std::option::Option::Some(_inner),
            Self::MaximumOpenPullRequestsExceededException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::MultipleRepositoriesInPullRequestException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::ReferenceDoesNotExistException(_inner) => ::std::option::Option::Some(_inner),
            Self::ReferenceNameRequiredException(_inner) => ::std::option::Option::Some(_inner),
            Self::ReferenceTypeNotSupportedException(_inner) => ::std::option::Option::Some(_inner),
            Self::RepositoryDoesNotExistException(_inner) => ::std::option::Option::Some(_inner),
            Self::RepositoryNameRequiredException(_inner) => ::std::option::Option::Some(_inner),
            Self::SourceAndDestinationAreSameException(_inner) => {
                ::std::option::Option::Some(_inner)
            }
            Self::TargetRequiredException(_inner) => ::std::option::Option::Some(_inner),
            Self::TargetsRequiredException(_inner) => ::std::option::Option::Some(_inner),
            Self::TitleRequiredException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::create_pull_request::_create_pull_request_output::CreatePullRequestOutput;

pub use crate::operation::create_pull_request::_create_pull_request_input::CreatePullRequestInput;

mod _create_pull_request_input;

mod _create_pull_request_output;

/// Builders
pub mod builders;
