// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_user_settings_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_user_settings::CreateUserSettingsInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.copy_allowed {
        object.key("copyAllowed").string(var_2.as_str());
    }
    if let Some(var_3) = &input.disconnect_timeout_in_minutes {
        object.key("disconnectTimeoutInMinutes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.download_allowed {
        object.key("downloadAllowed").string(var_4.as_str());
    }
    if let Some(var_5) = &input.idle_disconnect_timeout_in_minutes {
        object.key("idleDisconnectTimeoutInMinutes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.paste_allowed {
        object.key("pasteAllowed").string(var_6.as_str());
    }
    if let Some(var_7) = &input.print_allowed {
        object.key("printAllowed").string(var_7.as_str());
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("tags").start_array();
        for item_10 in var_8 {
            {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.upload_allowed {
        object.key("uploadAllowed").string(var_12.as_str());
    }
    Ok(())
}
