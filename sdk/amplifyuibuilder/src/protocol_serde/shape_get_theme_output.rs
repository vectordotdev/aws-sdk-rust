// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_theme_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::Theme>,
    crate::operation::get_theme::GetThemeError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_theme::de_theme_payload(body)
                .map_err(crate::operation::get_theme::GetThemeError::unhandled)
        })
        .transpose()
}
