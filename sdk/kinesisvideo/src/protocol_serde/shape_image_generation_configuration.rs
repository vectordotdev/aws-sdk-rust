// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_image_generation_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::ImageGenerationConfiguration>,
    ::aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<
        Item = Result<
            ::aws_smithy_json::deserialize::Token<'a>,
            ::aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder =
                crate::types::builders::ImageGenerationConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Status" => {
                                builder = builder.set_status(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::ConfigurationStatus::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "ImageSelectorType" => {
                                builder = builder.set_image_selector_type(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::ImageSelectorType::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "DestinationConfig" => {
                                builder = builder.set_destination_config(
                                    crate::protocol_serde::shape_image_generation_destination_config::de_image_generation_destination_config(tokens)?
                                );
                            }
                            "SamplingInterval" => {
                                builder = builder.set_sampling_interval(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "Format" => {
                                builder = builder.set_format(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::Format::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "FormatConfig" => {
                                builder = builder.set_format_config(
                                    crate::protocol_serde::shape_format_config::de_format_config(
                                        tokens,
                                    )?,
                                );
                            }
                            "WidthPixels" => {
                                builder = builder.set_width_pixels(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "HeightPixels" => {
                                builder = builder.set_height_pixels(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    other => {
                        return Err(
                            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                format!("expected object key or end object, found: {:?}", other),
                            ),
                        )
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(
            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ),
        ),
    }
}

pub fn ser_image_generation_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ImageGenerationConfiguration,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.status {
        object.key("Status").string(var_1.as_str());
    }
    if let Some(var_2) = &input.image_selector_type {
        object.key("ImageSelectorType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.destination_config {
        #[allow(unused_mut)]
        let mut object_4 = object.key("DestinationConfig").start_object();
        crate::protocol_serde::shape_image_generation_destination_config::ser_image_generation_destination_config(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.sampling_interval {
        object.key("SamplingInterval").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.format {
        object.key("Format").string(var_6.as_str());
    }
    if let Some(var_7) = &input.format_config {
        #[allow(unused_mut)]
        let mut object_8 = object.key("FormatConfig").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    if let Some(var_11) = &input.width_pixels {
        object.key("WidthPixels").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    if let Some(var_12) = &input.height_pixels {
        object.key("HeightPixels").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    Ok(())
}
