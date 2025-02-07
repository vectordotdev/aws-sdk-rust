// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_alias_target(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::AliasTarget, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AliasTarget::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("HostedZoneId") /* HostedZoneId com.amazonaws.route53#AliasTarget$HostedZoneId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_hosted_zone_id(var_1);
            }
            ,
            s if s.matches("DNSName") /* DNSName com.amazonaws.route53#AliasTarget$DNSName */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_dns_name(var_2);
            }
            ,
            s if s.matches("EvaluateTargetHealth") /* EvaluateTargetHealth com.amazonaws.route53#AliasTarget$EvaluateTargetHealth */ =>  {
                let var_3 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.route53#AliasHealthEnabled`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_evaluate_target_health(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

pub fn ser_alias_target(
    input: &crate::types::AliasTarget,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_4) = &input.hosted_zone_id {
        let mut inner_writer = scope.start_el("HostedZoneId").finish();
        inner_writer.data(var_4.as_str());
    }
    if let Some(var_5) = &input.dns_name {
        let mut inner_writer = scope.start_el("DNSName").finish();
        inner_writer.data(var_5.as_str());
    }
    {
        let mut inner_writer = scope.start_el("EvaluateTargetHealth").finish();
        inner_writer.data(
            ::aws_smithy_types::primitive::Encoder::from(input.evaluate_target_health).encode(),
        );
    }
    scope.finish();
    Ok(())
}
