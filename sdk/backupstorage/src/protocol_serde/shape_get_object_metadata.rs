// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_object_metadata_op_response(
    op_response: &mut ::aws_smithy_http::operation::Response,
) -> ::std::result::Result<
    crate::operation::get_object_metadata::GetObjectMetadataOutput,
    crate::operation::get_object_metadata::GetObjectMetadataError,
> {
    #[allow(unused_variables)]
    let (response, properties) = op_response.parts_mut();
    crate::protocol_serde::shape_get_object_metadata::de_get_object_metadata_http_response_with_props(response, &properties)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_object_metadata_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_object_metadata::GetObjectMetadataOutput,
    crate::operation::get_object_metadata::GetObjectMetadataError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::get_object_metadata::GetObjectMetadataError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::get_object_metadata::GetObjectMetadataError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::get_object_metadata::GetObjectMetadataError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output).map_err(crate::operation::get_object_metadata::GetObjectMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "IllegalArgumentException" => crate::operation::get_object_metadata::GetObjectMetadataError::IllegalArgumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::IllegalArgumentExceptionBuilder::default();
                    output = crate::protocol_serde::shape_illegal_argument_exception::de_illegal_argument_exception_json_err(_response_body, output).map_err(crate::operation::get_object_metadata::GetObjectMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSInvalidKeyUsageException" => crate::operation::get_object_metadata::GetObjectMetadataError::KmsInvalidKeyUsageException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::KmsInvalidKeyUsageExceptionBuilder::default();
                    output = crate::protocol_serde::shape_kms_invalid_key_usage_exception::de_kms_invalid_key_usage_exception_json_err(_response_body, output).map_err(crate::operation::get_object_metadata::GetObjectMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::get_object_metadata::GetObjectMetadataError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output).map_err(crate::operation::get_object_metadata::GetObjectMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RetryableException" => crate::operation::get_object_metadata::GetObjectMetadataError::RetryableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::RetryableExceptionBuilder::default();
                    output = crate::protocol_serde::shape_retryable_exception::de_retryable_exception_json_err(_response_body, output).map_err(crate::operation::get_object_metadata::GetObjectMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceInternalException" => crate::operation::get_object_metadata::GetObjectMetadataError::ServiceInternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceInternalExceptionBuilder::default();
                    output = crate::protocol_serde::shape_service_internal_exception::de_service_internal_exception_json_err(_response_body, output).map_err(crate::operation::get_object_metadata::GetObjectMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::operation::get_object_metadata::GetObjectMetadataError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(_response_body, output).map_err(crate::operation::get_object_metadata::GetObjectMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::operation::get_object_metadata::GetObjectMetadataError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output).map_err(crate::operation::get_object_metadata::GetObjectMetadataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::get_object_metadata::GetObjectMetadataError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
#[allow(unused_variables)]
pub fn de_get_object_metadata_http_response_with_props(
    response: &mut ::http::Response<::aws_smithy_http::body::SdkBody>,
    properties: &::aws_smithy_http::property_bag::PropertyBag,
) -> std::result::Result<
    crate::operation::get_object_metadata::GetObjectMetadataOutput,
    crate::operation::get_object_metadata::GetObjectMetadataError,
> {
    let mut _response_body = ::aws_smithy_http::body::SdkBody::taken();
    std::mem::swap(&mut _response_body, response.body_mut());
    let _response_body = &mut _response_body;

    let _response_status = response.status().as_u16();
    let _response_headers = response.headers();
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_object_metadata::builders::GetObjectMetadataOutputBuilder::default();
        output = output.set_metadata_blob(Some(
            crate::protocol_serde::shape_get_object_metadata_output::de_metadata_blob_payload(
                _response_body,
            )?,
        ));
        output = output.set_metadata_blob_checksum(
            crate::protocol_serde::shape_get_object_metadata_output::de_metadata_blob_checksum_header(_response_headers)
                                    .map_err(|_|crate::operation::get_object_metadata::GetObjectMetadataError::unhandled("Failed to parse MetadataBlobChecksum from header `x-amz-checksum"))?
        );
        output = output.set_metadata_blob_checksum_algorithm(
            crate::protocol_serde::shape_get_object_metadata_output::de_metadata_blob_checksum_algorithm_header(_response_headers)
                                    .map_err(|_|crate::operation::get_object_metadata::GetObjectMetadataError::unhandled("Failed to parse MetadataBlobChecksumAlgorithm from header `x-amz-checksum-algorithm"))?
        );
        output = output.set_metadata_blob_length(
            crate::protocol_serde::shape_get_object_metadata_output::de_metadata_blob_length_header(_response_headers)
                                    .map_err(|_|crate::operation::get_object_metadata::GetObjectMetadataError::unhandled("Failed to parse MetadataBlobLength from header `x-amz-data-length"))?
        );
        output = output.set_metadata_string(
            crate::protocol_serde::shape_get_object_metadata_output::de_metadata_string_header(
                _response_headers,
            )
            .map_err(|_| {
                crate::operation::get_object_metadata::GetObjectMetadataError::unhandled(
                    "Failed to parse MetadataString from header `x-amz-metadata-string",
                )
            })?,
        );
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
