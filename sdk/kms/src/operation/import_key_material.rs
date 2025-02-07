// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl ImportKeyMaterialInput {
    /// Consumes the builder and constructs an Operation<[`ImportKeyMaterial`](crate::operation::import_key_material::ImportKeyMaterial)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::import_key_material::ImportKeyMaterial,
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
                _input: &crate::operation::import_key_material::ImportKeyMaterialInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::import_key_material::ImportKeyMaterialInput,
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
                "TrentService.ImportKeyMaterial",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_import_key_material::ser_import_key_material_input(&self)?,
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
            crate::operation::import_key_material::ImportKeyMaterial::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "ImportKeyMaterial",
            "kms",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `ImportKeyMaterial`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct ImportKeyMaterial;
impl ImportKeyMaterial {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for ImportKeyMaterial {
    type Output = ::std::result::Result<
        crate::operation::import_key_material::ImportKeyMaterialOutput,
        crate::operation::import_key_material::ImportKeyMaterialError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_import_key_material::de_import_key_material_http_error(
                status, headers, body,
            )
        } else {
            crate::protocol_serde::shape_import_key_material::de_import_key_material_http_response_with_props(status, headers, body)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type ImportKeyMaterialErrorKind = ImportKeyMaterialError;
/// Error type for the `ImportKeyMaterialError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum ImportKeyMaterialError {
    /// <p>The system timed out while trying to fulfill the request. You can retry the request.</p>
    DependencyTimeoutException(crate::types::error::DependencyTimeoutException),
    /// <p>The request was rejected because the specified import token is expired. Use <code>GetParametersForImport</code> to get a new import token and public key, use the new public key to encrypt the key material, and then try the request again.</p>
    ExpiredImportTokenException(crate::types::error::ExpiredImportTokenException),
    /// <p>The request was rejected because the key material in the request is, expired, invalid, or is not the same key material that was previously imported into this KMS key.</p>
    IncorrectKeyMaterialException(crate::types::error::IncorrectKeyMaterialException),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArnException(crate::types::error::InvalidArnException),
    /// <p>From the <code>Decrypt</code> or <code>ReEncrypt</code> operation, the request was rejected because the specified ciphertext, or additional authenticated data incorporated into the ciphertext, such as the encryption context, is corrupted, missing, or otherwise invalid.</p>
    /// <p>From the <code>ImportKeyMaterial</code> operation, the request was rejected because KMS could not decrypt the encrypted (wrapped) key material. </p>
    InvalidCiphertextException(crate::types::error::InvalidCiphertextException),
    /// <p>The request was rejected because the provided import token is invalid or is associated with a different KMS key.</p>
    InvalidImportTokenException(crate::types::error::InvalidImportTokenException),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KmsInternalException(crate::types::error::KmsInternalException),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p>
    /// <p>This exceptions means one of the following:</p>
    /// <ul>
    /// <li> <p>The key state of the KMS key is not compatible with the operation. </p> <p>To find the key state, use the <code>DescribeKey</code> operation. For more information about which key states are compatible with each KMS operation, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key states of KMS keys</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p> </li>
    /// <li> <p>For cryptographic operations on KMS keys in custom key stores, this exception represents a general failure with many possible causes. To identify the cause, see the error message that accompanies the exception.</p> </li>
    /// </ul>
    KmsInvalidStateException(crate::types::error::KmsInvalidStateException),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFoundException(crate::types::error::NotFoundException),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperationException(crate::types::error::UnsupportedOperationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for ImportKeyMaterialError {
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
impl ::std::fmt::Display for ImportKeyMaterialError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::DependencyTimeoutException(_inner) => _inner.fmt(f),
            Self::ExpiredImportTokenException(_inner) => _inner.fmt(f),
            Self::IncorrectKeyMaterialException(_inner) => _inner.fmt(f),
            Self::InvalidArnException(_inner) => _inner.fmt(f),
            Self::InvalidCiphertextException(_inner) => _inner.fmt(f),
            Self::InvalidImportTokenException(_inner) => _inner.fmt(f),
            Self::KmsInternalException(_inner) => _inner.fmt(f),
            Self::KmsInvalidStateException(_inner) => _inner.fmt(f),
            Self::NotFoundException(_inner) => _inner.fmt(f),
            Self::UnsupportedOperationException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ImportKeyMaterialError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::DependencyTimeoutException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ExpiredImportTokenException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::IncorrectKeyMaterialException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidArnException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidCiphertextException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidImportTokenException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsInternalException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsInvalidStateException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NotFoundException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UnsupportedOperationException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId
    for crate::operation::import_key_material::ImportKeyMaterialError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for ImportKeyMaterialError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ImportKeyMaterialError {
    /// Creates the `ImportKeyMaterialError::Unhandled` variant from any error type.
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

    /// Creates the `ImportKeyMaterialError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::DependencyTimeoutException(e) => e.meta(),
            Self::ExpiredImportTokenException(e) => e.meta(),
            Self::IncorrectKeyMaterialException(e) => e.meta(),
            Self::InvalidArnException(e) => e.meta(),
            Self::InvalidCiphertextException(e) => e.meta(),
            Self::InvalidImportTokenException(e) => e.meta(),
            Self::KmsInternalException(e) => e.meta(),
            Self::KmsInvalidStateException(e) => e.meta(),
            Self::NotFoundException(e) => e.meta(),
            Self::UnsupportedOperationException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `ImportKeyMaterialError::DependencyTimeoutException`.
    pub fn is_dependency_timeout_exception(&self) -> bool {
        matches!(self, Self::DependencyTimeoutException(_))
    }
    /// Returns `true` if the error kind is `ImportKeyMaterialError::ExpiredImportTokenException`.
    pub fn is_expired_import_token_exception(&self) -> bool {
        matches!(self, Self::ExpiredImportTokenException(_))
    }
    /// Returns `true` if the error kind is `ImportKeyMaterialError::IncorrectKeyMaterialException`.
    pub fn is_incorrect_key_material_exception(&self) -> bool {
        matches!(self, Self::IncorrectKeyMaterialException(_))
    }
    /// Returns `true` if the error kind is `ImportKeyMaterialError::InvalidArnException`.
    pub fn is_invalid_arn_exception(&self) -> bool {
        matches!(self, Self::InvalidArnException(_))
    }
    /// Returns `true` if the error kind is `ImportKeyMaterialError::InvalidCiphertextException`.
    pub fn is_invalid_ciphertext_exception(&self) -> bool {
        matches!(self, Self::InvalidCiphertextException(_))
    }
    /// Returns `true` if the error kind is `ImportKeyMaterialError::InvalidImportTokenException`.
    pub fn is_invalid_import_token_exception(&self) -> bool {
        matches!(self, Self::InvalidImportTokenException(_))
    }
    /// Returns `true` if the error kind is `ImportKeyMaterialError::KmsInternalException`.
    pub fn is_kms_internal_exception(&self) -> bool {
        matches!(self, Self::KmsInternalException(_))
    }
    /// Returns `true` if the error kind is `ImportKeyMaterialError::KmsInvalidStateException`.
    pub fn is_kms_invalid_state_exception(&self) -> bool {
        matches!(self, Self::KmsInvalidStateException(_))
    }
    /// Returns `true` if the error kind is `ImportKeyMaterialError::NotFoundException`.
    pub fn is_not_found_exception(&self) -> bool {
        matches!(self, Self::NotFoundException(_))
    }
    /// Returns `true` if the error kind is `ImportKeyMaterialError::UnsupportedOperationException`.
    pub fn is_unsupported_operation_exception(&self) -> bool {
        matches!(self, Self::UnsupportedOperationException(_))
    }
}
impl ::std::error::Error for ImportKeyMaterialError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::DependencyTimeoutException(_inner) => ::std::option::Option::Some(_inner),
            Self::ExpiredImportTokenException(_inner) => ::std::option::Option::Some(_inner),
            Self::IncorrectKeyMaterialException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidArnException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidCiphertextException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidImportTokenException(_inner) => ::std::option::Option::Some(_inner),
            Self::KmsInternalException(_inner) => ::std::option::Option::Some(_inner),
            Self::KmsInvalidStateException(_inner) => ::std::option::Option::Some(_inner),
            Self::NotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::UnsupportedOperationException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::import_key_material::_import_key_material_output::ImportKeyMaterialOutput;

pub use crate::operation::import_key_material::_import_key_material_input::ImportKeyMaterialInput;

mod _import_key_material_input;

mod _import_key_material_output;

/// Builders
pub mod builders;
