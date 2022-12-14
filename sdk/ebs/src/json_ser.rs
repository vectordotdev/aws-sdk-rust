// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_start_snapshot_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartSnapshotInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("ClientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.encrypted {
        object.key("Encrypted").boolean(*var_3);
    }
    if let Some(var_4) = &input.kms_key_arn {
        object.key("KmsKeyArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.parent_snapshot_id {
        object.key("ParentSnapshotId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        let mut array_7 = object.key("Tags").start_array();
        for item_8 in var_6 {
            {
                let mut object_9 = array_7.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.timeout {
        object.key("Timeout").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_10).into()),
        );
    }
    if let Some(var_11) = &input.volume_size {
        object.key("VolumeSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.key {
        object.key("Key").string(var_12.as_str());
    }
    if let Some(var_13) = &input.value {
        object.key("Value").string(var_13.as_str());
    }
    Ok(())
}
