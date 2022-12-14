// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_tracker_consumer_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateTrackerConsumerInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.consumer_arn {
        object.key("ConsumerArn").string(var_1.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_delete_device_position_history_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchDeleteDevicePositionHistoryInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_2) = &input.device_ids {
        let mut array_3 = object.key("DeviceIds").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_delete_geofence_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchDeleteGeofenceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_5) = &input.geofence_ids {
        let mut array_6 = object.key("GeofenceIds").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_evaluate_geofences_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchEvaluateGeofencesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_8) = &input.device_position_updates {
        let mut array_9 = object.key("DevicePositionUpdates").start_array();
        for item_10 in var_8 {
            {
                let mut object_11 = array_9.value().start_object();
                crate::json_ser::serialize_structure_crate_model_device_position_update(
                    &mut object_11,
                    item_10,
                )?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_get_device_position_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetDevicePositionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.device_ids {
        let mut array_13 = object.key("DeviceIds").start_array();
        for item_14 in var_12 {
            {
                array_13.value().string(item_14.as_str());
            }
        }
        array_13.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_put_geofence_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchPutGeofenceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_15) = &input.entries {
        let mut array_16 = object.key("Entries").start_array();
        for item_17 in var_15 {
            {
                let mut object_18 = array_16.value().start_object();
                crate::json_ser::serialize_structure_crate_model_batch_put_geofence_request_entry(
                    &mut object_18,
                    item_17,
                )?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_update_device_position_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchUpdateDevicePositionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_19) = &input.updates {
        let mut array_20 = object.key("Updates").start_array();
        for item_21 in var_19 {
            {
                let mut object_22 = array_20.value().start_object();
                crate::json_ser::serialize_structure_crate_model_device_position_update(
                    &mut object_22,
                    item_21,
                )?;
                object_22.finish();
            }
        }
        array_20.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_calculate_route_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CalculateRouteInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_23) = &input.car_mode_options {
        let mut object_24 = object.key("CarModeOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_calculate_route_car_mode_options(
            &mut object_24,
            var_23,
        )?;
        object_24.finish();
    }
    if let Some(var_25) = &input.depart_now {
        object.key("DepartNow").boolean(*var_25);
    }
    if let Some(var_26) = &input.departure_position {
        let mut array_27 = object.key("DeparturePosition").start_array();
        for item_28 in var_26 {
            {
                array_27.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::Float((*item_28).into()),
                );
            }
        }
        array_27.finish();
    }
    if let Some(var_29) = &input.departure_time {
        object
            .key("DepartureTime")
            .date_time(var_29, aws_smithy_types::date_time::Format::DateTime)?;
    }
    if let Some(var_30) = &input.destination_position {
        let mut array_31 = object.key("DestinationPosition").start_array();
        for item_32 in var_30 {
            {
                array_31.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::Float((*item_32).into()),
                );
            }
        }
        array_31.finish();
    }
    if let Some(var_33) = &input.distance_unit {
        object.key("DistanceUnit").string(var_33.as_str());
    }
    if let Some(var_34) = &input.include_leg_geometry {
        object.key("IncludeLegGeometry").boolean(*var_34);
    }
    if let Some(var_35) = &input.travel_mode {
        object.key("TravelMode").string(var_35.as_str());
    }
    if let Some(var_36) = &input.truck_mode_options {
        let mut object_37 = object.key("TruckModeOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_calculate_route_truck_mode_options(
            &mut object_37,
            var_36,
        )?;
        object_37.finish();
    }
    if let Some(var_38) = &input.waypoint_positions {
        let mut array_39 = object.key("WaypointPositions").start_array();
        for item_40 in var_38 {
            {
                let mut array_41 = array_39.value().start_array();
                for item_42 in item_40 {
                    {
                        array_41.value().number(
                            #[allow(clippy::useless_conversion)]
                            aws_smithy_types::Number::Float((*item_42).into()),
                        );
                    }
                }
                array_41.finish();
            }
        }
        array_39.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_calculate_route_matrix_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CalculateRouteMatrixInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_43) = &input.car_mode_options {
        let mut object_44 = object.key("CarModeOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_calculate_route_car_mode_options(
            &mut object_44,
            var_43,
        )?;
        object_44.finish();
    }
    if let Some(var_45) = &input.depart_now {
        object.key("DepartNow").boolean(*var_45);
    }
    if let Some(var_46) = &input.departure_positions {
        let mut array_47 = object.key("DeparturePositions").start_array();
        for item_48 in var_46 {
            {
                let mut array_49 = array_47.value().start_array();
                for item_50 in item_48 {
                    {
                        array_49.value().number(
                            #[allow(clippy::useless_conversion)]
                            aws_smithy_types::Number::Float((*item_50).into()),
                        );
                    }
                }
                array_49.finish();
            }
        }
        array_47.finish();
    }
    if let Some(var_51) = &input.departure_time {
        object
            .key("DepartureTime")
            .date_time(var_51, aws_smithy_types::date_time::Format::DateTime)?;
    }
    if let Some(var_52) = &input.destination_positions {
        let mut array_53 = object.key("DestinationPositions").start_array();
        for item_54 in var_52 {
            {
                let mut array_55 = array_53.value().start_array();
                for item_56 in item_54 {
                    {
                        array_55.value().number(
                            #[allow(clippy::useless_conversion)]
                            aws_smithy_types::Number::Float((*item_56).into()),
                        );
                    }
                }
                array_55.finish();
            }
        }
        array_53.finish();
    }
    if let Some(var_57) = &input.distance_unit {
        object.key("DistanceUnit").string(var_57.as_str());
    }
    if let Some(var_58) = &input.travel_mode {
        object.key("TravelMode").string(var_58.as_str());
    }
    if let Some(var_59) = &input.truck_mode_options {
        let mut object_60 = object.key("TruckModeOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_calculate_route_truck_mode_options(
            &mut object_60,
            var_59,
        )?;
        object_60.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_geofence_collection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateGeofenceCollectionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_61) = &input.collection_name {
        object.key("CollectionName").string(var_61.as_str());
    }
    if let Some(var_62) = &input.description {
        object.key("Description").string(var_62.as_str());
    }
    if let Some(var_63) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_63.as_str());
    }
    if let Some(var_64) = &input.pricing_plan {
        object.key("PricingPlan").string(var_64.as_str());
    }
    if let Some(var_65) = &input.pricing_plan_data_source {
        object.key("PricingPlanDataSource").string(var_65.as_str());
    }
    if let Some(var_66) = &input.tags {
        let mut object_67 = object.key("Tags").start_object();
        for (key_68, value_69) in var_66 {
            {
                object_67.key(key_68.as_str()).string(value_69.as_str());
            }
        }
        object_67.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_map_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateMapInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.configuration {
        let mut object_71 = object.key("Configuration").start_object();
        crate::json_ser::serialize_structure_crate_model_map_configuration(&mut object_71, var_70)?;
        object_71.finish();
    }
    if let Some(var_72) = &input.description {
        object.key("Description").string(var_72.as_str());
    }
    if let Some(var_73) = &input.map_name {
        object.key("MapName").string(var_73.as_str());
    }
    if let Some(var_74) = &input.pricing_plan {
        object.key("PricingPlan").string(var_74.as_str());
    }
    if let Some(var_75) = &input.tags {
        let mut object_76 = object.key("Tags").start_object();
        for (key_77, value_78) in var_75 {
            {
                object_76.key(key_77.as_str()).string(value_78.as_str());
            }
        }
        object_76.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_place_index_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePlaceIndexInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_79) = &input.data_source {
        object.key("DataSource").string(var_79.as_str());
    }
    if let Some(var_80) = &input.data_source_configuration {
        let mut object_81 = object.key("DataSourceConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_data_source_configuration(
            &mut object_81,
            var_80,
        )?;
        object_81.finish();
    }
    if let Some(var_82) = &input.description {
        object.key("Description").string(var_82.as_str());
    }
    if let Some(var_83) = &input.index_name {
        object.key("IndexName").string(var_83.as_str());
    }
    if let Some(var_84) = &input.pricing_plan {
        object.key("PricingPlan").string(var_84.as_str());
    }
    if let Some(var_85) = &input.tags {
        let mut object_86 = object.key("Tags").start_object();
        for (key_87, value_88) in var_85 {
            {
                object_86.key(key_87.as_str()).string(value_88.as_str());
            }
        }
        object_86.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_route_calculator_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRouteCalculatorInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_89) = &input.calculator_name {
        object.key("CalculatorName").string(var_89.as_str());
    }
    if let Some(var_90) = &input.data_source {
        object.key("DataSource").string(var_90.as_str());
    }
    if let Some(var_91) = &input.description {
        object.key("Description").string(var_91.as_str());
    }
    if let Some(var_92) = &input.pricing_plan {
        object.key("PricingPlan").string(var_92.as_str());
    }
    if let Some(var_93) = &input.tags {
        let mut object_94 = object.key("Tags").start_object();
        for (key_95, value_96) in var_93 {
            {
                object_94.key(key_95.as_str()).string(value_96.as_str());
            }
        }
        object_94.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_tracker_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTrackerInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_97) = &input.description {
        object.key("Description").string(var_97.as_str());
    }
    if let Some(var_98) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_98.as_str());
    }
    if let Some(var_99) = &input.position_filtering {
        object.key("PositionFiltering").string(var_99.as_str());
    }
    if let Some(var_100) = &input.pricing_plan {
        object.key("PricingPlan").string(var_100.as_str());
    }
    if let Some(var_101) = &input.pricing_plan_data_source {
        object.key("PricingPlanDataSource").string(var_101.as_str());
    }
    if let Some(var_102) = &input.tags {
        let mut object_103 = object.key("Tags").start_object();
        for (key_104, value_105) in var_102 {
            {
                object_103.key(key_104.as_str()).string(value_105.as_str());
            }
        }
        object_103.finish();
    }
    if let Some(var_106) = &input.tracker_name {
        object.key("TrackerName").string(var_106.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_device_position_history_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDevicePositionHistoryInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_107) = &input.end_time_exclusive {
        object
            .key("EndTimeExclusive")
            .date_time(var_107, aws_smithy_types::date_time::Format::DateTime)?;
    }
    if let Some(var_108) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_108).into()),
        );
    }
    if let Some(var_109) = &input.next_token {
        object.key("NextToken").string(var_109.as_str());
    }
    if let Some(var_110) = &input.start_time_inclusive {
        object
            .key("StartTimeInclusive")
            .date_time(var_110, aws_smithy_types::date_time::Format::DateTime)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_device_positions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDevicePositionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_111) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_111).into()),
        );
    }
    if let Some(var_112) = &input.next_token {
        object.key("NextToken").string(var_112.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_geofence_collections_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListGeofenceCollectionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_113) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_113).into()),
        );
    }
    if let Some(var_114) = &input.next_token {
        object.key("NextToken").string(var_114.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_geofences_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListGeofencesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_115) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_115).into()),
        );
    }
    if let Some(var_116) = &input.next_token {
        object.key("NextToken").string(var_116.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_maps_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListMapsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_117) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_117).into()),
        );
    }
    if let Some(var_118) = &input.next_token {
        object.key("NextToken").string(var_118.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_place_indexes_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPlaceIndexesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_119) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_119).into()),
        );
    }
    if let Some(var_120) = &input.next_token {
        object.key("NextToken").string(var_120.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_route_calculators_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListRouteCalculatorsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_121) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_121).into()),
        );
    }
    if let Some(var_122) = &input.next_token {
        object.key("NextToken").string(var_122.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tracker_consumers_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTrackerConsumersInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_123) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_123).into()),
        );
    }
    if let Some(var_124) = &input.next_token {
        object.key("NextToken").string(var_124.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_trackers_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTrackersInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_125) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_125).into()),
        );
    }
    if let Some(var_126) = &input.next_token {
        object.key("NextToken").string(var_126.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_geofence_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutGeofenceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_127) = &input.geometry {
        let mut object_128 = object.key("Geometry").start_object();
        crate::json_ser::serialize_structure_crate_model_geofence_geometry(
            &mut object_128,
            var_127,
        )?;
        object_128.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_search_place_index_for_position_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SearchPlaceIndexForPositionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_129) = &input.language {
        object.key("Language").string(var_129.as_str());
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_130) = &input.position {
        let mut array_131 = object.key("Position").start_array();
        for item_132 in var_130 {
            {
                array_131.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::Float((*item_132).into()),
                );
            }
        }
        array_131.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_search_place_index_for_suggestions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SearchPlaceIndexForSuggestionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_133) = &input.bias_position {
        let mut array_134 = object.key("BiasPosition").start_array();
        for item_135 in var_133 {
            {
                array_134.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::Float((*item_135).into()),
                );
            }
        }
        array_134.finish();
    }
    if let Some(var_136) = &input.filter_b_box {
        let mut array_137 = object.key("FilterBBox").start_array();
        for item_138 in var_136 {
            {
                array_137.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::Float((*item_138).into()),
                );
            }
        }
        array_137.finish();
    }
    if let Some(var_139) = &input.filter_countries {
        let mut array_140 = object.key("FilterCountries").start_array();
        for item_141 in var_139 {
            {
                array_140.value().string(item_141.as_str());
            }
        }
        array_140.finish();
    }
    if let Some(var_142) = &input.language {
        object.key("Language").string(var_142.as_str());
    }
    if let Some(var_143) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_143).into()),
        );
    }
    if let Some(var_144) = &input.text {
        object.key("Text").string(var_144.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_search_place_index_for_text_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SearchPlaceIndexForTextInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_145) = &input.bias_position {
        let mut array_146 = object.key("BiasPosition").start_array();
        for item_147 in var_145 {
            {
                array_146.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::Float((*item_147).into()),
                );
            }
        }
        array_146.finish();
    }
    if let Some(var_148) = &input.filter_b_box {
        let mut array_149 = object.key("FilterBBox").start_array();
        for item_150 in var_148 {
            {
                array_149.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::Float((*item_150).into()),
                );
            }
        }
        array_149.finish();
    }
    if let Some(var_151) = &input.filter_countries {
        let mut array_152 = object.key("FilterCountries").start_array();
        for item_153 in var_151 {
            {
                array_152.value().string(item_153.as_str());
            }
        }
        array_152.finish();
    }
    if let Some(var_154) = &input.language {
        object.key("Language").string(var_154.as_str());
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_155) = &input.text {
        object.key("Text").string(var_155.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_156) = &input.tags {
        let mut object_157 = object.key("Tags").start_object();
        for (key_158, value_159) in var_156 {
            {
                object_157.key(key_158.as_str()).string(value_159.as_str());
            }
        }
        object_157.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_geofence_collection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateGeofenceCollectionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_160) = &input.description {
        object.key("Description").string(var_160.as_str());
    }
    if let Some(var_161) = &input.pricing_plan {
        object.key("PricingPlan").string(var_161.as_str());
    }
    if let Some(var_162) = &input.pricing_plan_data_source {
        object.key("PricingPlanDataSource").string(var_162.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_map_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateMapInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_163) = &input.description {
        object.key("Description").string(var_163.as_str());
    }
    if let Some(var_164) = &input.pricing_plan {
        object.key("PricingPlan").string(var_164.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_place_index_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePlaceIndexInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_165) = &input.data_source_configuration {
        let mut object_166 = object.key("DataSourceConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_data_source_configuration(
            &mut object_166,
            var_165,
        )?;
        object_166.finish();
    }
    if let Some(var_167) = &input.description {
        object.key("Description").string(var_167.as_str());
    }
    if let Some(var_168) = &input.pricing_plan {
        object.key("PricingPlan").string(var_168.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_route_calculator_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRouteCalculatorInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_169) = &input.description {
        object.key("Description").string(var_169.as_str());
    }
    if let Some(var_170) = &input.pricing_plan {
        object.key("PricingPlan").string(var_170.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_tracker_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTrackerInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_171) = &input.description {
        object.key("Description").string(var_171.as_str());
    }
    if let Some(var_172) = &input.position_filtering {
        object.key("PositionFiltering").string(var_172.as_str());
    }
    if let Some(var_173) = &input.pricing_plan {
        object.key("PricingPlan").string(var_173.as_str());
    }
    if let Some(var_174) = &input.pricing_plan_data_source {
        object.key("PricingPlanDataSource").string(var_174.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_device_position_update(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DevicePositionUpdate,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_175) = &input.device_id {
        object.key("DeviceId").string(var_175.as_str());
    }
    if let Some(var_176) = &input.sample_time {
        object
            .key("SampleTime")
            .date_time(var_176, aws_smithy_types::date_time::Format::DateTime)?;
    }
    if let Some(var_177) = &input.position {
        let mut array_178 = object.key("Position").start_array();
        for item_179 in var_177 {
            {
                array_178.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::Float((*item_179).into()),
                );
            }
        }
        array_178.finish();
    }
    if let Some(var_180) = &input.accuracy {
        let mut object_181 = object.key("Accuracy").start_object();
        crate::json_ser::serialize_structure_crate_model_positional_accuracy(
            &mut object_181,
            var_180,
        )?;
        object_181.finish();
    }
    if let Some(var_182) = &input.position_properties {
        let mut object_183 = object.key("PositionProperties").start_object();
        for (key_184, value_185) in var_182 {
            {
                object_183.key(key_184.as_str()).string(value_185.as_str());
            }
        }
        object_183.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_batch_put_geofence_request_entry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BatchPutGeofenceRequestEntry,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_186) = &input.geofence_id {
        object.key("GeofenceId").string(var_186.as_str());
    }
    if let Some(var_187) = &input.geometry {
        let mut object_188 = object.key("Geometry").start_object();
        crate::json_ser::serialize_structure_crate_model_geofence_geometry(
            &mut object_188,
            var_187,
        )?;
        object_188.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_calculate_route_car_mode_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CalculateRouteCarModeOptions,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_189) = &input.avoid_ferries {
        object.key("AvoidFerries").boolean(*var_189);
    }
    if let Some(var_190) = &input.avoid_tolls {
        object.key("AvoidTolls").boolean(*var_190);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_calculate_route_truck_mode_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CalculateRouteTruckModeOptions,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_191) = &input.avoid_ferries {
        object.key("AvoidFerries").boolean(*var_191);
    }
    if let Some(var_192) = &input.avoid_tolls {
        object.key("AvoidTolls").boolean(*var_192);
    }
    if let Some(var_193) = &input.dimensions {
        let mut object_194 = object.key("Dimensions").start_object();
        crate::json_ser::serialize_structure_crate_model_truck_dimensions(
            &mut object_194,
            var_193,
        )?;
        object_194.finish();
    }
    if let Some(var_195) = &input.weight {
        let mut object_196 = object.key("Weight").start_object();
        crate::json_ser::serialize_structure_crate_model_truck_weight(&mut object_196, var_195)?;
        object_196.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_map_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MapConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_197) = &input.style {
        object.key("Style").string(var_197.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_data_source_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DataSourceConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_198) = &input.intended_use {
        object.key("IntendedUse").string(var_198.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_geofence_geometry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::GeofenceGeometry,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_199) = &input.polygon {
        let mut array_200 = object.key("Polygon").start_array();
        for item_201 in var_199 {
            {
                let mut array_202 = array_200.value().start_array();
                for item_203 in item_201 {
                    {
                        let mut array_204 = array_202.value().start_array();
                        for item_205 in item_203 {
                            {
                                array_204.value().number(
                                    #[allow(clippy::useless_conversion)]
                                    aws_smithy_types::Number::Float((*item_205).into()),
                                );
                            }
                        }
                        array_204.finish();
                    }
                }
                array_202.finish();
            }
        }
        array_200.finish();
    }
    if let Some(var_206) = &input.circle {
        let mut object_207 = object.key("Circle").start_object();
        crate::json_ser::serialize_structure_crate_model_circle(&mut object_207, var_206)?;
        object_207.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_positional_accuracy(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PositionalAccuracy,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_208) = &input.horizontal {
        object.key("Horizontal").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_208).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_truck_dimensions(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TruckDimensions,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_209) = &input.length {
        object.key("Length").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_209).into()),
        );
    }
    if let Some(var_210) = &input.height {
        object.key("Height").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_210).into()),
        );
    }
    if let Some(var_211) = &input.width {
        object.key("Width").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_211).into()),
        );
    }
    if let Some(var_212) = &input.unit {
        object.key("Unit").string(var_212.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_truck_weight(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TruckWeight,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_213) = &input.total {
        object.key("Total").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_213).into()),
        );
    }
    if let Some(var_214) = &input.unit {
        object.key("Unit").string(var_214.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_circle(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Circle,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_215) = &input.center {
        let mut array_216 = object.key("Center").start_array();
        for item_217 in var_215 {
            {
                array_216.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::Float((*item_217).into()),
                );
            }
        }
        array_216.finish();
    }
    if let Some(var_218) = &input.radius {
        object.key("Radius").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_218).into()),
        );
    }
    Ok(())
}
