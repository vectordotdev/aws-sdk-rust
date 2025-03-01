// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_ssm_action_definition(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SsmActionDefinition,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action_sub_type {
        object.key("ActionSubType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.region {
        object.key("Region").string(var_2.as_str());
    }
    if let Some(var_3) = &input.instance_ids {
        let mut array_4 = object.key("InstanceIds").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}

pub(crate) fn de_ssm_action_definition<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::SsmActionDefinition>,
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
            let mut builder = crate::types::builders::SsmActionDefinitionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key
                        .to_unescaped()?
                        .as_ref()
                    {
                        "ActionSubType" => {
                            builder = builder.set_action_sub_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| {
                                    s.to_unescaped()
                                        .map(|u| crate::types::ActionSubType::from(u.as_ref()))
                                })
                                .transpose()?,
                            );
                        }
                        "Region" => {
                            builder = builder.set_region(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "InstanceIds" => {
                            builder = builder.set_instance_ids(
                                crate::protocol_serde::shape_instance_ids::de_instance_ids(tokens)?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
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
