// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_database_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DatabaseInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.location_uri {
        object.key("LocationUri").string(var_3.as_str());
    }
    if let Some(var_4) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Parameters").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.create_table_default_permissions {
        let mut array_9 = object.key("CreateTableDefaultPermissions").start_array();
        for item_10 in var_8 {
            {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_principal_permissions::ser_principal_permissions(
                    &mut object_11,
                    item_10,
                )?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.target_database {
        #[allow(unused_mut)]
        let mut object_13 = object.key("TargetDatabase").start_object();
        crate::protocol_serde::shape_database_identifier::ser_database_identifier(
            &mut object_13,
            var_12,
        )?;
        object_13.finish();
    }
    if let Some(var_14) = &input.federated_database {
        #[allow(unused_mut)]
        let mut object_15 = object.key("FederatedDatabase").start_object();
        crate::protocol_serde::shape_federated_database::ser_federated_database(
            &mut object_15,
            var_14,
        )?;
        object_15.finish();
    }
    Ok(())
}
