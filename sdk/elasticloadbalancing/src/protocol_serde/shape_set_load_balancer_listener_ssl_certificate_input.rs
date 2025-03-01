// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_set_load_balancer_listener_ssl_certificate_input_input(
    input: &crate::operation::set_load_balancer_listener_ssl_certificate::SetLoadBalancerListenerSslCertificateInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(
        &mut out,
        "SetLoadBalancerListenerSSLCertificate",
        "2012-06-01",
    );
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("LoadBalancerName");
    if let Some(var_2) = &input.load_balancer_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("LoadBalancerPort");
    {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.load_balancer_port).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_4 = writer.prefix("SSLCertificateId");
    if let Some(var_5) = &input.ssl_certificate_id {
        scope_4.string(var_5);
    }
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
