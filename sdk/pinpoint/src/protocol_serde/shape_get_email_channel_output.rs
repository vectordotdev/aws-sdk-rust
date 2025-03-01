// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_email_channel_response_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::EmailChannelResponse>,
    crate::operation::get_email_channel::GetEmailChannelError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_email_channel_response::de_email_channel_response_payload(
                body,
            )
            .map_err(crate::operation::get_email_channel::GetEmailChannelError::unhandled)
        })
        .transpose()
}
