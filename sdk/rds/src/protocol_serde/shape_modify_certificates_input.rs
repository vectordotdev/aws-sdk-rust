// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_certificates_input_input(
    input: &crate::operation::modify_certificates::ModifyCertificatesInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyCertificates", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CertificateIdentifier");
    if let Some(var_2) = &input.certificate_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("RemoveCustomerOverride");
    if let Some(var_4) = &input.remove_customer_override {
        scope_3.boolean(*var_4);
    }
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
