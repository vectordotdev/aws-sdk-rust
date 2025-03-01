// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_deactivate_evaluation_form_input(
    input: &crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_deactivate_evaluation_form_input::ser_deactivate_evaluation_form_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deactivate_evaluation_form_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormOutput,
    crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(
        crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::unhandled,
    )?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServiceExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(_response_body, output).map_err(crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output).map_err(crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceConflictException" => crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::ResourceConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceConflictExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_conflict_exception::de_resource_conflict_exception_json_err(_response_body, output).map_err(crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output).map_err(crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output).map_err(crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deactivate_evaluation_form_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormOutput,
    crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::deactivate_evaluation_form::builders::DeactivateEvaluationFormOutputBuilder::default();
        output = crate::protocol_serde::shape_deactivate_evaluation_form::de_deactivate_evaluation_form(_response_body, output).map_err(crate::operation::deactivate_evaluation_form::DeactivateEvaluationFormError::unhandled)?;
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_deactivate_evaluation_form(
    value: &[u8],
    mut builder: crate::operation::deactivate_evaluation_form::builders::DeactivateEvaluationFormOutputBuilder,
) -> Result<
    crate::operation::deactivate_evaluation_form::builders::DeactivateEvaluationFormOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned =
        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "EvaluationFormArn" => {
                        builder = builder.set_evaluation_form_arn(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "EvaluationFormId" => {
                        builder = builder.set_evaluation_form_id(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "EvaluationFormVersion" => {
                        builder = builder.set_evaluation_form_version(
                            ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                tokens.next(),
                            )?
                            .map(i32::try_from)
                            .transpose()?,
                        );
                    }
                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(
                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )),
                )
            }
        }
    }
    if tokens.next().is_some() {
        return Err(
            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "found more JSON tokens after completing parsing",
            ),
        );
    }
    Ok(builder)
}
