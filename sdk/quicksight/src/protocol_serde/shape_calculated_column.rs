// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_calculated_column(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CalculatedColumn,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.column_name {
        object.key("ColumnName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.column_id {
        object.key("ColumnId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.expression {
        object.key("Expression").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_calculated_column<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::CalculatedColumn>,
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
            let mut builder = crate::types::builders::CalculatedColumnBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ColumnName" => {
                                builder = builder.set_column_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ColumnId" => {
                                builder = builder.set_column_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Expression" => {
                                builder = builder.set_expression(
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
