// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_template_input_input(
    input: &crate::operation::get_template::GetTemplateInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetTemplate", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TemplateName");
    if let Some(var_2) = &input.template_name {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
