// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_ipam_cidr_authorization_context(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::IpamCidrAuthorizationContext,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Message");
    if let Some(var_2) = &input.message {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Signature");
    if let Some(var_4) = &input.signature {
        scope_3.string(var_4);
    }
    Ok(())
}
