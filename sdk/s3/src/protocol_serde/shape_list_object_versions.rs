// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_object_versions_headers(
    input: &crate::operation::list_object_versions::ListObjectVersionsInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_http::operation::error::BuildError>
{
    if let ::std::option::Option::Some(inner_1) = &input.expected_bucket_owner {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_http::operation::error::BuildError::invalid_field(
                    "expected_bucket_owner",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("x-amz-expected-bucket-owner", header_value);
        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_object_versions_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_object_versions::ListObjectVersionsOutput,
    crate::operation::list_object_versions::ListObjectVersionsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(
        _response_status,
        _response_headers,
        _response_body,
    )
    .map_err(crate::operation::list_object_versions::ListObjectVersionsError::unhandled)?;
    generic_builder =
        crate::s3_request_id::apply_extended_request_id(generic_builder, _response_headers);
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::list_object_versions::ListObjectVersionsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_object_versions_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_object_versions::ListObjectVersionsOutput,
    crate::operation::list_object_versions::ListObjectVersionsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_object_versions::builders::ListObjectVersionsOutputBuilder::default();
        output = crate::protocol_serde::shape_list_object_versions::de_list_object_versions(
            _response_body,
            output,
        )
        .map_err(crate::operation::list_object_versions::ListObjectVersionsError::unhandled)?;
        output._set_extended_request_id(
            crate::s3_request_id::RequestIdExt::extended_request_id(_response_headers)
                .map(str::to_string),
        );
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_list_object_versions(
    inp: &[u8],
    mut builder: crate::operation::list_object_versions::builders::ListObjectVersionsOutputBuilder,
) -> Result<
    crate::operation::list_object_versions::builders::ListObjectVersionsOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("ListVersionsResult") {
        return Err(
                                ::aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected ListVersionsResult but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            );
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("CommonPrefixes") /* CommonPrefixes com.amazonaws.s3.synthetic#ListObjectVersionsOutput$CommonPrefixes */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::vec::Vec<crate::types::CommonPrefix>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_4 = builder.common_prefixes.take().unwrap_or_default();
                            list_4.push(
                                crate::protocol_serde::shape_common_prefix::de_common_prefix(&mut tag)
                                ?
                            );
                            list_4
                        })
                        ?
                    )
                ;
                builder = builder.set_common_prefixes(var_3);
            }
            ,
            s if s.matches("NextKeyMarker") /* NextKeyMarker com.amazonaws.s3.synthetic#ListObjectVersionsOutput$NextKeyMarker */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_key_marker(var_5);
            }
            ,
            s if s.matches("Delimiter") /* Delimiter com.amazonaws.s3.synthetic#ListObjectVersionsOutput$Delimiter */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_delimiter(var_6);
            }
            ,
            s if s.matches("EncodingType") /* EncodingType com.amazonaws.s3.synthetic#ListObjectVersionsOutput$EncodingType */ =>  {
                let var_7 =
                    Some(
                        Result::<crate::types::EncodingType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::EncodingType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_encoding_type(var_7);
            }
            ,
            s if s.matches("IsTruncated") /* IsTruncated com.amazonaws.s3.synthetic#ListObjectVersionsOutput$IsTruncated */ =>  {
                let var_8 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3#IsTruncated`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_truncated(var_8);
            }
            ,
            s if s.matches("NextVersionIdMarker") /* NextVersionIdMarker com.amazonaws.s3.synthetic#ListObjectVersionsOutput$NextVersionIdMarker */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_version_id_marker(var_9);
            }
            ,
            s if s.matches("Prefix") /* Prefix com.amazonaws.s3.synthetic#ListObjectVersionsOutput$Prefix */ =>  {
                let var_10 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_10);
            }
            ,
            s if s.matches("Name") /* Name com.amazonaws.s3.synthetic#ListObjectVersionsOutput$Name */ =>  {
                let var_11 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_11);
            }
            ,
            s if s.matches("VersionIdMarker") /* VersionIdMarker com.amazonaws.s3.synthetic#ListObjectVersionsOutput$VersionIdMarker */ =>  {
                let var_12 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_version_id_marker(var_12);
            }
            ,
            s if s.matches("Version") /* Versions com.amazonaws.s3.synthetic#ListObjectVersionsOutput$Versions */ =>  {
                let var_13 =
                    Some(
                        Result::<::std::vec::Vec<crate::types::ObjectVersion>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_14 = builder.versions.take().unwrap_or_default();
                            list_14.push(
                                crate::protocol_serde::shape_object_version::de_object_version(&mut tag)
                                ?
                            );
                            list_14
                        })
                        ?
                    )
                ;
                builder = builder.set_versions(var_13);
            }
            ,
            s if s.matches("MaxKeys") /* MaxKeys com.amazonaws.s3.synthetic#ListObjectVersionsOutput$MaxKeys */ =>  {
                let var_15 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.s3#MaxKeys`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_keys(var_15);
            }
            ,
            s if s.matches("DeleteMarker") /* DeleteMarkers com.amazonaws.s3.synthetic#ListObjectVersionsOutput$DeleteMarkers */ =>  {
                let var_16 =
                    Some(
                        Result::<::std::vec::Vec<crate::types::DeleteMarkerEntry>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_17 = builder.delete_markers.take().unwrap_or_default();
                            list_17.push(
                                crate::protocol_serde::shape_delete_marker_entry::de_delete_marker_entry(&mut tag)
                                ?
                            );
                            list_17
                        })
                        ?
                    )
                ;
                builder = builder.set_delete_markers(var_16);
            }
            ,
            s if s.matches("KeyMarker") /* KeyMarker com.amazonaws.s3.synthetic#ListObjectVersionsOutput$KeyMarker */ =>  {
                let var_18 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_key_marker(var_18);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
