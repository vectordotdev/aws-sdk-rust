// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_impersonation_rule(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ImpersonationRule,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.impersonation_rule_id {
        object.key("ImpersonationRuleId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.effect {
        object.key("Effect").string(var_4.as_str());
    }
    if let Some(var_5) = &input.target_users {
        let mut array_6 = object.key("TargetUsers").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.not_target_users {
        let mut array_9 = object.key("NotTargetUsers").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    Ok(())
}

pub(crate) fn de_impersonation_rule<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::ImpersonationRule>,
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
            let mut builder = crate::types::builders::ImpersonationRuleBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key
                        .to_unescaped()?
                        .as_ref()
                    {
                        "ImpersonationRuleId" => {
                            builder = builder.set_impersonation_rule_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
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
                        "Description" => {
                            builder = builder.set_description(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "Effect" => {
                            builder = builder.set_effect(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| {
                                    s.to_unescaped()
                                        .map(|u| crate::types::AccessEffect::from(u.as_ref()))
                                })
                                .transpose()?,
                            );
                        }
                        "TargetUsers" => {
                            builder = builder.set_target_users(
                                crate::protocol_serde::shape_target_users::de_target_users(tokens)?,
                            );
                        }
                        "NotTargetUsers" => {
                            builder = builder.set_not_target_users(
                                crate::protocol_serde::shape_target_users::de_target_users(tokens)?,
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
