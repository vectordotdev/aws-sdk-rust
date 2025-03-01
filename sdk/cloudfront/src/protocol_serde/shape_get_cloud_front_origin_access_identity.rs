// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_cloud_front_origin_access_identity_http_error(_response_status: u16, _response_headers: &::http::header::HeaderMap, _response_body: &[u8]) -> std::result::Result<crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityOutput, crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDenied" => crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityError::AccessDenied({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedBuilder::default();
                    output = crate::protocol_serde::shape_access_denied::de_access_denied_xml_err(_response_body, output).map_err(crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchCloudFrontOriginAccessIdentity" => crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityError::NoSuchCloudFrontOriginAccessIdentity({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchCloudFrontOriginAccessIdentityBuilder::default();
                    output = crate::protocol_serde::shape_no_such_cloud_front_origin_access_identity::de_no_such_cloud_front_origin_access_identity_xml_err(_response_body, output).map_err(crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_cloud_front_origin_access_identity_http_response_with_props(_response_status: u16, _response_headers: &::http::header::HeaderMap, _response_body: &[u8]) -> std::result::Result<crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityOutput, crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_cloud_front_origin_access_identity::builders::GetCloudFrontOriginAccessIdentityOutputBuilder::default();
        output = output.set_cloud_front_origin_access_identity(
            crate::protocol_serde::shape_get_cloud_front_origin_access_identity_output::de_cloud_front_origin_access_identity_payload(_response_body)?
        );
        output = output.set_e_tag(
            crate::protocol_serde::shape_get_cloud_front_origin_access_identity_output::de_e_tag_header(_response_headers)
                                    .map_err(|_|crate::operation::get_cloud_front_origin_access_identity::GetCloudFrontOriginAccessIdentityError::unhandled("Failed to parse ETag from header `ETag"))?
        );
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
