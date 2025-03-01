// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_topic_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_topic::DeleteTopicOutput,
    crate::operation::delete_topic::DeleteTopicError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::delete_topic::DeleteTopicError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::delete_topic::DeleteTopicError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AuthorizationError" => {
            crate::operation::delete_topic::DeleteTopicError::AuthorizationErrorException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AuthorizationErrorExceptionBuilder::default(
                        );
                    output = crate::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_xml_err(_response_body, output).map_err(crate::operation::delete_topic::DeleteTopicError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ConcurrentAccess" => {
            crate::operation::delete_topic::DeleteTopicError::ConcurrentAccessException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ConcurrentAccessExceptionBuilder::default();
                    output = crate::protocol_serde::shape_concurrent_access_exception::de_concurrent_access_exception_xml_err(_response_body, output).map_err(crate::operation::delete_topic::DeleteTopicError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InternalError" => {
            crate::operation::delete_topic::DeleteTopicError::InternalErrorException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalErrorExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_error_exception::de_internal_error_exception_xml_err(_response_body, output).map_err(crate::operation::delete_topic::DeleteTopicError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameter" => {
            crate::operation::delete_topic::DeleteTopicError::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_xml_err(_response_body, output).map_err(crate::operation::delete_topic::DeleteTopicError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NotFound" => crate::operation::delete_topic::DeleteTopicError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_xml_err(_response_body, output).map_err(crate::operation::delete_topic::DeleteTopicError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "StaleTag" => crate::operation::delete_topic::DeleteTopicError::StaleTagException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::StaleTagExceptionBuilder::default();
                output = crate::protocol_serde::shape_stale_tag_exception::de_stale_tag_exception_xml_err(_response_body, output).map_err(crate::operation::delete_topic::DeleteTopicError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TagPolicy" => crate::operation::delete_topic::DeleteTopicError::TagPolicyException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output =
                    crate::types::error::builders::TagPolicyExceptionBuilder::default();
                output = crate::protocol_serde::shape_tag_policy_exception::de_tag_policy_exception_xml_err(_response_body, output).map_err(crate::operation::delete_topic::DeleteTopicError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_topic::DeleteTopicError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_topic_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_topic::DeleteTopicOutput,
    crate::operation::delete_topic::DeleteTopicError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::delete_topic::builders::DeleteTopicOutputBuilder::default();
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
