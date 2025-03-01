// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_new_public_virtual_interface_allocation(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::NewPublicVirtualInterfaceAllocation,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.virtual_interface_name {
        object.key("virtualInterfaceName").string(var_1.as_str());
    }
    {
        object.key("vlan").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.vlan).into()),
        );
    }
    {
        object.key("asn").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.asn).into()),
        );
    }
    if let Some(var_2) = &input.auth_key {
        object.key("authKey").string(var_2.as_str());
    }
    if let Some(var_3) = &input.amazon_address {
        object.key("amazonAddress").string(var_3.as_str());
    }
    if let Some(var_4) = &input.customer_address {
        object.key("customerAddress").string(var_4.as_str());
    }
    if let Some(var_5) = &input.address_family {
        object.key("addressFamily").string(var_5.as_str());
    }
    if let Some(var_6) = &input.route_filter_prefixes {
        let mut array_7 = object.key("routeFilterPrefixes").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_route_filter_prefix::ser_route_filter_prefix(
                    &mut object_9,
                    item_8,
                )?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.tags {
        let mut array_11 = object.key("tags").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}
