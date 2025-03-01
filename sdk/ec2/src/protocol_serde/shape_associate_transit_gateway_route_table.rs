// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_transit_gateway_route_table_http_error(_response_status: u16, _response_headers: &::http::header::HeaderMap, _response_body: &[u8]) -> std::result::Result<crate::operation::associate_transit_gateway_route_table::AssociateTransitGatewayRouteTableOutput, crate::operation::associate_transit_gateway_route_table::AssociateTransitGatewayRouteTableError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::associate_transit_gateway_route_table::AssociateTransitGatewayRouteTableError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::associate_transit_gateway_route_table::AssociateTransitGatewayRouteTableError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_transit_gateway_route_table_http_response_with_props(_response_status: u16, _response_headers: &::http::header::HeaderMap, _response_body: &[u8]) -> std::result::Result<crate::operation::associate_transit_gateway_route_table::AssociateTransitGatewayRouteTableOutput, crate::operation::associate_transit_gateway_route_table::AssociateTransitGatewayRouteTableError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::associate_transit_gateway_route_table::builders::AssociateTransitGatewayRouteTableOutputBuilder::default();
        output = crate::protocol_serde::shape_associate_transit_gateway_route_table::de_associate_transit_gateway_route_table(_response_body, output).map_err(crate::operation::associate_transit_gateway_route_table::AssociateTransitGatewayRouteTableError::unhandled)?;
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_associate_transit_gateway_route_table(inp: &[u8], mut builder: crate::operation::associate_transit_gateway_route_table::builders::AssociateTransitGatewayRouteTableOutputBuilder) -> Result<crate::operation::associate_transit_gateway_route_table::builders::AssociateTransitGatewayRouteTableOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("AssociateTransitGatewayRouteTableResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected AssociateTransitGatewayRouteTableResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("association") /* Association com.amazonaws.ec2.synthetic#AssociateTransitGatewayRouteTableOutput$Association */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_transit_gateway_association::de_transit_gateway_association(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_association(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
