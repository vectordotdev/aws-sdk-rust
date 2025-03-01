// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_dynamo_db_table_details(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsDynamoDbTableDetails,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.attribute_definitions {
        let mut array_2 = object.key("AttributeDefinitions").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_aws_dynamo_db_table_attribute_definition::ser_aws_dynamo_db_table_attribute_definition(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.billing_mode_summary {
        #[allow(unused_mut)]
        let mut object_6 = object.key("BillingModeSummary").start_object();
        crate::protocol_serde::shape_aws_dynamo_db_table_billing_mode_summary::ser_aws_dynamo_db_table_billing_mode_summary(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.creation_date_time {
        object.key("CreationDateTime").string(var_7.as_str());
    }
    if let Some(var_8) = &input.global_secondary_indexes {
        let mut array_9 = object.key("GlobalSecondaryIndexes").start_array();
        for item_10 in var_8 {
            {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_aws_dynamo_db_table_global_secondary_index::ser_aws_dynamo_db_table_global_secondary_index(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.global_table_version {
        object.key("GlobalTableVersion").string(var_12.as_str());
    }
    if input.item_count != 0 {
        object.key("ItemCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.item_count).into()),
        );
    }
    if let Some(var_13) = &input.key_schema {
        let mut array_14 = object.key("KeySchema").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_aws_dynamo_db_table_key_schema::ser_aws_dynamo_db_table_key_schema(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.latest_stream_arn {
        object.key("LatestStreamArn").string(var_17.as_str());
    }
    if let Some(var_18) = &input.latest_stream_label {
        object.key("LatestStreamLabel").string(var_18.as_str());
    }
    if let Some(var_19) = &input.local_secondary_indexes {
        let mut array_20 = object.key("LocalSecondaryIndexes").start_array();
        for item_21 in var_19 {
            {
                #[allow(unused_mut)]
                let mut object_22 = array_20.value().start_object();
                crate::protocol_serde::shape_aws_dynamo_db_table_local_secondary_index::ser_aws_dynamo_db_table_local_secondary_index(&mut object_22, item_21)?;
                object_22.finish();
            }
        }
        array_20.finish();
    }
    if let Some(var_23) = &input.provisioned_throughput {
        #[allow(unused_mut)]
        let mut object_24 = object.key("ProvisionedThroughput").start_object();
        crate::protocol_serde::shape_aws_dynamo_db_table_provisioned_throughput::ser_aws_dynamo_db_table_provisioned_throughput(&mut object_24, var_23)?;
        object_24.finish();
    }
    if let Some(var_25) = &input.replicas {
        let mut array_26 = object.key("Replicas").start_array();
        for item_27 in var_25 {
            {
                #[allow(unused_mut)]
                let mut object_28 = array_26.value().start_object();
                crate::protocol_serde::shape_aws_dynamo_db_table_replica::ser_aws_dynamo_db_table_replica(&mut object_28, item_27)?;
                object_28.finish();
            }
        }
        array_26.finish();
    }
    if let Some(var_29) = &input.restore_summary {
        #[allow(unused_mut)]
        let mut object_30 = object.key("RestoreSummary").start_object();
        crate::protocol_serde::shape_aws_dynamo_db_table_restore_summary::ser_aws_dynamo_db_table_restore_summary(&mut object_30, var_29)?;
        object_30.finish();
    }
    if let Some(var_31) = &input.sse_description {
        #[allow(unused_mut)]
        let mut object_32 = object.key("SseDescription").start_object();
        crate::protocol_serde::shape_aws_dynamo_db_table_sse_description::ser_aws_dynamo_db_table_sse_description(&mut object_32, var_31)?;
        object_32.finish();
    }
    if let Some(var_33) = &input.stream_specification {
        #[allow(unused_mut)]
        let mut object_34 = object.key("StreamSpecification").start_object();
        crate::protocol_serde::shape_aws_dynamo_db_table_stream_specification::ser_aws_dynamo_db_table_stream_specification(&mut object_34, var_33)?;
        object_34.finish();
    }
    if let Some(var_35) = &input.table_id {
        object.key("TableId").string(var_35.as_str());
    }
    if let Some(var_36) = &input.table_name {
        object.key("TableName").string(var_36.as_str());
    }
    if input.table_size_bytes != 0 {
        object.key("TableSizeBytes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.table_size_bytes).into()),
        );
    }
    if let Some(var_37) = &input.table_status {
        object.key("TableStatus").string(var_37.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_dynamo_db_table_details<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::AwsDynamoDbTableDetails>,
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
            let mut builder = crate::types::builders::AwsDynamoDbTableDetailsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "AttributeDefinitions" => {
                                builder = builder.set_attribute_definitions(
                                    crate::protocol_serde::shape_aws_dynamo_db_table_attribute_definition_list::de_aws_dynamo_db_table_attribute_definition_list(tokens)?
                                );
                            }
                            "BillingModeSummary" => {
                                builder = builder.set_billing_mode_summary(
                                    crate::protocol_serde::shape_aws_dynamo_db_table_billing_mode_summary::de_aws_dynamo_db_table_billing_mode_summary(tokens)?
                                );
                            }
                            "CreationDateTime" => {
                                builder = builder.set_creation_date_time(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "GlobalSecondaryIndexes" => {
                                builder = builder.set_global_secondary_indexes(
                                    crate::protocol_serde::shape_aws_dynamo_db_table_global_secondary_index_list::de_aws_dynamo_db_table_global_secondary_index_list(tokens)?
                                );
                            }
                            "GlobalTableVersion" => {
                                builder = builder.set_global_table_version(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ItemCount" => {
                                builder = builder.set_item_count(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "KeySchema" => {
                                builder = builder.set_key_schema(
                                    crate::protocol_serde::shape_aws_dynamo_db_table_key_schema_list::de_aws_dynamo_db_table_key_schema_list(tokens)?
                                );
                            }
                            "LatestStreamArn" => {
                                builder = builder.set_latest_stream_arn(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "LatestStreamLabel" => {
                                builder = builder.set_latest_stream_label(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "LocalSecondaryIndexes" => {
                                builder = builder.set_local_secondary_indexes(
                                    crate::protocol_serde::shape_aws_dynamo_db_table_local_secondary_index_list::de_aws_dynamo_db_table_local_secondary_index_list(tokens)?
                                );
                            }
                            "ProvisionedThroughput" => {
                                builder = builder.set_provisioned_throughput(
                                    crate::protocol_serde::shape_aws_dynamo_db_table_provisioned_throughput::de_aws_dynamo_db_table_provisioned_throughput(tokens)?
                                );
                            }
                            "Replicas" => {
                                builder = builder.set_replicas(
                                    crate::protocol_serde::shape_aws_dynamo_db_table_replica_list::de_aws_dynamo_db_table_replica_list(tokens)?
                                );
                            }
                            "RestoreSummary" => {
                                builder = builder.set_restore_summary(
                                    crate::protocol_serde::shape_aws_dynamo_db_table_restore_summary::de_aws_dynamo_db_table_restore_summary(tokens)?
                                );
                            }
                            "SseDescription" => {
                                builder = builder.set_sse_description(
                                    crate::protocol_serde::shape_aws_dynamo_db_table_sse_description::de_aws_dynamo_db_table_sse_description(tokens)?
                                );
                            }
                            "StreamSpecification" => {
                                builder = builder.set_stream_specification(
                                    crate::protocol_serde::shape_aws_dynamo_db_table_stream_specification::de_aws_dynamo_db_table_stream_specification(tokens)?
                                );
                            }
                            "TableId" => {
                                builder = builder.set_table_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "TableName" => {
                                builder = builder.set_table_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "TableSizeBytes" => {
                                builder = builder.set_table_size_bytes(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i64::try_from)
                                    .transpose()?,
                                );
                            }
                            "TableStatus" => {
                                builder = builder.set_table_status(
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
