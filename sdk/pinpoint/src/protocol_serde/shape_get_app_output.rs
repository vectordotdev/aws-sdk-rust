// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_application_response_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::ApplicationResponse>,
    crate::operation::get_app::GetAppError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_application_response::de_application_response_payload(body)
                .map_err(crate::operation::get_app::GetAppError::unhandled)
        })
        .transpose()
}
