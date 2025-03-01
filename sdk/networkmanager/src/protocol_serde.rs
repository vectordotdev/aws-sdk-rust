// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::http::HeaderMap,
    response_body: &[u8],
) -> Result<
    ::aws_smithy_types::error::metadata::Builder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_accept_attachment;

pub(crate) mod shape_associate_connect_peer;

pub(crate) mod shape_associate_customer_gateway;

pub(crate) mod shape_associate_link;

pub(crate) mod shape_associate_transit_gateway_connect_peer;

pub(crate) mod shape_create_connect_attachment;

pub(crate) mod shape_create_connect_peer;

pub(crate) mod shape_create_connection;

pub(crate) mod shape_create_core_network;

pub(crate) mod shape_create_device;

pub(crate) mod shape_create_global_network;

pub(crate) mod shape_create_link;

pub(crate) mod shape_create_site;

pub(crate) mod shape_create_site_to_site_vpn_attachment;

pub(crate) mod shape_create_transit_gateway_peering;

pub(crate) mod shape_create_transit_gateway_route_table_attachment;

pub(crate) mod shape_create_vpc_attachment;

pub(crate) mod shape_delete_attachment;

pub(crate) mod shape_delete_connect_peer;

pub(crate) mod shape_delete_connection;

pub(crate) mod shape_delete_core_network;

pub(crate) mod shape_delete_core_network_policy_version;

pub(crate) mod shape_delete_device;

pub(crate) mod shape_delete_global_network;

pub(crate) mod shape_delete_link;

pub(crate) mod shape_delete_peering;

pub(crate) mod shape_delete_resource_policy;

pub(crate) mod shape_delete_site;

pub(crate) mod shape_deregister_transit_gateway;

pub(crate) mod shape_describe_global_networks;

pub(crate) mod shape_disassociate_connect_peer;

pub(crate) mod shape_disassociate_customer_gateway;

pub(crate) mod shape_disassociate_link;

pub(crate) mod shape_disassociate_transit_gateway_connect_peer;

pub(crate) mod shape_execute_core_network_change_set;

pub(crate) mod shape_get_connect_attachment;

pub(crate) mod shape_get_connect_peer;

pub(crate) mod shape_get_connect_peer_associations;

pub(crate) mod shape_get_connections;

pub(crate) mod shape_get_core_network;

pub(crate) mod shape_get_core_network_change_events;

pub(crate) mod shape_get_core_network_change_set;

pub(crate) mod shape_get_core_network_policy;

pub(crate) mod shape_get_customer_gateway_associations;

pub(crate) mod shape_get_devices;

pub(crate) mod shape_get_link_associations;

pub(crate) mod shape_get_links;

pub(crate) mod shape_get_network_resource_counts;

pub(crate) mod shape_get_network_resource_relationships;

pub(crate) mod shape_get_network_resources;

pub(crate) mod shape_get_network_routes;

pub(crate) mod shape_get_network_telemetry;

pub(crate) mod shape_get_resource_policy;

pub(crate) mod shape_get_route_analysis;

pub(crate) mod shape_get_site_to_site_vpn_attachment;

pub(crate) mod shape_get_sites;

pub(crate) mod shape_get_transit_gateway_connect_peer_associations;

pub(crate) mod shape_get_transit_gateway_peering;

pub(crate) mod shape_get_transit_gateway_registrations;

pub(crate) mod shape_get_transit_gateway_route_table_attachment;

pub(crate) mod shape_get_vpc_attachment;

pub(crate) mod shape_list_attachments;

pub(crate) mod shape_list_connect_peers;

pub(crate) mod shape_list_core_network_policy_versions;

pub(crate) mod shape_list_core_networks;

pub(crate) mod shape_list_organization_service_access_status;

pub(crate) mod shape_list_peerings;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_put_core_network_policy;

pub(crate) mod shape_put_resource_policy;

pub(crate) mod shape_register_transit_gateway;

pub(crate) mod shape_reject_attachment;

pub(crate) mod shape_restore_core_network_policy_version;

pub(crate) mod shape_start_organization_service_access_update;

pub(crate) mod shape_start_route_analysis;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_connection;

pub(crate) mod shape_update_core_network;

pub(crate) mod shape_update_device;

pub(crate) mod shape_update_global_network;

pub(crate) mod shape_update_link;

pub(crate) mod shape_update_network_resource_metadata;

pub(crate) mod shape_update_site;

pub(crate) mod shape_update_vpc_attachment;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_associate_connect_peer_input;

pub(crate) mod shape_associate_customer_gateway_input;

pub(crate) mod shape_associate_link_input;

pub(crate) mod shape_associate_transit_gateway_connect_peer_input;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_core_network_policy_exception;

pub(crate) mod shape_create_connect_attachment_input;

pub(crate) mod shape_create_connect_peer_input;

pub(crate) mod shape_create_connection_input;

pub(crate) mod shape_create_core_network_input;

pub(crate) mod shape_create_device_input;

pub(crate) mod shape_create_global_network_input;

pub(crate) mod shape_create_link_input;

pub(crate) mod shape_create_site_input;

pub(crate) mod shape_create_site_to_site_vpn_attachment_input;

pub(crate) mod shape_create_transit_gateway_peering_input;

pub(crate) mod shape_create_transit_gateway_route_table_attachment_input;

pub(crate) mod shape_create_vpc_attachment_input;

pub(crate) mod shape_get_network_routes_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_put_core_network_policy_input;

pub(crate) mod shape_put_resource_policy_input;

pub(crate) mod shape_register_transit_gateway_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_start_organization_service_access_update_input;

pub(crate) mod shape_start_route_analysis_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_connection_input;

pub(crate) mod shape_update_core_network_input;

pub(crate) mod shape_update_device_input;

pub(crate) mod shape_update_global_network_input;

pub(crate) mod shape_update_link_input;

pub(crate) mod shape_update_network_resource_metadata_input;

pub(crate) mod shape_update_site_input;

pub(crate) mod shape_update_vpc_attachment_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_attachment;

pub(crate) mod shape_attachment_list;

pub(crate) mod shape_aws_location;

pub(crate) mod shape_bandwidth;

pub(crate) mod shape_bgp_options;

pub(crate) mod shape_connect_attachment;

pub(crate) mod shape_connect_attachment_options;

pub(crate) mod shape_connect_peer;

pub(crate) mod shape_connect_peer_association;

pub(crate) mod shape_connect_peer_association_list;

pub(crate) mod shape_connect_peer_summary_list;

pub(crate) mod shape_connection;

pub(crate) mod shape_connection_list;

pub(crate) mod shape_core_network;

pub(crate) mod shape_core_network_change_event_list;

pub(crate) mod shape_core_network_change_list;

pub(crate) mod shape_core_network_policy;

pub(crate) mod shape_core_network_policy_error_list;

pub(crate) mod shape_core_network_policy_version_list;

pub(crate) mod shape_core_network_segment_edge_identifier;

pub(crate) mod shape_core_network_summary_list;

pub(crate) mod shape_customer_gateway_association;

pub(crate) mod shape_customer_gateway_association_list;

pub(crate) mod shape_device;

pub(crate) mod shape_device_list;

pub(crate) mod shape_exception_context_map;

pub(crate) mod shape_global_network;

pub(crate) mod shape_global_network_list;

pub(crate) mod shape_link;

pub(crate) mod shape_link_association;

pub(crate) mod shape_link_association_list;

pub(crate) mod shape_link_list;

pub(crate) mod shape_location;

pub(crate) mod shape_network_resource_count_list;

pub(crate) mod shape_network_resource_list;

pub(crate) mod shape_network_resource_metadata_map;

pub(crate) mod shape_network_route_list;

pub(crate) mod shape_network_telemetry_list;

pub(crate) mod shape_organization_status;

pub(crate) mod shape_peering;

pub(crate) mod shape_peering_list;

pub(crate) mod shape_relationship_list;

pub(crate) mod shape_route_analysis;

pub(crate) mod shape_route_analysis_endpoint_options_specification;

pub(crate) mod shape_route_table_identifier;

pub(crate) mod shape_site;

pub(crate) mod shape_site_list;

pub(crate) mod shape_site_to_site_vpn_attachment;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_transit_gateway_connect_peer_association;

pub(crate) mod shape_transit_gateway_connect_peer_association_list;

pub(crate) mod shape_transit_gateway_peering;

pub(crate) mod shape_transit_gateway_registration;

pub(crate) mod shape_transit_gateway_registration_list;

pub(crate) mod shape_transit_gateway_route_table_attachment;

pub(crate) mod shape_validation_exception_field_list;

pub(crate) mod shape_vpc_attachment;

pub(crate) mod shape_vpc_options;

pub(crate) mod shape_account_status_list;

pub(crate) mod shape_connect_peer_configuration;

pub(crate) mod shape_connect_peer_summary;

pub(crate) mod shape_core_network_change;

pub(crate) mod shape_core_network_change_event;

pub(crate) mod shape_core_network_edge_list;

pub(crate) mod shape_core_network_policy_error;

pub(crate) mod shape_core_network_policy_version;

pub(crate) mod shape_core_network_segment_list;

pub(crate) mod shape_core_network_summary;

pub(crate) mod shape_network_resource;

pub(crate) mod shape_network_resource_count;

pub(crate) mod shape_network_route;

pub(crate) mod shape_network_telemetry;

pub(crate) mod shape_proposed_segment_change;

pub(crate) mod shape_relationship;

pub(crate) mod shape_route_analysis_endpoint_options;

pub(crate) mod shape_route_analysis_path;

pub(crate) mod shape_subnet_arn_list;

pub(crate) mod shape_transit_gateway_registration_state_reason;

pub(crate) mod shape_validation_exception_field;

pub(crate) mod shape_account_status;

pub(crate) mod shape_connect_peer_bgp_configuration_list;

pub(crate) mod shape_connection_health;

pub(crate) mod shape_constrained_string_list;

pub(crate) mod shape_core_network_change_event_values;

pub(crate) mod shape_core_network_change_values;

pub(crate) mod shape_core_network_edge;

pub(crate) mod shape_core_network_segment;

pub(crate) mod shape_network_route_destination_list;

pub(crate) mod shape_path_component_list;

pub(crate) mod shape_route_analysis_completion;

pub(crate) mod shape_connect_peer_bgp_configuration;

pub(crate) mod shape_external_region_code_list;

pub(crate) mod shape_network_route_destination;

pub(crate) mod shape_path_component;

pub(crate) mod shape_reason_context_map;

pub(crate) mod shape_network_resource_summary;
