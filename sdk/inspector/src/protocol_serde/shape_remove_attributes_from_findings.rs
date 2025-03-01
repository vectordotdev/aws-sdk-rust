// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_remove_attributes_from_findings_input(
    input: &crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_remove_attributes_from_findings_input::ser_remove_attributes_from_findings_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_attributes_from_findings_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsOutput,
    crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output).map_err(crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalException" => crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::InternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(_response_body, output).map_err(crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInputException" => crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(_response_body, output).map_err(crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchEntityException" => crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_json_err(_response_body, output).map_err(crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceTemporarilyUnavailableException" => crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::ServiceTemporarilyUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceTemporarilyUnavailableExceptionBuilder::default();
                    output = crate::protocol_serde::shape_service_temporarily_unavailable_exception::de_service_temporarily_unavailable_exception_json_err(_response_body, output).map_err(crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_attributes_from_findings_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsOutput,
    crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::remove_attributes_from_findings::builders::RemoveAttributesFromFindingsOutputBuilder::default();
        output = crate::protocol_serde::shape_remove_attributes_from_findings::de_remove_attributes_from_findings(_response_body, output).map_err(crate::operation::remove_attributes_from_findings::RemoveAttributesFromFindingsError::unhandled)?;
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_remove_attributes_from_findings(value: &[u8], mut builder: crate::operation::remove_attributes_from_findings::builders::RemoveAttributesFromFindingsOutputBuilder) -> Result<crate::operation::remove_attributes_from_findings::builders::RemoveAttributesFromFindingsOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "failedItems" => {
                        builder = builder.set_failed_items(
                            crate::protocol_serde::shape_failed_items::de_failed_items(tokens)?,
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
