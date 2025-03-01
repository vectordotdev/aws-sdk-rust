// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_attribute(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Attribute,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.fully_qualified_name {
        object.key("fullyQualifiedName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.data_type {
        object.key("dataType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.unit {
        object.key("unit").string(var_4.as_str());
    }
    if let Some(var_5) = &input.allowed_values {
        let mut array_6 = object.key("allowedValues").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.min {
        object.key("min").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.max {
        object.key("max").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_9).into()),
        );
    }
    if let Some(var_10) = &input.assigned_value {
        object.key("assignedValue").string(var_10.as_str());
    }
    if let Some(var_11) = &input.default_value {
        object.key("defaultValue").string(var_11.as_str());
    }
    Ok(())
}

pub(crate) fn de_attribute<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::Attribute>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
            let mut builder = crate::types::builders::AttributeBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "fullyQualifiedName" => {
                                builder = builder.set_fully_qualified_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "dataType" => {
                                builder = builder.set_data_type(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::NodeDataType::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "description" => {
                                builder = builder.set_description(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "unit" => {
                                builder = builder.set_unit(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "allowedValues" => {
                                builder = builder.set_allowed_values(
                                    crate::protocol_serde::shape_list_of_strings::de_list_of_strings(tokens)?
                                );
                            }
                            "min" => {
                                builder = builder.set_min(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_f64_lossy()),
                                );
                            }
                            "max" => {
                                builder = builder.set_max(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_f64_lossy()),
                                );
                            }
                            "assignedValue" => {
                                builder = builder.set_assigned_value(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "defaultValue" => {
                                builder = builder.set_default_value(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
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
