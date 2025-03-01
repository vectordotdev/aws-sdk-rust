// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_luna_client_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_luna_client::DescribeLunaClientInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_arn {
        object.key("ClientArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.certificate_fingerprint {
        object.key("CertificateFingerprint").string(var_2.as_str());
    }
    Ok(())
}
