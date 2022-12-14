// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_media_capture_pipeline_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateMediaCapturePipelineInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.chime_sdk_meeting_configuration {
        let mut object_2 = object.key("ChimeSdkMeetingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_chime_sdk_meeting_configuration(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    if let Some(var_3) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.sink_arn {
        object.key("SinkArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.sink_type {
        object.key("SinkType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.source_arn {
        object.key("SourceArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.source_type {
        object.key("SourceType").string(var_7.as_str());
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("Tags").start_array();
        for item_10 in var_8 {
            {
                let mut object_11 = array_9.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_media_concatenation_pipeline_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateMediaConcatenationPipelineInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_12.as_str());
    }
    if let Some(var_13) = &input.sinks {
        let mut array_14 = object.key("Sinks").start_array();
        for item_15 in var_13 {
            {
                let mut object_16 = array_14.value().start_object();
                crate::json_ser::serialize_structure_crate_model_concatenation_sink(
                    &mut object_16,
                    item_15,
                )?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.sources {
        let mut array_18 = object.key("Sources").start_array();
        for item_19 in var_17 {
            {
                let mut object_20 = array_18.value().start_object();
                crate::json_ser::serialize_structure_crate_model_concatenation_source(
                    &mut object_20,
                    item_19,
                )?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.tags {
        let mut array_22 = object.key("Tags").start_array();
        for item_23 in var_21 {
            {
                let mut object_24 = array_22.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_24, item_23)?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_media_live_connector_pipeline_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateMediaLiveConnectorPipelineInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_25) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_25.as_str());
    }
    if let Some(var_26) = &input.sinks {
        let mut array_27 = object.key("Sinks").start_array();
        for item_28 in var_26 {
            {
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_crate_model_live_connector_sink_configuration(
                    &mut object_29,
                    item_28,
                )?;
                object_29.finish();
            }
        }
        array_27.finish();
    }
    if let Some(var_30) = &input.sources {
        let mut array_31 = object.key("Sources").start_array();
        for item_32 in var_30 {
            {
                let mut object_33 = array_31.value().start_object();
                crate::json_ser::serialize_structure_crate_model_live_connector_source_configuration(&mut object_33, item_32)?;
                object_33.finish();
            }
        }
        array_31.finish();
    }
    if let Some(var_34) = &input.tags {
        let mut array_35 = object.key("Tags").start_array();
        for item_36 in var_34 {
            {
                let mut object_37 = array_35.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_37, item_36)?;
                object_37.finish();
            }
        }
        array_35.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_38) = &input.resource_arn {
        object.key("ResourceARN").string(var_38.as_str());
    }
    if let Some(var_39) = &input.tags {
        let mut array_40 = object.key("Tags").start_array();
        for item_41 in var_39 {
            {
                let mut object_42 = array_40.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_42, item_41)?;
                object_42.finish();
            }
        }
        array_40.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_43) = &input.resource_arn {
        object.key("ResourceARN").string(var_43.as_str());
    }
    if let Some(var_44) = &input.tag_keys {
        let mut array_45 = object.key("TagKeys").start_array();
        for item_46 in var_44 {
            {
                array_45.value().string(item_46.as_str());
            }
        }
        array_45.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_chime_sdk_meeting_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ChimeSdkMeetingConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_47) = &input.source_configuration {
        let mut object_48 = object.key("SourceConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_source_configuration(
            &mut object_48,
            var_47,
        )?;
        object_48.finish();
    }
    if let Some(var_49) = &input.artifacts_configuration {
        let mut object_50 = object.key("ArtifactsConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_artifacts_configuration(
            &mut object_50,
            var_49,
        )?;
        object_50.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_51) = &input.key {
        object.key("Key").string(var_51.as_str());
    }
    if let Some(var_52) = &input.value {
        object.key("Value").string(var_52.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_concatenation_sink(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ConcatenationSink,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_53) = &input.r#type {
        object.key("Type").string(var_53.as_str());
    }
    if let Some(var_54) = &input.s3_bucket_sink_configuration {
        let mut object_55 = object.key("S3BucketSinkConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_bucket_sink_configuration(
            &mut object_55,
            var_54,
        )?;
        object_55.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_concatenation_source(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ConcatenationSource,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_56) = &input.r#type {
        object.key("Type").string(var_56.as_str());
    }
    if let Some(var_57) = &input.media_capture_pipeline_source_configuration {
        let mut object_58 = object
            .key("MediaCapturePipelineSourceConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_media_capture_pipeline_source_configuration(&mut object_58, var_57)?;
        object_58.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_live_connector_sink_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LiveConnectorSinkConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_59) = &input.sink_type {
        object.key("SinkType").string(var_59.as_str());
    }
    if let Some(var_60) = &input.rtmp_configuration {
        let mut object_61 = object.key("RTMPConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_live_connector_rtmp_configuration(
            &mut object_61,
            var_60,
        )?;
        object_61.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_live_connector_source_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LiveConnectorSourceConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_62) = &input.source_type {
        object.key("SourceType").string(var_62.as_str());
    }
    if let Some(var_63) = &input.chime_sdk_meeting_live_connector_configuration {
        let mut object_64 = object
            .key("ChimeSdkMeetingLiveConnectorConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_chime_sdk_meeting_live_connector_configuration(&mut object_64, var_63)?;
        object_64.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_source_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SourceConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_65) = &input.selected_video_streams {
        let mut object_66 = object.key("SelectedVideoStreams").start_object();
        crate::json_ser::serialize_structure_crate_model_selected_video_streams(
            &mut object_66,
            var_65,
        )?;
        object_66.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_artifacts_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ArtifactsConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_67) = &input.audio {
        let mut object_68 = object.key("Audio").start_object();
        crate::json_ser::serialize_structure_crate_model_audio_artifacts_configuration(
            &mut object_68,
            var_67,
        )?;
        object_68.finish();
    }
    if let Some(var_69) = &input.video {
        let mut object_70 = object.key("Video").start_object();
        crate::json_ser::serialize_structure_crate_model_video_artifacts_configuration(
            &mut object_70,
            var_69,
        )?;
        object_70.finish();
    }
    if let Some(var_71) = &input.content {
        let mut object_72 = object.key("Content").start_object();
        crate::json_ser::serialize_structure_crate_model_content_artifacts_configuration(
            &mut object_72,
            var_71,
        )?;
        object_72.finish();
    }
    if let Some(var_73) = &input.composited_video {
        let mut object_74 = object.key("CompositedVideo").start_object();
        crate::json_ser::serialize_structure_crate_model_composited_video_artifacts_configuration(
            &mut object_74,
            var_73,
        )?;
        object_74.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_bucket_sink_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3BucketSinkConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_75) = &input.destination {
        object.key("Destination").string(var_75.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_media_capture_pipeline_source_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MediaCapturePipelineSourceConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_76) = &input.media_pipeline_arn {
        object.key("MediaPipelineArn").string(var_76.as_str());
    }
    if let Some(var_77) = &input.chime_sdk_meeting_configuration {
        let mut object_78 = object.key("ChimeSdkMeetingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_chime_sdk_meeting_concatenation_configuration(&mut object_78, var_77)?;
        object_78.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_live_connector_rtmp_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LiveConnectorRtmpConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_79) = &input.url {
        object.key("Url").string(var_79.as_str());
    }
    if let Some(var_80) = &input.audio_channels {
        object.key("AudioChannels").string(var_80.as_str());
    }
    if let Some(var_81) = &input.audio_sample_rate {
        object.key("AudioSampleRate").string(var_81.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_chime_sdk_meeting_live_connector_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ChimeSdkMeetingLiveConnectorConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_82) = &input.arn {
        object.key("Arn").string(var_82.as_str());
    }
    if let Some(var_83) = &input.mux_type {
        object.key("MuxType").string(var_83.as_str());
    }
    if let Some(var_84) = &input.composited_video {
        let mut object_85 = object.key("CompositedVideo").start_object();
        crate::json_ser::serialize_structure_crate_model_composited_video_artifacts_configuration(
            &mut object_85,
            var_84,
        )?;
        object_85.finish();
    }
    if let Some(var_86) = &input.source_configuration {
        let mut object_87 = object.key("SourceConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_source_configuration(
            &mut object_87,
            var_86,
        )?;
        object_87.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_selected_video_streams(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SelectedVideoStreams,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_88) = &input.attendee_ids {
        let mut array_89 = object.key("AttendeeIds").start_array();
        for item_90 in var_88 {
            {
                array_89.value().string(item_90.as_str());
            }
        }
        array_89.finish();
    }
    if let Some(var_91) = &input.external_user_ids {
        let mut array_92 = object.key("ExternalUserIds").start_array();
        for item_93 in var_91 {
            {
                array_92.value().string(item_93.as_str());
            }
        }
        array_92.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_audio_artifacts_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AudioArtifactsConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_94) = &input.mux_type {
        object.key("MuxType").string(var_94.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_video_artifacts_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VideoArtifactsConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_95) = &input.state {
        object.key("State").string(var_95.as_str());
    }
    if let Some(var_96) = &input.mux_type {
        object.key("MuxType").string(var_96.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_content_artifacts_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ContentArtifactsConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_97) = &input.state {
        object.key("State").string(var_97.as_str());
    }
    if let Some(var_98) = &input.mux_type {
        object.key("MuxType").string(var_98.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_composited_video_artifacts_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CompositedVideoArtifactsConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_99) = &input.layout {
        object.key("Layout").string(var_99.as_str());
    }
    if let Some(var_100) = &input.resolution {
        object.key("Resolution").string(var_100.as_str());
    }
    if let Some(var_101) = &input.grid_view_configuration {
        let mut object_102 = object.key("GridViewConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_grid_view_configuration(
            &mut object_102,
            var_101,
        )?;
        object_102.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_chime_sdk_meeting_concatenation_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ChimeSdkMeetingConcatenationConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_103) = &input.artifacts_configuration {
        let mut object_104 = object.key("ArtifactsConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_artifacts_concatenation_configuration(
            &mut object_104,
            var_103,
        )?;
        object_104.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_grid_view_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::GridViewConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_105) = &input.content_share_layout {
        object.key("ContentShareLayout").string(var_105.as_str());
    }
    if let Some(var_106) = &input.presenter_only_configuration {
        let mut object_107 = object.key("PresenterOnlyConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_presenter_only_configuration(
            &mut object_107,
            var_106,
        )?;
        object_107.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_artifacts_concatenation_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ArtifactsConcatenationConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_108) = &input.audio {
        let mut object_109 = object.key("Audio").start_object();
        crate::json_ser::serialize_structure_crate_model_audio_concatenation_configuration(
            &mut object_109,
            var_108,
        )?;
        object_109.finish();
    }
    if let Some(var_110) = &input.video {
        let mut object_111 = object.key("Video").start_object();
        crate::json_ser::serialize_structure_crate_model_video_concatenation_configuration(
            &mut object_111,
            var_110,
        )?;
        object_111.finish();
    }
    if let Some(var_112) = &input.content {
        let mut object_113 = object.key("Content").start_object();
        crate::json_ser::serialize_structure_crate_model_content_concatenation_configuration(
            &mut object_113,
            var_112,
        )?;
        object_113.finish();
    }
    if let Some(var_114) = &input.data_channel {
        let mut object_115 = object.key("DataChannel").start_object();
        crate::json_ser::serialize_structure_crate_model_data_channel_concatenation_configuration(
            &mut object_115,
            var_114,
        )?;
        object_115.finish();
    }
    if let Some(var_116) = &input.transcription_messages {
        let mut object_117 = object.key("TranscriptionMessages").start_object();
        crate::json_ser::serialize_structure_crate_model_transcription_messages_concatenation_configuration(&mut object_117, var_116)?;
        object_117.finish();
    }
    if let Some(var_118) = &input.meeting_events {
        let mut object_119 = object.key("MeetingEvents").start_object();
        crate::json_ser::serialize_structure_crate_model_meeting_events_concatenation_configuration(&mut object_119, var_118)?;
        object_119.finish();
    }
    if let Some(var_120) = &input.composited_video {
        let mut object_121 = object.key("CompositedVideo").start_object();
        crate::json_ser::serialize_structure_crate_model_composited_video_concatenation_configuration(&mut object_121, var_120)?;
        object_121.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_presenter_only_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PresenterOnlyConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_122) = &input.presenter_position {
        object.key("PresenterPosition").string(var_122.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_audio_concatenation_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AudioConcatenationConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_123) = &input.state {
        object.key("State").string(var_123.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_video_concatenation_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VideoConcatenationConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_124) = &input.state {
        object.key("State").string(var_124.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_content_concatenation_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ContentConcatenationConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_125) = &input.state {
        object.key("State").string(var_125.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_data_channel_concatenation_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DataChannelConcatenationConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_126) = &input.state {
        object.key("State").string(var_126.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_transcription_messages_concatenation_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TranscriptionMessagesConcatenationConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_127) = &input.state {
        object.key("State").string(var_127.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_meeting_events_concatenation_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MeetingEventsConcatenationConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_128) = &input.state {
        object.key("State").string(var_128.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_composited_video_concatenation_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CompositedVideoConcatenationConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_129) = &input.state {
        object.key("State").string(var_129.as_str());
    }
    Ok(())
}
