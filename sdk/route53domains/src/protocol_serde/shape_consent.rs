// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_consent(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Consent,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    {
        object.key("MaxPrice").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((input.max_price).into()),
        );
    }
    if let Some(var_1) = &input.currency {
        object.key("Currency").string(var_1.as_str());
    }
    Ok(())
}
