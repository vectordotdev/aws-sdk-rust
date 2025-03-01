// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_pipeline_declaration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::PipelineDeclaration>,
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
            let mut builder = crate::types::builders::PipelineDeclarationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "name" => {
                                builder = builder.set_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "roleArn" => {
                                builder = builder.set_role_arn(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "artifactStore" => {
                                builder = builder.set_artifact_store(
                                    crate::protocol_serde::shape_artifact_store::de_artifact_store(
                                        tokens,
                                    )?,
                                );
                            }
                            "artifactStores" => {
                                builder = builder.set_artifact_stores(
                                    crate::protocol_serde::shape_artifact_store_map::de_artifact_store_map(tokens)?
                                );
                            }
                            "stages" => {
                                builder = builder.set_stages(
                                    crate::protocol_serde::shape_pipeline_stage_declaration_list::de_pipeline_stage_declaration_list(tokens)?
                                );
                            }
                            "version" => {
                                builder = builder.set_version(
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

pub fn ser_pipeline_declaration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PipelineDeclaration,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.role_arn {
        object.key("roleArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.artifact_store {
        #[allow(unused_mut)]
        let mut object_4 = object.key("artifactStore").start_object();
        crate::protocol_serde::shape_artifact_store::ser_artifact_store(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.artifact_stores {
        #[allow(unused_mut)]
        let mut object_6 = object.key("artifactStores").start_object();
        for (key_7, value_8) in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_9 = object_6.key(key_7.as_str()).start_object();
                crate::protocol_serde::shape_artifact_store::ser_artifact_store(
                    &mut object_9,
                    value_8,
                )?;
                object_9.finish();
            }
        }
        object_6.finish();
    }
    if let Some(var_10) = &input.stages {
        let mut array_11 = object.key("stages").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_stage_declaration::ser_stage_declaration(
                    &mut object_13,
                    item_12,
                )?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.version {
        object.key("version").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    Ok(())
}
