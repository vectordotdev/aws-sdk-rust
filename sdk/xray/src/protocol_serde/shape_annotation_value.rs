// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_annotation_value<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::AnnotationValue>,
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
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => loop {
            match tokens.next().transpose()? {
                Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                    if variant.is_some() {
                        return Err(
                            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                "encountered mixed variants in union",
                            ),
                        );
                    }
                    variant = match key.to_unescaped()?.as_ref() {
                        "NumberValue" => Some(crate::types::AnnotationValue::NumberValue(
                            ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                tokens.next(),
                            )?
                            .map(|v| v.to_f64_lossy())
                            .unwrap_or_default(),
                        )),
                        "BooleanValue" => Some(crate::types::AnnotationValue::BooleanValue(
                            ::aws_smithy_json::deserialize::token::expect_bool_or_null(
                                tokens.next(),
                            )?
                            .unwrap_or_default(),
                        )),
                        "StringValue" => Some(crate::types::AnnotationValue::StringValue(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?
                            .unwrap_or_default(),
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::AnnotationValue::Unknown)
                        }
                    };
                }
                other => {
                    return Err(
                        ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )),
                    )
                }
            }
        },
        _ => {
            return Err(
                ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                    "expected start object or null",
                ),
            )
        }
    }
    Ok(variant)
}
