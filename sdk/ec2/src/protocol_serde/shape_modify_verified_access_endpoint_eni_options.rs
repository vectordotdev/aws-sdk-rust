// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_modify_verified_access_endpoint_eni_options(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::ModifyVerifiedAccessEndpointEniOptions,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Protocol");
    if let Some(var_2) = &input.protocol {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Port");
    if let Some(var_4) = &input.port {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    Ok(())
}
