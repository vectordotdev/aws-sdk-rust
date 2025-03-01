// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_queue_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_queue::CreateQueueOutput,
    crate::operation::create_queue::CreateQueueError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::create_queue::CreateQueueError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::create_queue::CreateQueueError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AWS.SimpleQueueService.QueueDeletedRecently" => {
            crate::operation::create_queue::CreateQueueError::QueueDeletedRecently({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::QueueDeletedRecentlyBuilder::default();
                    output = crate::protocol_serde::shape_queue_deleted_recently::de_queue_deleted_recently_xml_err(_response_body, output).map_err(crate::operation::create_queue::CreateQueueError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "QueueAlreadyExists" => {
            crate::operation::create_queue::CreateQueueError::QueueNameExists({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::QueueNameExistsBuilder::default();
                    output = crate::protocol_serde::shape_queue_name_exists::de_queue_name_exists_xml_err(_response_body, output).map_err(crate::operation::create_queue::CreateQueueError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::create_queue::CreateQueueError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_queue_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_queue::CreateQueueOutput,
    crate::operation::create_queue::CreateQueueError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::create_queue::builders::CreateQueueOutputBuilder::default();
        output = crate::protocol_serde::shape_create_queue::de_create_queue(_response_body, output)
            .map_err(crate::operation::create_queue::CreateQueueError::unhandled)?;
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_queue(
    inp: &[u8],
    mut builder: crate::operation::create_queue::builders::CreateQueueOutputBuilder,
) -> Result<
    crate::operation::create_queue::builders::CreateQueueOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CreateQueueResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CreateQueueResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("CreateQueueResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected CreateQueueResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("QueueUrl") /* QueueUrl com.amazonaws.sqs.synthetic#CreateQueueOutput$QueueUrl */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_queue_url(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected CreateQueueResult tag",
        ));
    };
    Ok(builder)
}
