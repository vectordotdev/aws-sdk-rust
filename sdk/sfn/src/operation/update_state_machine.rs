// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl UpdateStateMachineInput {
    /// Consumes the builder and constructs an Operation<[`UpdateStateMachine`](crate::operation::update_state_machine::UpdateStateMachine)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> ::std::result::Result<
        ::aws_smithy_http::operation::Operation<
            crate::operation::update_state_machine::UpdateStateMachine,
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
                _input: &crate::operation::update_state_machine::UpdateStateMachineInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError>
            {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::update_state_machine::UpdateStateMachineInput,
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
                "application/x-amz-json-1.0",
            );
            builder = ::aws_smithy_http::header::set_request_header_if_absent(
                builder,
                ::http::header::HeaderName::from_static("x-amz-target"),
                "AWSStepFunctions.UpdateStateMachine",
            );
            builder
        };
        let mut properties = ::aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_update_state_machine::ser_update_state_machine_input(
                &self,
            )?,
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
            crate::operation::update_state_machine::UpdateStateMachine::new(),
        )
        .with_metadata(::aws_smithy_http::operation::Metadata::new(
            "UpdateStateMachine",
            "sfn",
        ));
        let op = op.with_retry_classifier(::aws_http::retry::AwsResponseRetryClassifier::new());
        ::std::result::Result::Ok(op)
    }
}
/// `ParseStrictResponse` impl for `UpdateStateMachine`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct UpdateStateMachine;
impl UpdateStateMachine {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl ::aws_smithy_http::response::ParseStrictResponse for UpdateStateMachine {
    type Output = ::std::result::Result<
        crate::operation::update_state_machine::UpdateStateMachineOutput,
        crate::operation::update_state_machine::UpdateStateMachineError,
    >;
    fn parse(&self, response: &::http::Response<::bytes::Bytes>) -> Self::Output {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().as_ref();
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        if !success && status != 200 {
            crate::protocol_serde::shape_update_state_machine::de_update_state_machine_http_error(
                status, headers, body,
            )
        } else {
            crate::protocol_serde::shape_update_state_machine::de_update_state_machine_http_response_with_props(status, headers, body)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type UpdateStateMachineErrorKind = UpdateStateMachineError;
/// Error type for the `UpdateStateMachineError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum UpdateStateMachineError {
    /// <p>The provided Amazon Resource Name (ARN) is not valid.</p>
    InvalidArn(crate::types::error::InvalidArn),
    /// <p>The provided Amazon States Language definition is not valid.</p>
    InvalidDefinition(crate::types::error::InvalidDefinition),
    /// <p></p>
    InvalidLoggingConfiguration(crate::types::error::InvalidLoggingConfiguration),
    /// <p>Your <code>tracingConfiguration</code> key does not match, or <code>enabled</code> has not been set to <code>true</code> or <code>false</code>.</p>
    InvalidTracingConfiguration(crate::types::error::InvalidTracingConfiguration),
    /// <p>Request is missing a required parameter. This error occurs if both <code>definition</code> and <code>roleArn</code> are not specified.</p>
    MissingRequiredParameter(crate::types::error::MissingRequiredParameter),
    /// <p>The specified state machine is being deleted.</p>
    StateMachineDeleting(crate::types::error::StateMachineDeleting),
    /// <p>The specified state machine does not exist.</p>
    StateMachineDoesNotExist(crate::types::error::StateMachineDoesNotExist),
    /// <p>The input does not satisfy the constraints specified by an Amazon Web Services service.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for UpdateStateMachineError {
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
impl ::std::fmt::Display for UpdateStateMachineError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::InvalidArn(_inner) => _inner.fmt(f),
            Self::InvalidDefinition(_inner) => _inner.fmt(f),
            Self::InvalidLoggingConfiguration(_inner) => _inner.fmt(f),
            Self::InvalidTracingConfiguration(_inner) => _inner.fmt(f),
            Self::MissingRequiredParameter(_inner) => _inner.fmt(f),
            Self::StateMachineDeleting(_inner) => _inner.fmt(f),
            Self::StateMachineDoesNotExist(_inner) => _inner.fmt(f),
            Self::ValidationException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for UpdateStateMachineError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InvalidArn(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidDefinition(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidLoggingConfiguration(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidTracingConfiguration(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::MissingRequiredParameter(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::StateMachineDeleting(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::StateMachineDoesNotExist(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ValidationException(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl ::aws_http::request_id::RequestId
    for crate::operation::update_state_machine::UpdateStateMachineError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for UpdateStateMachineError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl UpdateStateMachineError {
    /// Creates the `UpdateStateMachineError::Unhandled` variant from any error type.
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

    /// Creates the `UpdateStateMachineError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::InvalidArn(e) => e.meta(),
            Self::InvalidDefinition(e) => e.meta(),
            Self::InvalidLoggingConfiguration(e) => e.meta(),
            Self::InvalidTracingConfiguration(e) => e.meta(),
            Self::MissingRequiredParameter(e) => e.meta(),
            Self::StateMachineDeleting(e) => e.meta(),
            Self::StateMachineDoesNotExist(e) => e.meta(),
            Self::ValidationException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `UpdateStateMachineError::InvalidArn`.
    pub fn is_invalid_arn(&self) -> bool {
        matches!(self, Self::InvalidArn(_))
    }
    /// Returns `true` if the error kind is `UpdateStateMachineError::InvalidDefinition`.
    pub fn is_invalid_definition(&self) -> bool {
        matches!(self, Self::InvalidDefinition(_))
    }
    /// Returns `true` if the error kind is `UpdateStateMachineError::InvalidLoggingConfiguration`.
    pub fn is_invalid_logging_configuration(&self) -> bool {
        matches!(self, Self::InvalidLoggingConfiguration(_))
    }
    /// Returns `true` if the error kind is `UpdateStateMachineError::InvalidTracingConfiguration`.
    pub fn is_invalid_tracing_configuration(&self) -> bool {
        matches!(self, Self::InvalidTracingConfiguration(_))
    }
    /// Returns `true` if the error kind is `UpdateStateMachineError::MissingRequiredParameter`.
    pub fn is_missing_required_parameter(&self) -> bool {
        matches!(self, Self::MissingRequiredParameter(_))
    }
    /// Returns `true` if the error kind is `UpdateStateMachineError::StateMachineDeleting`.
    pub fn is_state_machine_deleting(&self) -> bool {
        matches!(self, Self::StateMachineDeleting(_))
    }
    /// Returns `true` if the error kind is `UpdateStateMachineError::StateMachineDoesNotExist`.
    pub fn is_state_machine_does_not_exist(&self) -> bool {
        matches!(self, Self::StateMachineDoesNotExist(_))
    }
    /// Returns `true` if the error kind is `UpdateStateMachineError::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(self, Self::ValidationException(_))
    }
}
impl ::std::error::Error for UpdateStateMachineError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::InvalidArn(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidDefinition(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidLoggingConfiguration(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidTracingConfiguration(_inner) => ::std::option::Option::Some(_inner),
            Self::MissingRequiredParameter(_inner) => ::std::option::Option::Some(_inner),
            Self::StateMachineDeleting(_inner) => ::std::option::Option::Some(_inner),
            Self::StateMachineDoesNotExist(_inner) => ::std::option::Option::Some(_inner),
            Self::ValidationException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::update_state_machine::_update_state_machine_output::UpdateStateMachineOutput;

pub use crate::operation::update_state_machine::_update_state_machine_input::UpdateStateMachineInput;

mod _update_state_machine_input;

mod _update_state_machine_output;

/// Builders
pub mod builders;
