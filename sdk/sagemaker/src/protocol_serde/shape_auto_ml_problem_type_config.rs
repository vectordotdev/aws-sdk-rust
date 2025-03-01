// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_auto_ml_problem_type_config(
    object_9: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AutoMlProblemTypeConfig,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::types::AutoMlProblemTypeConfig::ImageClassificationJobConfig(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_9.key("ImageClassificationJobConfig").start_object();
            crate::protocol_serde::shape_image_classification_job_config::ser_image_classification_job_config(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::AutoMlProblemTypeConfig::TextClassificationJobConfig(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_9.key("TextClassificationJobConfig").start_object();
            crate::protocol_serde::shape_text_classification_job_config::ser_text_classification_job_config(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::AutoMlProblemTypeConfig::Unknown => {
            return Err(
                ::aws_smithy_http::operation::error::SerializationError::unknown_variant(
                    "AutoMlProblemTypeConfig",
                ),
            )
        }
    }
    Ok(())
}

pub(crate) fn de_auto_ml_problem_type_config<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::AutoMlProblemTypeConfig>,
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
                            "ImageClassificationJobConfig" => {
                                Some(crate::types::AutoMlProblemTypeConfig::ImageClassificationJobConfig(
                                    crate::protocol_serde::shape_image_classification_job_config::de_image_classification_job_config(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'ImageClassificationJobConfig' cannot be null"))?
                                ))
                            }
                            "TextClassificationJobConfig" => {
                                Some(crate::types::AutoMlProblemTypeConfig::TextClassificationJobConfig(
                                    crate::protocol_serde::shape_text_classification_job_config::de_text_classification_job_config(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'TextClassificationJobConfig' cannot be null"))?
                                ))
                            }
                            _ => {
                                                                      ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                      Some(crate::types::AutoMlProblemTypeConfig::Unknown)
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
