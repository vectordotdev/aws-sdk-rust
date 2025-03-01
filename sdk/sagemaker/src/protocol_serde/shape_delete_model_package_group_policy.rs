// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_model_package_group_policy_input(
    input: &crate::operation::delete_model_package_group_policy::DeleteModelPackageGroupPolicyInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_model_package_group_policy_input::ser_delete_model_package_group_policy_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_model_package_group_policy_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_model_package_group_policy::DeleteModelPackageGroupPolicyOutput,
    crate::operation::delete_model_package_group_policy::DeleteModelPackageGroupPolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::delete_model_package_group_policy::DeleteModelPackageGroupPolicyError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::delete_model_package_group_policy::DeleteModelPackageGroupPolicyError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_model_package_group_policy_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_model_package_group_policy::DeleteModelPackageGroupPolicyOutput,
    crate::operation::delete_model_package_group_policy::DeleteModelPackageGroupPolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_model_package_group_policy::builders::DeleteModelPackageGroupPolicyOutputBuilder::default();
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
