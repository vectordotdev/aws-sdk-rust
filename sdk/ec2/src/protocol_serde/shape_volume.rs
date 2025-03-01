// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_volume(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::Volume, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Volume::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("attachmentSet") /* Attachments com.amazonaws.ec2#Volume$Attachments */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_volume_attachment_list::de_volume_attachment_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_attachments(var_1);
            }
            ,
            s if s.matches("availabilityZone") /* AvailabilityZone com.amazonaws.ec2#Volume$AvailabilityZone */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone(var_2);
            }
            ,
            s if s.matches("createTime") /* CreateTime com.amazonaws.ec2#Volume$CreateTime */ =>  {
                let var_3 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_create_time(var_3);
            }
            ,
            s if s.matches("encrypted") /* Encrypted com.amazonaws.ec2#Volume$Encrypted */ =>  {
                let var_4 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_encrypted(var_4);
            }
            ,
            s if s.matches("kmsKeyId") /* KmsKeyId com.amazonaws.ec2#Volume$KmsKeyId */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_kms_key_id(var_5);
            }
            ,
            s if s.matches("outpostArn") /* OutpostArn com.amazonaws.ec2#Volume$OutpostArn */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_outpost_arn(var_6);
            }
            ,
            s if s.matches("size") /* Size com.amazonaws.ec2#Volume$Size */ =>  {
                let var_7 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_size(var_7);
            }
            ,
            s if s.matches("snapshotId") /* SnapshotId com.amazonaws.ec2#Volume$SnapshotId */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_snapshot_id(var_8);
            }
            ,
            s if s.matches("status") /* State com.amazonaws.ec2#Volume$State */ =>  {
                let var_9 =
                    Some(
                        Result::<crate::types::VolumeState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::VolumeState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_9);
            }
            ,
            s if s.matches("volumeId") /* VolumeId com.amazonaws.ec2#Volume$VolumeId */ =>  {
                let var_10 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_volume_id(var_10);
            }
            ,
            s if s.matches("iops") /* Iops com.amazonaws.ec2#Volume$Iops */ =>  {
                let var_11 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_iops(var_11);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#Volume$Tags */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_12);
            }
            ,
            s if s.matches("volumeType") /* VolumeType com.amazonaws.ec2#Volume$VolumeType */ =>  {
                let var_13 =
                    Some(
                        Result::<crate::types::VolumeType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::VolumeType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_volume_type(var_13);
            }
            ,
            s if s.matches("fastRestored") /* FastRestored com.amazonaws.ec2#Volume$FastRestored */ =>  {
                let var_14 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_fast_restored(var_14);
            }
            ,
            s if s.matches("multiAttachEnabled") /* MultiAttachEnabled com.amazonaws.ec2#Volume$MultiAttachEnabled */ =>  {
                let var_15 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_multi_attach_enabled(var_15);
            }
            ,
            s if s.matches("throughput") /* Throughput com.amazonaws.ec2#Volume$Throughput */ =>  {
                let var_16 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_throughput(var_16);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
