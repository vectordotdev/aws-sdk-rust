// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_virtual_router_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::VirtualRouterData>,
    crate::operation::update_virtual_router::UpdateVirtualRouterError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_virtual_router_data::de_virtual_router_data_payload(body)
                .map_err(
                    crate::operation::update_virtual_router::UpdateVirtualRouterError::unhandled,
                )
        })
        .transpose()
}
