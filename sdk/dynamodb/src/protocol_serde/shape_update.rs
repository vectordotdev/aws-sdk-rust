// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Update,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.key {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Key").start_object();
        for (key_3, value_4) in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_5 = object_2.key(key_3.as_str()).start_object();
                crate::protocol_serde::shape_attribute_value::ser_attribute_value(
                    &mut object_5,
                    value_4,
                )?;
                object_5.finish();
            }
        }
        object_2.finish();
    }
    if let Some(var_6) = &input.update_expression {
        object.key("UpdateExpression").string(var_6.as_str());
    }
    if let Some(var_7) = &input.table_name {
        object.key("TableName").string(var_7.as_str());
    }
    if let Some(var_8) = &input.condition_expression {
        object.key("ConditionExpression").string(var_8.as_str());
    }
    if let Some(var_9) = &input.expression_attribute_names {
        #[allow(unused_mut)]
        let mut object_10 = object.key("ExpressionAttributeNames").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    if let Some(var_13) = &input.expression_attribute_values {
        #[allow(unused_mut)]
        let mut object_14 = object.key("ExpressionAttributeValues").start_object();
        for (key_15, value_16) in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_17 = object_14.key(key_15.as_str()).start_object();
                crate::protocol_serde::shape_attribute_value::ser_attribute_value(
                    &mut object_17,
                    value_16,
                )?;
                object_17.finish();
            }
        }
        object_14.finish();
    }
    if let Some(var_18) = &input.return_values_on_condition_check_failure {
        object
            .key("ReturnValuesOnConditionCheckFailure")
            .string(var_18.as_str());
    }
    Ok(())
}
