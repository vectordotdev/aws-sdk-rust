// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_modify_verified_access_endpoint_load_balancer_options(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::ModifyVerifiedAccessEndpointLoadBalancerOptions,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SubnetId");
    if let Some(var_2) = &input.subnet_ids {
        let mut list_4 = scope_1.start_list(true, Some("item"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("Protocol");
    if let Some(var_7) = &input.protocol {
        scope_6.string(var_7.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("Port");
    if let Some(var_9) = &input.port {
        scope_8.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_9).into()),
        );
    }
    Ok(())
}
