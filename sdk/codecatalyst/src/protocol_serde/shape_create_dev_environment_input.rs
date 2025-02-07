// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_dev_environment_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_dev_environment::CreateDevEnvironmentInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.alias {
        object.key("alias").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_token {
        object.key("clientToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.ides {
        let mut array_4 = object.key("ides").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_ide_configuration::ser_ide_configuration(
                    &mut object_6,
                    item_5,
                )?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if input.inactivity_timeout_minutes != 0 {
        object.key("inactivityTimeoutMinutes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.inactivity_timeout_minutes).into()),
        );
    }
    if let Some(var_7) = &input.instance_type {
        object.key("instanceType").string(var_7.as_str());
    }
    if let Some(var_8) = &input.persistent_storage {
        #[allow(unused_mut)]
        let mut object_9 = object.key("persistentStorage").start_object();
        crate::protocol_serde::shape_persistent_storage_configuration::ser_persistent_storage_configuration(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.repositories {
        let mut array_11 = object.key("repositories").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_repository_input::ser_repository_input(
                    &mut object_13,
                    item_12,
                )?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}
