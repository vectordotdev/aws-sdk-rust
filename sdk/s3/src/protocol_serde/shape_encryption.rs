// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_encryption(
    input: &crate::types::Encryption,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.encryption_type {
        let mut inner_writer = scope.start_el("EncryptionType").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.kms_key_id {
        let mut inner_writer = scope.start_el("KMSKeyId").finish();
        inner_writer.data(var_2.as_str());
    }
    if let Some(var_3) = &input.kms_context {
        let mut inner_writer = scope.start_el("KMSContext").finish();
        inner_writer.data(var_3.as_str());
    }
    scope.finish();
    Ok(())
}
