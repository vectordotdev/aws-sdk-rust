// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_business_report_schedule_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_business_report_schedule::CreateBusinessReportScheduleInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.schedule_name {
        object.key("ScheduleName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.s3_bucket_name {
        object.key("S3BucketName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.s3_key_prefix {
        object.key("S3KeyPrefix").string(var_3.as_str());
    }
    if let Some(var_4) = &input.format {
        object.key("Format").string(var_4.as_str());
    }
    if let Some(var_5) = &input.content_range {
        #[allow(unused_mut)]
        let mut object_6 = object.key("ContentRange").start_object();
        crate::protocol_serde::shape_business_report_content_range::ser_business_report_content_range(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.recurrence {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Recurrence").start_object();
        crate::protocol_serde::shape_business_report_recurrence::ser_business_report_recurrence(
            &mut object_8,
            var_7,
        )?;
        object_8.finish();
    }
    if let Some(var_9) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_9.as_str());
    }
    if let Some(var_10) = &input.tags {
        let mut array_11 = object.key("Tags").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}
