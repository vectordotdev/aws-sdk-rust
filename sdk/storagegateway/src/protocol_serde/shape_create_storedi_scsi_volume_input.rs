// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_storedi_scsi_volume_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_storedi_scsi_volume::CreateStorediScsiVolumeInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.gateway_arn {
        object.key("GatewayARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.disk_id {
        object.key("DiskId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.snapshot_id {
        object.key("SnapshotId").string(var_3.as_str());
    }
    {
        object
            .key("PreserveExistingData")
            .boolean(input.preserve_existing_data);
    }
    if let Some(var_4) = &input.target_name {
        object.key("TargetName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.network_interface_id {
        object.key("NetworkInterfaceId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.kms_encrypted {
        object.key("KMSEncrypted").boolean(*var_6);
    }
    if let Some(var_7) = &input.kms_key {
        object.key("KMSKey").string(var_7.as_str());
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("Tags").start_array();
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
    Ok(())
}
