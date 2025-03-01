// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_generate_service_last_accessed_details_input_input(
    input: &crate::operation::generate_service_last_accessed_details::GenerateServiceLastAccessedDetailsInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(
        &mut out,
        "GenerateServiceLastAccessedDetails",
        "2010-05-08",
    );
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Arn");
    if let Some(var_2) = &input.arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Granularity");
    if let Some(var_4) = &input.granularity {
        scope_3.string(var_4.as_str());
    }
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
