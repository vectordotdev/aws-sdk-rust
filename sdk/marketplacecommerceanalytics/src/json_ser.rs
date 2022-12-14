// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_generate_data_set_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GenerateDataSetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.data_set_type {
        object.key("dataSetType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.data_set_publication_date {
        object
            .key("dataSetPublicationDate")
            .date_time(var_2, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_3) = &input.role_name_arn {
        object.key("roleNameArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.destination_s3_bucket_name {
        object.key("destinationS3BucketName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.destination_s3_prefix {
        object.key("destinationS3Prefix").string(var_5.as_str());
    }
    if let Some(var_6) = &input.sns_topic_arn {
        object.key("snsTopicArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.customer_defined_values {
        let mut object_8 = object.key("customerDefinedValues").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_support_data_export_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartSupportDataExportInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_11) = &input.data_set_type {
        object.key("dataSetType").string(var_11.as_str());
    }
    if let Some(var_12) = &input.from_date {
        object
            .key("fromDate")
            .date_time(var_12, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_13) = &input.role_name_arn {
        object.key("roleNameArn").string(var_13.as_str());
    }
    if let Some(var_14) = &input.destination_s3_bucket_name {
        object
            .key("destinationS3BucketName")
            .string(var_14.as_str());
    }
    if let Some(var_15) = &input.destination_s3_prefix {
        object.key("destinationS3Prefix").string(var_15.as_str());
    }
    if let Some(var_16) = &input.sns_topic_arn {
        object.key("snsTopicArn").string(var_16.as_str());
    }
    if let Some(var_17) = &input.customer_defined_values {
        let mut object_18 = object.key("customerDefinedValues").start_object();
        for (key_19, value_20) in var_17 {
            {
                object_18.key(key_19.as_str()).string(value_20.as_str());
            }
        }
        object_18.finish();
    }
    Ok(())
}
