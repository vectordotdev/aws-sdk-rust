// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_apns_sandbox_channel_response_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::ApnsSandboxChannelResponse>,
    crate::operation::delete_apns_sandbox_channel::DeleteApnsSandboxChannelError,
> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_apns_sandbox_channel_response::de_apns_sandbox_channel_response_payload(body).map_err(crate::operation::delete_apns_sandbox_channel::DeleteApnsSandboxChannelError::unhandled)
    }).transpose()
}
