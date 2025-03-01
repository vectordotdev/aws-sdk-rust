// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_endpoint_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_endpoint::DeleteEndpointOutput,
    crate::operation::delete_endpoint::DeleteEndpointError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::delete_endpoint::DeleteEndpointError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::delete_endpoint::DeleteEndpointError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AuthorizationError" => {
            crate::operation::delete_endpoint::DeleteEndpointError::AuthorizationErrorException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AuthorizationErrorExceptionBuilder::default(
                        );
                    output = crate::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_xml_err(_response_body, output).map_err(crate::operation::delete_endpoint::DeleteEndpointError::unhandled)?;
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
            crate::operation::delete_endpoint::DeleteEndpointError::InternalErrorException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalErrorExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_error_exception::de_internal_error_exception_xml_err(_response_body, output).map_err(crate::operation::delete_endpoint::DeleteEndpointError::unhandled)?;
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
            crate::operation::delete_endpoint::DeleteEndpointError::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_xml_err(_response_body, output).map_err(crate::operation::delete_endpoint::DeleteEndpointError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::delete_endpoint::DeleteEndpointError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_endpoint_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_endpoint::DeleteEndpointOutput,
    crate::operation::delete_endpoint::DeleteEndpointError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::delete_endpoint::builders::DeleteEndpointOutputBuilder::default();
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
