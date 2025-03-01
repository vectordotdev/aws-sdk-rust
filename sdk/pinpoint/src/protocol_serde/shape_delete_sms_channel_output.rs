// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_sms_channel_response_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::SmsChannelResponse>,
    crate::operation::delete_sms_channel::DeleteSmsChannelError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_sms_channel_response::de_sms_channel_response_payload(body)
                .map_err(crate::operation::delete_sms_channel::DeleteSmsChannelError::unhandled)
        })
        .transpose()
}
