// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_put_raw_message_content_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutRawMessageContentInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.content {
        let mut object_2 = object.key("content").start_object();
        crate::json_ser::serialize_structure_crate_model_raw_message_content(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_raw_message_content(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RawMessageContent,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_3) = &input.s3_reference {
        let mut object_4 = object.key("s3Reference").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_reference(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_reference(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3Reference,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_5) = &input.bucket {
        object.key("bucket").string(var_5.as_str());
    }
    if let Some(var_6) = &input.key {
        object.key("key").string(var_6.as_str());
    }
    if let Some(var_7) = &input.object_version {
        object.key("objectVersion").string(var_7.as_str());
    }
    Ok(())
}
