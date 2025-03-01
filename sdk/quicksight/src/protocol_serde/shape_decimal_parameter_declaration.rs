// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_decimal_parameter_declaration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DecimalParameterDeclaration,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.parameter_value_type {
        object.key("ParameterValueType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.default_values {
        #[allow(unused_mut)]
        let mut object_4 = object.key("DefaultValues").start_object();
        crate::protocol_serde::shape_decimal_default_values::ser_decimal_default_values(
            &mut object_4,
            var_3,
        )?;
        object_4.finish();
    }
    if let Some(var_5) = &input.value_when_unset {
        #[allow(unused_mut)]
        let mut object_6 = object.key("ValueWhenUnset").start_object();
        crate::protocol_serde::shape_decimal_value_when_unset_configuration::ser_decimal_value_when_unset_configuration(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.mapped_data_set_parameters {
        let mut array_8 = object.key("MappedDataSetParameters").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_mapped_data_set_parameter::ser_mapped_data_set_parameter(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    Ok(())
}

pub(crate) fn de_decimal_parameter_declaration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::DecimalParameterDeclaration>,
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
            let mut builder = crate::types::builders::DecimalParameterDeclarationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ParameterValueType" => {
                                builder = builder.set_parameter_value_type(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::ParameterValueType::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "Name" => {
                                builder = builder.set_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "DefaultValues" => {
                                builder = builder.set_default_values(
                                    crate::protocol_serde::shape_decimal_default_values::de_decimal_default_values(tokens)?
                                );
                            }
                            "ValueWhenUnset" => {
                                builder = builder.set_value_when_unset(
                                    crate::protocol_serde::shape_decimal_value_when_unset_configuration::de_decimal_value_when_unset_configuration(tokens)?
                                );
                            }
                            "MappedDataSetParameters" => {
                                builder = builder.set_mapped_data_set_parameters(
                                    crate::protocol_serde::shape_mapped_data_set_parameters::de_mapped_data_set_parameters(tokens)?
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
