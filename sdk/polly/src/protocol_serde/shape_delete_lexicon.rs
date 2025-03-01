// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_lexicon_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_lexicon::DeleteLexiconOutput,
    crate::operation::delete_lexicon::DeleteLexiconError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::delete_lexicon::DeleteLexiconError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::delete_lexicon::DeleteLexiconError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "LexiconNotFoundException" => {
            crate::operation::delete_lexicon::DeleteLexiconError::LexiconNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::LexiconNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_lexicon_not_found_exception::de_lexicon_not_found_exception_json_err(_response_body, output).map_err(crate::operation::delete_lexicon::DeleteLexiconError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceFailureException" => {
            crate::operation::delete_lexicon::DeleteLexiconError::ServiceFailureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_json_err(_response_body, output).map_err(crate::operation::delete_lexicon::DeleteLexiconError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::delete_lexicon::DeleteLexiconError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_lexicon_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_lexicon::DeleteLexiconOutput,
    crate::operation::delete_lexicon::DeleteLexiconError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::delete_lexicon::builders::DeleteLexiconOutputBuilder::default();
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
