// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_what_if_forecast_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_what_if_forecast::DeleteWhatIfForecastInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.what_if_forecast_arn {
        object.key("WhatIfForecastArn").string(var_1.as_str());
    }
    Ok(())
}
