// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetNetworkRoutes`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_network_id(impl ::std::convert::Into<String>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::global_network_id) / [`set_global_network_id(Option<String>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::set_global_network_id): <p>The ID of the global network.</p>
    ///   - [`route_table_identifier(RouteTableIdentifier)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::route_table_identifier) / [`set_route_table_identifier(Option<RouteTableIdentifier>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::set_route_table_identifier): <p>The ID of the route table.</p>
    ///   - [`exact_cidr_matches(Vec<String>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::exact_cidr_matches) / [`set_exact_cidr_matches(Option<Vec<String>>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::set_exact_cidr_matches): <p>An exact CIDR block.</p>
    ///   - [`longest_prefix_matches(Vec<String>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::longest_prefix_matches) / [`set_longest_prefix_matches(Option<Vec<String>>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::set_longest_prefix_matches): <p>The most specific route that matches the traffic (longest prefix match).</p>
    ///   - [`subnet_of_matches(Vec<String>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::subnet_of_matches) / [`set_subnet_of_matches(Option<Vec<String>>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::set_subnet_of_matches): <p>The routes with a subnet that match the specified CIDR filter.</p>
    ///   - [`supernet_of_matches(Vec<String>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::supernet_of_matches) / [`set_supernet_of_matches(Option<Vec<String>>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::set_supernet_of_matches): <p>The routes with a CIDR that encompasses the CIDR filter. Example: If you specify 10.0.1.0/30, then the result returns 10.0.1.0/29.</p>
    ///   - [`prefix_list_ids(Vec<String>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::prefix_list_ids) / [`set_prefix_list_ids(Option<Vec<String>>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::set_prefix_list_ids): <p>The IDs of the prefix lists.</p>
    ///   - [`states(Vec<RouteState>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::states) / [`set_states(Option<Vec<RouteState>>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::set_states): <p>The route states.</p>
    ///   - [`types(Vec<RouteType>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::types) / [`set_types(Option<Vec<RouteType>>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::set_types): <p>The route types.</p>
    ///   - [`destination_filters(HashMap<String, Vec<String>>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::destination_filters) / [`set_destination_filters(Option<HashMap<String, Vec<String>>>)`](crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::set_destination_filters): <p>Filter by route table destination. Possible Values: TRANSIT_GATEWAY_ATTACHMENT_ID, RESOURCE_ID, or RESOURCE_TYPE.</p>
    /// - On success, responds with [`GetNetworkRoutesOutput`](crate::operation::get_network_routes::GetNetworkRoutesOutput) with field(s):
    ///   - [`route_table_arn(Option<String>)`](crate::operation::get_network_routes::GetNetworkRoutesOutput::route_table_arn): <p>The ARN of the route table.</p>
    ///   - [`core_network_segment_edge(Option<CoreNetworkSegmentEdgeIdentifier>)`](crate::operation::get_network_routes::GetNetworkRoutesOutput::core_network_segment_edge): <p>Describes a core network segment edge.</p>
    ///   - [`route_table_type(Option<RouteTableType>)`](crate::operation::get_network_routes::GetNetworkRoutesOutput::route_table_type): <p>The route table type.</p>
    ///   - [`route_table_timestamp(Option<DateTime>)`](crate::operation::get_network_routes::GetNetworkRoutesOutput::route_table_timestamp): <p>The route table creation time.</p>
    ///   - [`network_routes(Option<Vec<NetworkRoute>>)`](crate::operation::get_network_routes::GetNetworkRoutesOutput::network_routes): <p>The network routes.</p>
    /// - On failure, responds with [`SdkError<GetNetworkRoutesError>`](crate::operation::get_network_routes::GetNetworkRoutesError)
    pub fn get_network_routes(
        &self,
    ) -> crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder {
        crate::operation::get_network_routes::builders::GetNetworkRoutesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
