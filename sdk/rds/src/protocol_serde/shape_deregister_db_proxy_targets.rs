// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_db_proxy_targets_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::deregister_db_proxy_targets::DeregisterDbProxyTargetsOutput,
    crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(
        crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError::unhandled,
    )?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DBProxyNotFoundFault" => crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError::DbProxyNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DbProxyNotFoundFaultBuilder::default();
                    output = crate::protocol_serde::shape_db_proxy_not_found_fault::de_db_proxy_not_found_fault_xml_err(_response_body, output).map_err(crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBProxyTargetGroupNotFoundFault" => crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError::DbProxyTargetGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DbProxyTargetGroupNotFoundFaultBuilder::default();
                    output = crate::protocol_serde::shape_db_proxy_target_group_not_found_fault::de_db_proxy_target_group_not_found_fault_xml_err(_response_body, output).map_err(crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBProxyTargetNotFoundFault" => crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError::DbProxyTargetNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DbProxyTargetNotFoundFaultBuilder::default();
                    output = crate::protocol_serde::shape_db_proxy_target_not_found_fault::de_db_proxy_target_not_found_fault_xml_err(_response_body, output).map_err(crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDBProxyStateFault" => crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError::InvalidDbProxyStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidDbProxyStateFaultBuilder::default();
                    output = crate::protocol_serde::shape_invalid_db_proxy_state_fault::de_invalid_db_proxy_state_fault_xml_err(_response_body, output).map_err(crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_db_proxy_targets_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::deregister_db_proxy_targets::DeregisterDbProxyTargetsOutput,
    crate::operation::deregister_db_proxy_targets::DeregisterDBProxyTargetsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::deregister_db_proxy_targets::builders::DeregisterDbProxyTargetsOutputBuilder::default();
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
