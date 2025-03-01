// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_availability_zone(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::AvailabilityZone, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AvailabilityZone::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Name") /* Name com.amazonaws.redshift#AvailabilityZone$Name */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_1);
            }
            ,
            s if s.matches("SupportedPlatforms") /* SupportedPlatforms com.amazonaws.redshift#AvailabilityZone$SupportedPlatforms */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_supported_platforms_list::de_supported_platforms_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_supported_platforms(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
