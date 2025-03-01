// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_gnss(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Gnss,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.payload {
        object.key("Payload").string(var_1.as_str());
    }
    if let Some(var_2) = &input.capture_time {
        object.key("CaptureTime").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.capture_time_accuracy {
        object.key("CaptureTimeAccuracy").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.assist_position {
        let mut array_5 = object.key("AssistPosition").start_array();
        for item_6 in var_4 {
            {
                array_5.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::Float((*item_6).into()),
                );
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.assist_altitude {
        object.key("AssistAltitude").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_7).into()),
        );
    }
    if input.use2_d_solver {
        object.key("Use2DSolver").boolean(input.use2_d_solver);
    }
    Ok(())
}
