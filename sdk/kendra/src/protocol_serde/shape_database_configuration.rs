// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_database_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DatabaseConfiguration,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.database_engine_type {
        object.key("DatabaseEngineType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.connection_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("ConnectionConfiguration").start_object();
        crate::protocol_serde::shape_connection_configuration::ser_connection_configuration(
            &mut object_3,
            var_2,
        )?;
        object_3.finish();
    }
    if let Some(var_4) = &input.vpc_configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("VpcConfiguration").start_object();
        crate::protocol_serde::shape_data_source_vpc_configuration::ser_data_source_vpc_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.column_configuration {
        #[allow(unused_mut)]
        let mut object_7 = object.key("ColumnConfiguration").start_object();
        crate::protocol_serde::shape_column_configuration::ser_column_configuration(
            &mut object_7,
            var_6,
        )?;
        object_7.finish();
    }
    if let Some(var_8) = &input.acl_configuration {
        #[allow(unused_mut)]
        let mut object_9 = object.key("AclConfiguration").start_object();
        crate::protocol_serde::shape_acl_configuration::ser_acl_configuration(
            &mut object_9,
            var_8,
        )?;
        object_9.finish();
    }
    if let Some(var_10) = &input.sql_configuration {
        #[allow(unused_mut)]
        let mut object_11 = object.key("SqlConfiguration").start_object();
        crate::protocol_serde::shape_sql_configuration::ser_sql_configuration(
            &mut object_11,
            var_10,
        )?;
        object_11.finish();
    }
    Ok(())
}

pub(crate) fn de_database_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::DatabaseConfiguration>,
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
            let mut builder = crate::types::builders::DatabaseConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "DatabaseEngineType" => {
                                builder = builder.set_database_engine_type(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::DatabaseEngineType::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "ConnectionConfiguration" => {
                                builder = builder.set_connection_configuration(
                                    crate::protocol_serde::shape_connection_configuration::de_connection_configuration(tokens)?
                                );
                            }
                            "VpcConfiguration" => {
                                builder = builder.set_vpc_configuration(
                                    crate::protocol_serde::shape_data_source_vpc_configuration::de_data_source_vpc_configuration(tokens)?
                                );
                            }
                            "ColumnConfiguration" => {
                                builder = builder.set_column_configuration(
                                    crate::protocol_serde::shape_column_configuration::de_column_configuration(tokens)?
                                );
                            }
                            "AclConfiguration" => {
                                builder = builder.set_acl_configuration(
                                    crate::protocol_serde::shape_acl_configuration::de_acl_configuration(tokens)?
                                );
                            }
                            "SqlConfiguration" => {
                                builder = builder.set_sql_configuration(
                                    crate::protocol_serde::shape_sql_configuration::de_sql_configuration(tokens)?
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
