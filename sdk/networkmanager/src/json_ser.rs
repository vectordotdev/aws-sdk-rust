// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_customer_gateway_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateCustomerGatewayInput,
) {
    if let Some(var_1) = &input.customer_gateway_arn {
        object.key("CustomerGatewayArn").string(var_1);
    }
    if let Some(var_2) = &input.device_id {
        object.key("DeviceId").string(var_2);
    }
    if let Some(var_3) = &input.link_id {
        object.key("LinkId").string(var_3);
    }
}

pub fn serialize_structure_crate_input_associate_link_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateLinkInput,
) {
    if let Some(var_4) = &input.device_id {
        object.key("DeviceId").string(var_4);
    }
    if let Some(var_5) = &input.link_id {
        object.key("LinkId").string(var_5);
    }
}

pub fn serialize_structure_crate_input_associate_transit_gateway_connect_peer_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateTransitGatewayConnectPeerInput,
) {
    if let Some(var_6) = &input.device_id {
        object.key("DeviceId").string(var_6);
    }
    if let Some(var_7) = &input.link_id {
        object.key("LinkId").string(var_7);
    }
    if let Some(var_8) = &input.transit_gateway_connect_peer_arn {
        object.key("TransitGatewayConnectPeerArn").string(var_8);
    }
}

pub fn serialize_structure_crate_input_create_connection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateConnectionInput,
) {
    if let Some(var_9) = &input.connected_device_id {
        object.key("ConnectedDeviceId").string(var_9);
    }
    if let Some(var_10) = &input.connected_link_id {
        object.key("ConnectedLinkId").string(var_10);
    }
    if let Some(var_11) = &input.description {
        object.key("Description").string(var_11);
    }
    if let Some(var_12) = &input.device_id {
        object.key("DeviceId").string(var_12);
    }
    if let Some(var_13) = &input.link_id {
        object.key("LinkId").string(var_13);
    }
    if let Some(var_14) = &input.tags {
        let mut array_15 = object.key("Tags").start_array();
        for item_16 in var_14 {
            {
                let mut object_17 = array_15.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_17, item_16);
                object_17.finish();
            }
        }
        array_15.finish();
    }
}

pub fn serialize_structure_crate_input_create_device_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDeviceInput,
) {
    if let Some(var_18) = &input.aws_location {
        let mut object_19 = object.key("AWSLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_aws_location(&mut object_19, var_18);
        object_19.finish();
    }
    if let Some(var_20) = &input.description {
        object.key("Description").string(var_20);
    }
    if let Some(var_21) = &input.location {
        let mut object_22 = object.key("Location").start_object();
        crate::json_ser::serialize_structure_crate_model_location(&mut object_22, var_21);
        object_22.finish();
    }
    if let Some(var_23) = &input.model {
        object.key("Model").string(var_23);
    }
    if let Some(var_24) = &input.serial_number {
        object.key("SerialNumber").string(var_24);
    }
    if let Some(var_25) = &input.site_id {
        object.key("SiteId").string(var_25);
    }
    if let Some(var_26) = &input.tags {
        let mut array_27 = object.key("Tags").start_array();
        for item_28 in var_26 {
            {
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_29, item_28);
                object_29.finish();
            }
        }
        array_27.finish();
    }
    if let Some(var_30) = &input.r#type {
        object.key("Type").string(var_30);
    }
    if let Some(var_31) = &input.vendor {
        object.key("Vendor").string(var_31);
    }
}

pub fn serialize_structure_crate_input_create_global_network_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateGlobalNetworkInput,
) {
    if let Some(var_32) = &input.description {
        object.key("Description").string(var_32);
    }
    if let Some(var_33) = &input.tags {
        let mut array_34 = object.key("Tags").start_array();
        for item_35 in var_33 {
            {
                let mut object_36 = array_34.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_36, item_35);
                object_36.finish();
            }
        }
        array_34.finish();
    }
}

pub fn serialize_structure_crate_input_create_link_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLinkInput,
) {
    if let Some(var_37) = &input.bandwidth {
        let mut object_38 = object.key("Bandwidth").start_object();
        crate::json_ser::serialize_structure_crate_model_bandwidth(&mut object_38, var_37);
        object_38.finish();
    }
    if let Some(var_39) = &input.description {
        object.key("Description").string(var_39);
    }
    if let Some(var_40) = &input.provider {
        object.key("Provider").string(var_40);
    }
    if let Some(var_41) = &input.site_id {
        object.key("SiteId").string(var_41);
    }
    if let Some(var_42) = &input.tags {
        let mut array_43 = object.key("Tags").start_array();
        for item_44 in var_42 {
            {
                let mut object_45 = array_43.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_45, item_44);
                object_45.finish();
            }
        }
        array_43.finish();
    }
    if let Some(var_46) = &input.r#type {
        object.key("Type").string(var_46);
    }
}

pub fn serialize_structure_crate_input_create_site_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSiteInput,
) {
    if let Some(var_47) = &input.description {
        object.key("Description").string(var_47);
    }
    if let Some(var_48) = &input.location {
        let mut object_49 = object.key("Location").start_object();
        crate::json_ser::serialize_structure_crate_model_location(&mut object_49, var_48);
        object_49.finish();
    }
    if let Some(var_50) = &input.tags {
        let mut array_51 = object.key("Tags").start_array();
        for item_52 in var_50 {
            {
                let mut object_53 = array_51.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_53, item_52);
                object_53.finish();
            }
        }
        array_51.finish();
    }
}

pub fn serialize_structure_crate_input_get_network_routes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetNetworkRoutesInput,
) {
    if let Some(var_54) = &input.destination_filters {
        let mut object_55 = object.key("DestinationFilters").start_object();
        for (key_56, value_57) in var_54 {
            {
                let mut array_58 = object_55.key(key_56).start_array();
                for item_59 in value_57 {
                    {
                        array_58.value().string(item_59);
                    }
                }
                array_58.finish();
            }
        }
        object_55.finish();
    }
    if let Some(var_60) = &input.exact_cidr_matches {
        let mut array_61 = object.key("ExactCidrMatches").start_array();
        for item_62 in var_60 {
            {
                array_61.value().string(item_62);
            }
        }
        array_61.finish();
    }
    if let Some(var_63) = &input.longest_prefix_matches {
        let mut array_64 = object.key("LongestPrefixMatches").start_array();
        for item_65 in var_63 {
            {
                array_64.value().string(item_65);
            }
        }
        array_64.finish();
    }
    if let Some(var_66) = &input.prefix_list_ids {
        let mut array_67 = object.key("PrefixListIds").start_array();
        for item_68 in var_66 {
            {
                array_67.value().string(item_68);
            }
        }
        array_67.finish();
    }
    if let Some(var_69) = &input.route_table_identifier {
        let mut object_70 = object.key("RouteTableIdentifier").start_object();
        crate::json_ser::serialize_structure_crate_model_route_table_identifier(
            &mut object_70,
            var_69,
        );
        object_70.finish();
    }
    if let Some(var_71) = &input.states {
        let mut array_72 = object.key("States").start_array();
        for item_73 in var_71 {
            {
                array_72.value().string(item_73.as_str());
            }
        }
        array_72.finish();
    }
    if let Some(var_74) = &input.subnet_of_matches {
        let mut array_75 = object.key("SubnetOfMatches").start_array();
        for item_76 in var_74 {
            {
                array_75.value().string(item_76);
            }
        }
        array_75.finish();
    }
    if let Some(var_77) = &input.supernet_of_matches {
        let mut array_78 = object.key("SupernetOfMatches").start_array();
        for item_79 in var_77 {
            {
                array_78.value().string(item_79);
            }
        }
        array_78.finish();
    }
    if let Some(var_80) = &input.types {
        let mut array_81 = object.key("Types").start_array();
        for item_82 in var_80 {
            {
                array_81.value().string(item_82.as_str());
            }
        }
        array_81.finish();
    }
}

pub fn serialize_structure_crate_input_register_transit_gateway_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RegisterTransitGatewayInput,
) {
    if let Some(var_83) = &input.transit_gateway_arn {
        object.key("TransitGatewayArn").string(var_83);
    }
}

pub fn serialize_structure_crate_input_start_route_analysis_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartRouteAnalysisInput,
) {
    if let Some(var_84) = &input.destination {
        let mut object_85 = object.key("Destination").start_object();
        crate::json_ser::serialize_structure_crate_model_route_analysis_endpoint_options_specification(&mut object_85, var_84);
        object_85.finish();
    }
    if input.include_return_path {
        object
            .key("IncludeReturnPath")
            .boolean(input.include_return_path);
    }
    if let Some(var_86) = &input.source {
        let mut object_87 = object.key("Source").start_object();
        crate::json_ser::serialize_structure_crate_model_route_analysis_endpoint_options_specification(&mut object_87, var_86);
        object_87.finish();
    }
    if input.use_middleboxes {
        object.key("UseMiddleboxes").boolean(input.use_middleboxes);
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_88) = &input.tags {
        let mut array_89 = object.key("Tags").start_array();
        for item_90 in var_88 {
            {
                let mut object_91 = array_89.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_91, item_90);
                object_91.finish();
            }
        }
        array_89.finish();
    }
}

pub fn serialize_structure_crate_input_update_connection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateConnectionInput,
) {
    if let Some(var_92) = &input.connected_link_id {
        object.key("ConnectedLinkId").string(var_92);
    }
    if let Some(var_93) = &input.description {
        object.key("Description").string(var_93);
    }
    if let Some(var_94) = &input.link_id {
        object.key("LinkId").string(var_94);
    }
}

pub fn serialize_structure_crate_input_update_device_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDeviceInput,
) {
    if let Some(var_95) = &input.aws_location {
        let mut object_96 = object.key("AWSLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_aws_location(&mut object_96, var_95);
        object_96.finish();
    }
    if let Some(var_97) = &input.description {
        object.key("Description").string(var_97);
    }
    if let Some(var_98) = &input.location {
        let mut object_99 = object.key("Location").start_object();
        crate::json_ser::serialize_structure_crate_model_location(&mut object_99, var_98);
        object_99.finish();
    }
    if let Some(var_100) = &input.model {
        object.key("Model").string(var_100);
    }
    if let Some(var_101) = &input.serial_number {
        object.key("SerialNumber").string(var_101);
    }
    if let Some(var_102) = &input.site_id {
        object.key("SiteId").string(var_102);
    }
    if let Some(var_103) = &input.r#type {
        object.key("Type").string(var_103);
    }
    if let Some(var_104) = &input.vendor {
        object.key("Vendor").string(var_104);
    }
}

pub fn serialize_structure_crate_input_update_global_network_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateGlobalNetworkInput,
) {
    if let Some(var_105) = &input.description {
        object.key("Description").string(var_105);
    }
}

pub fn serialize_structure_crate_input_update_link_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLinkInput,
) {
    if let Some(var_106) = &input.bandwidth {
        let mut object_107 = object.key("Bandwidth").start_object();
        crate::json_ser::serialize_structure_crate_model_bandwidth(&mut object_107, var_106);
        object_107.finish();
    }
    if let Some(var_108) = &input.description {
        object.key("Description").string(var_108);
    }
    if let Some(var_109) = &input.provider {
        object.key("Provider").string(var_109);
    }
    if let Some(var_110) = &input.r#type {
        object.key("Type").string(var_110);
    }
}

pub fn serialize_structure_crate_input_update_network_resource_metadata_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateNetworkResourceMetadataInput,
) {
    if let Some(var_111) = &input.metadata {
        let mut object_112 = object.key("Metadata").start_object();
        for (key_113, value_114) in var_111 {
            {
                object_112.key(key_113).string(value_114);
            }
        }
        object_112.finish();
    }
}

pub fn serialize_structure_crate_input_update_site_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSiteInput,
) {
    if let Some(var_115) = &input.description {
        object.key("Description").string(var_115);
    }
    if let Some(var_116) = &input.location {
        let mut object_117 = object.key("Location").start_object();
        crate::json_ser::serialize_structure_crate_model_location(&mut object_117, var_116);
        object_117.finish();
    }
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_118) = &input.key {
        object.key("Key").string(var_118);
    }
    if let Some(var_119) = &input.value {
        object.key("Value").string(var_119);
    }
}

pub fn serialize_structure_crate_model_aws_location(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AwsLocation,
) {
    if let Some(var_120) = &input.zone {
        object.key("Zone").string(var_120);
    }
    if let Some(var_121) = &input.subnet_arn {
        object.key("SubnetArn").string(var_121);
    }
}

pub fn serialize_structure_crate_model_location(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Location,
) {
    if let Some(var_122) = &input.address {
        object.key("Address").string(var_122);
    }
    if let Some(var_123) = &input.latitude {
        object.key("Latitude").string(var_123);
    }
    if let Some(var_124) = &input.longitude {
        object.key("Longitude").string(var_124);
    }
}

pub fn serialize_structure_crate_model_bandwidth(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Bandwidth,
) {
    if let Some(var_125) = &input.upload_speed {
        object.key("UploadSpeed").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_125).into()),
        );
    }
    if let Some(var_126) = &input.download_speed {
        object.key("DownloadSpeed").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_126).into()),
        );
    }
}

pub fn serialize_structure_crate_model_route_table_identifier(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RouteTableIdentifier,
) {
    if let Some(var_127) = &input.transit_gateway_route_table_arn {
        object.key("TransitGatewayRouteTableArn").string(var_127);
    }
}

pub fn serialize_structure_crate_model_route_analysis_endpoint_options_specification(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RouteAnalysisEndpointOptionsSpecification,
) {
    if let Some(var_128) = &input.transit_gateway_attachment_arn {
        object.key("TransitGatewayAttachmentArn").string(var_128);
    }
    if let Some(var_129) = &input.ip_address {
        object.key("IpAddress").string(var_129);
    }
}
