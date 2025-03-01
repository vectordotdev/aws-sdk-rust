// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_streaming_distribution_headers(
    input: &crate::operation::delete_streaming_distribution::DeleteStreamingDistributionInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_http::operation::error::BuildError>
{
    if let ::std::option::Option::Some(inner_1) = &input.if_match {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_http::operation::error::BuildError::invalid_field(
                    "if_match",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("If-Match", header_value);
        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_streaming_distribution_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_streaming_distribution::DeleteStreamingDistributionOutput,
    crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDenied" => crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError::AccessDenied({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedBuilder::default();
                    output = crate::protocol_serde::shape_access_denied::de_access_denied_xml_err(_response_body, output).map_err(crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidIfMatchVersion" => crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError::InvalidIfMatchVersion({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidIfMatchVersionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_if_match_version::de_invalid_if_match_version_xml_err(_response_body, output).map_err(crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchStreamingDistribution" => crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError::NoSuchStreamingDistribution({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchStreamingDistributionBuilder::default();
                    output = crate::protocol_serde::shape_no_such_streaming_distribution::de_no_such_streaming_distribution_xml_err(_response_body, output).map_err(crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PreconditionFailed" => crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError::PreconditionFailed({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::PreconditionFailedBuilder::default();
                    output = crate::protocol_serde::shape_precondition_failed::de_precondition_failed_xml_err(_response_body, output).map_err(crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "StreamingDistributionNotDisabled" => crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError::StreamingDistributionNotDisabled({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::StreamingDistributionNotDisabledBuilder::default();
                    output = crate::protocol_serde::shape_streaming_distribution_not_disabled::de_streaming_distribution_not_disabled_xml_err(_response_body, output).map_err(crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_streaming_distribution_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_streaming_distribution::DeleteStreamingDistributionOutput,
    crate::operation::delete_streaming_distribution::DeleteStreamingDistributionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_streaming_distribution::builders::DeleteStreamingDistributionOutputBuilder::default();
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
