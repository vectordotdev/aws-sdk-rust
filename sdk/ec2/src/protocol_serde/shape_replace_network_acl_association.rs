// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_replace_network_acl_association_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociationOutput,
    crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociationError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociationError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_replace_network_acl_association_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociationOutput,
    crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationOutputBuilder::default();
        output = crate::protocol_serde::shape_replace_network_acl_association::de_replace_network_acl_association(_response_body, output).map_err(crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociationError::unhandled)?;
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_replace_network_acl_association(inp: &[u8], mut builder: crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationOutputBuilder) -> Result<crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ReplaceNetworkAclAssociationResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ReplaceNetworkAclAssociationResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("newAssociationId") /* NewAssociationId com.amazonaws.ec2.synthetic#ReplaceNetworkAclAssociationOutput$NewAssociationId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_new_association_id(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
