// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_fpga_device_memory_info(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::FpgaDeviceMemoryInfo, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::FpgaDeviceMemoryInfo::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("sizeInMiB") /* SizeInMiB com.amazonaws.ec2#FpgaDeviceMemoryInfo$SizeInMiB */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#FpgaDeviceMemorySize`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_size_in_mi_b(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
