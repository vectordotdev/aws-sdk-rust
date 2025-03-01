// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_replication_configuration(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ReplicationConfiguration, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ReplicationConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Role") /* Role com.amazonaws.s3control#ReplicationConfiguration$Role */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_role(var_1);
            }
            ,
            s if s.matches("Rules") /* Rules com.amazonaws.s3control#ReplicationConfiguration$Rules */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_replication_rules::de_replication_rules(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_rules(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

pub fn ser_replication_configuration(
    input: &crate::types::ReplicationConfiguration,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_3) = &input.role {
        let mut inner_writer = scope.start_el("Role").finish();
        inner_writer.data(var_3.as_str());
    }
    if let Some(var_4) = &input.rules {
        let mut inner_writer = scope.start_el("Rules").finish();
        for list_item_5 in var_4 {
            {
                let inner_writer = inner_writer.start_el("Rule");
                crate::protocol_serde::shape_replication_rule::ser_replication_rule(
                    list_item_5,
                    inner_writer,
                )?
            }
        }
    }
    scope.finish();
    Ok(())
}
