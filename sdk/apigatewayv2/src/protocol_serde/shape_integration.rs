// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_integration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::Integration>,
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
            let mut builder = crate::types::builders::IntegrationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key
                        .to_unescaped()?
                        .as_ref()
                    {
                        "apiGatewayManaged" => {
                            builder = builder.set_api_gateway_managed(
                                ::aws_smithy_json::deserialize::token::expect_bool_or_null(
                                    tokens.next(),
                                )?,
                            );
                        }
                        "connectionId" => {
                            builder = builder.set_connection_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "connectionType" => {
                            builder = builder.set_connection_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| {
                                    s.to_unescaped()
                                        .map(|u| crate::types::ConnectionType::from(u.as_ref()))
                                })
                                .transpose()?,
                            );
                        }
                        "contentHandlingStrategy" => {
                            builder = builder.set_content_handling_strategy(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| {
                                    s.to_unescaped().map(|u| {
                                        crate::types::ContentHandlingStrategy::from(u.as_ref())
                                    })
                                })
                                .transpose()?,
                            );
                        }
                        "credentialsArn" => {
                            builder = builder.set_credentials_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
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
                        "integrationId" => {
                            builder = builder.set_integration_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "integrationMethod" => {
                            builder = builder.set_integration_method(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "integrationResponseSelectionExpression" => {
                            builder = builder.set_integration_response_selection_expression(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "integrationSubtype" => {
                            builder = builder.set_integration_subtype(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "integrationType" => {
                            builder = builder.set_integration_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| {
                                    s.to_unescaped()
                                        .map(|u| crate::types::IntegrationType::from(u.as_ref()))
                                })
                                .transpose()?,
                            );
                        }
                        "integrationUri" => {
                            builder = builder.set_integration_uri(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "passthroughBehavior" => {
                            builder = builder.set_passthrough_behavior(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| {
                                    s.to_unescaped().map(|u| {
                                        crate::types::PassthroughBehavior::from(u.as_ref())
                                    })
                                })
                                .transpose()?,
                            );
                        }
                        "payloadFormatVersion" => {
                            builder = builder.set_payload_format_version(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "requestParameters" => {
                            builder = builder.set_request_parameters(
                                    crate::protocol_serde::shape_integration_parameters::de_integration_parameters(tokens)?
                                );
                        }
                        "requestTemplates" => {
                            builder = builder.set_request_templates(
                                crate::protocol_serde::shape_template_map::de_template_map(tokens)?,
                            );
                        }
                        "responseParameters" => {
                            builder = builder.set_response_parameters(
                                    crate::protocol_serde::shape_response_parameters::de_response_parameters(tokens)?
                                );
                        }
                        "templateSelectionExpression" => {
                            builder = builder.set_template_selection_expression(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "timeoutInMillis" => {
                            builder = builder.set_timeout_in_millis(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                    tokens.next(),
                                )?
                                .map(i32::try_from)
                                .transpose()?,
                            );
                        }
                        "tlsConfig" => {
                            builder = builder.set_tls_config(
                                crate::protocol_serde::shape_tls_config::de_tls_config(tokens)?,
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
