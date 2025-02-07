// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_end_of_meeting_reminder(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CreateEndOfMeetingReminder,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.reminder_at_minutes {
        let mut array_2 = object.key("ReminderAtMinutes").start_array();
        for item_3 in var_1 {
            {
                array_2.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::NegInt((*item_3).into()),
                );
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.reminder_type {
        object.key("ReminderType").string(var_4.as_str());
    }
    if let Some(var_5) = &input.enabled {
        object.key("Enabled").boolean(*var_5);
    }
    Ok(())
}
