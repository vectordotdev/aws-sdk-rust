// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Get,
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
    if let Some(var_6) = &input.table_name {
        object.key("TableName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.projection_expression {
        object.key("ProjectionExpression").string(var_7.as_str());
    }
    if let Some(var_8) = &input.expression_attribute_names {
        #[allow(unused_mut)]
        let mut object_9 = object.key("ExpressionAttributeNames").start_object();
        for (key_10, value_11) in var_8 {
            {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    Ok(())
}
