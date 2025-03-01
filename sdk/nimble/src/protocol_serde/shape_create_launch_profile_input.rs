// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_launch_profile_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_launch_profile::CreateLaunchProfileInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.ec2_subnet_ids {
        let mut array_3 = object.key("ec2SubnetIds").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.launch_profile_protocol_versions {
        let mut array_6 = object.key("launchProfileProtocolVersions").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.name {
        object.key("name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.stream_configuration {
        #[allow(unused_mut)]
        let mut object_10 = object.key("streamConfiguration").start_object();
        crate::protocol_serde::shape_stream_configuration_create::ser_stream_configuration_create(
            &mut object_10,
            var_9,
        )?;
        object_10.finish();
    }
    if let Some(var_11) = &input.studio_component_ids {
        let mut array_12 = object.key("studioComponentIds").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    if let Some(var_14) = &input.tags {
        #[allow(unused_mut)]
        let mut object_15 = object.key("tags").start_object();
        for (key_16, value_17) in var_14 {
            {
                object_15.key(key_16.as_str()).string(value_17.as_str());
            }
        }
        object_15.finish();
    }
    Ok(())
}
