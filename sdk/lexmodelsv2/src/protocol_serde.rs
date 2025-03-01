// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_batch_create_custom_vocabulary_item;

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

pub(crate) mod shape_batch_delete_custom_vocabulary_item;

pub(crate) mod shape_batch_update_custom_vocabulary_item;

pub(crate) mod shape_build_bot_locale;

pub(crate) mod shape_create_bot;

pub(crate) mod shape_create_bot_alias;

pub(crate) mod shape_create_bot_locale;

pub(crate) mod shape_create_bot_version;

pub(crate) mod shape_create_export;

pub(crate) mod shape_create_intent;

pub(crate) mod shape_create_resource_policy;

pub(crate) mod shape_create_resource_policy_statement;

pub(crate) mod shape_create_slot;

pub(crate) mod shape_create_slot_type;

pub(crate) mod shape_create_upload_url;

pub(crate) mod shape_delete_bot;

pub(crate) mod shape_delete_bot_alias;

pub(crate) mod shape_delete_bot_locale;

pub(crate) mod shape_delete_bot_version;

pub(crate) mod shape_delete_custom_vocabulary;

pub(crate) mod shape_delete_export;

pub(crate) mod shape_delete_import;

pub(crate) mod shape_delete_intent;

pub(crate) mod shape_delete_resource_policy;

pub(crate) mod shape_delete_resource_policy_statement;

pub(crate) mod shape_delete_slot;

pub(crate) mod shape_delete_slot_type;

pub(crate) mod shape_delete_utterances;

pub(crate) mod shape_describe_bot;

pub(crate) mod shape_describe_bot_alias;

pub(crate) mod shape_describe_bot_locale;

pub(crate) mod shape_describe_bot_recommendation;

pub(crate) mod shape_describe_bot_version;

pub(crate) mod shape_describe_custom_vocabulary_metadata;

pub(crate) mod shape_describe_export;

pub(crate) mod shape_describe_import;

pub(crate) mod shape_describe_intent;

pub(crate) mod shape_describe_resource_policy;

pub(crate) mod shape_describe_slot;

pub(crate) mod shape_describe_slot_type;

pub(crate) mod shape_list_aggregated_utterances;

pub(crate) mod shape_list_bot_aliases;

pub(crate) mod shape_list_bot_locales;

pub(crate) mod shape_list_bot_recommendations;

pub(crate) mod shape_list_bot_versions;

pub(crate) mod shape_list_bots;

pub(crate) mod shape_list_built_in_intents;

pub(crate) mod shape_list_built_in_slot_types;

pub(crate) mod shape_list_custom_vocabulary_items;

pub(crate) mod shape_list_exports;

pub(crate) mod shape_list_imports;

pub(crate) mod shape_list_intents;

pub(crate) mod shape_list_recommended_intents;

pub(crate) mod shape_list_slot_types;

pub(crate) mod shape_list_slots;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_search_associated_transcripts;

pub(crate) mod shape_start_bot_recommendation;

pub(crate) mod shape_start_import;

pub(crate) mod shape_stop_bot_recommendation;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_bot;

pub(crate) mod shape_update_bot_alias;

pub(crate) mod shape_update_bot_locale;

pub(crate) mod shape_update_bot_recommendation;

pub(crate) mod shape_update_export;

pub(crate) mod shape_update_intent;

pub(crate) mod shape_update_resource_policy;

pub(crate) mod shape_update_slot;

pub(crate) mod shape_update_slot_type;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_batch_create_custom_vocabulary_item_input;

pub(crate) mod shape_batch_delete_custom_vocabulary_item_input;

pub(crate) mod shape_batch_update_custom_vocabulary_item_input;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_bot_alias_input;

pub(crate) mod shape_create_bot_input;

pub(crate) mod shape_create_bot_locale_input;

pub(crate) mod shape_create_bot_version_input;

pub(crate) mod shape_create_export_input;

pub(crate) mod shape_create_intent_input;

pub(crate) mod shape_create_resource_policy_input;

pub(crate) mod shape_create_resource_policy_statement_input;

pub(crate) mod shape_create_slot_input;

pub(crate) mod shape_create_slot_type_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_list_aggregated_utterances_input;

pub(crate) mod shape_list_bot_aliases_input;

pub(crate) mod shape_list_bot_locales_input;

pub(crate) mod shape_list_bot_recommendations_input;

pub(crate) mod shape_list_bot_versions_input;

pub(crate) mod shape_list_bots_input;

pub(crate) mod shape_list_built_in_intents_input;

pub(crate) mod shape_list_built_in_slot_types_input;

pub(crate) mod shape_list_custom_vocabulary_items_input;

pub(crate) mod shape_list_exports_input;

pub(crate) mod shape_list_imports_input;

pub(crate) mod shape_list_intents_input;

pub(crate) mod shape_list_recommended_intents_input;

pub(crate) mod shape_list_slot_types_input;

pub(crate) mod shape_list_slots_input;

pub(crate) mod shape_precondition_failed_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_search_associated_transcripts_input;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_start_bot_recommendation_input;

pub(crate) mod shape_start_import_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_bot_alias_input;

pub(crate) mod shape_update_bot_input;

pub(crate) mod shape_update_bot_locale_input;

pub(crate) mod shape_update_bot_recommendation_input;

pub(crate) mod shape_update_export_input;

pub(crate) mod shape_update_intent_input;

pub(crate) mod shape_update_resource_policy_input;

pub(crate) mod shape_update_slot_input;

pub(crate) mod shape_update_slot_type_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_aggregated_utterances_filter;

pub(crate) mod shape_aggregated_utterances_sort_by;

pub(crate) mod shape_aggregated_utterances_summary_list;

pub(crate) mod shape_associated_transcript_filter;

pub(crate) mod shape_associated_transcript_list;

pub(crate) mod shape_bot_alias_history_events_list;

pub(crate) mod shape_bot_alias_locale_settings;

pub(crate) mod shape_bot_alias_locale_settings_map;

pub(crate) mod shape_bot_alias_summary_list;

pub(crate) mod shape_bot_filter;

pub(crate) mod shape_bot_locale_filter;

pub(crate) mod shape_bot_locale_history_events_list;

pub(crate) mod shape_bot_locale_sort_by;

pub(crate) mod shape_bot_locale_summary_list;

pub(crate) mod shape_bot_member;

pub(crate) mod shape_bot_members;

pub(crate) mod shape_bot_recommendation_results;

pub(crate) mod shape_bot_recommendation_summary_list;

pub(crate) mod shape_bot_sort_by;

pub(crate) mod shape_bot_summary_list;

pub(crate) mod shape_bot_version_locale_details;

pub(crate) mod shape_bot_version_locale_specification;

pub(crate) mod shape_bot_version_sort_by;

pub(crate) mod shape_bot_version_summary_list;

pub(crate) mod shape_built_in_intent_sort_by;

pub(crate) mod shape_built_in_intent_summary_list;

pub(crate) mod shape_built_in_slot_type_sort_by;

pub(crate) mod shape_built_in_slot_type_summary_list;

pub(crate) mod shape_composite_slot_type_setting;

pub(crate) mod shape_conversation_log_settings;

pub(crate) mod shape_custom_vocabulary_entry_id;

pub(crate) mod shape_custom_vocabulary_item;

pub(crate) mod shape_custom_vocabulary_items;

pub(crate) mod shape_data_privacy;

pub(crate) mod shape_dialog_code_hook_settings;

pub(crate) mod shape_encryption_setting;

pub(crate) mod shape_export_filter;

pub(crate) mod shape_export_resource_specification;

pub(crate) mod shape_export_sort_by;

pub(crate) mod shape_export_summary_list;

pub(crate) mod shape_external_source_setting;

pub(crate) mod shape_failed_custom_vocabulary_items;

pub(crate) mod shape_failure_reasons;

pub(crate) mod shape_fulfillment_code_hook_settings;

pub(crate) mod shape_import_filter;

pub(crate) mod shape_import_resource_specification;

pub(crate) mod shape_import_sort_by;

pub(crate) mod shape_import_summary_list;

pub(crate) mod shape_initial_response_setting;

pub(crate) mod shape_input_context;

pub(crate) mod shape_input_contexts_list;

pub(crate) mod shape_intent_closing_setting;

pub(crate) mod shape_intent_confirmation_setting;

pub(crate) mod shape_intent_filter;

pub(crate) mod shape_intent_sort_by;

pub(crate) mod shape_intent_summary_list;

pub(crate) mod shape_kendra_configuration;

pub(crate) mod shape_multiple_values_setting;

pub(crate) mod shape_new_custom_vocabulary_item;

pub(crate) mod shape_obfuscation_setting;

pub(crate) mod shape_output_context;

pub(crate) mod shape_output_contexts_list;

pub(crate) mod shape_parent_bot_networks;

pub(crate) mod shape_principal;

pub(crate) mod shape_recommended_actions;

pub(crate) mod shape_recommended_intent_summary_list;

pub(crate) mod shape_sample_utterance;

pub(crate) mod shape_sample_utterances_list;

pub(crate) mod shape_sentiment_analysis_settings;

pub(crate) mod shape_slot_filter;

pub(crate) mod shape_slot_priorities_list;

pub(crate) mod shape_slot_priority;

pub(crate) mod shape_slot_sort_by;

pub(crate) mod shape_slot_summary_list;

pub(crate) mod shape_slot_type_filter;

pub(crate) mod shape_slot_type_sort_by;

pub(crate) mod shape_slot_type_summary_list;

pub(crate) mod shape_slot_type_value;

pub(crate) mod shape_slot_type_values;

pub(crate) mod shape_slot_value_elicitation_setting;

pub(crate) mod shape_slot_value_selection_setting;

pub(crate) mod shape_sub_slot_setting;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_transcript_source_setting;

pub(crate) mod shape_utterance_aggregation_duration;

pub(crate) mod shape_voice_settings;

pub(crate) mod shape_advanced_recognition_setting;

pub(crate) mod shape_aggregated_utterances_summary;

pub(crate) mod shape_associated_transcript;

pub(crate) mod shape_audio_log_setting;

pub(crate) mod shape_audio_log_settings_list;

pub(crate) mod shape_bot_alias_history_event;

pub(crate) mod shape_bot_alias_summary;

pub(crate) mod shape_bot_export_specification;

pub(crate) mod shape_bot_import_specification;

pub(crate) mod shape_bot_locale_export_specification;

pub(crate) mod shape_bot_locale_history_event;

pub(crate) mod shape_bot_locale_import_specification;

pub(crate) mod shape_bot_locale_summary;

pub(crate) mod shape_bot_recommendation_result_statistics;

pub(crate) mod shape_bot_recommendation_summary;

pub(crate) mod shape_bot_summary;

pub(crate) mod shape_bot_version_summary;

pub(crate) mod shape_built_in_intent_summary;

pub(crate) mod shape_built_in_slot_type_summary;

pub(crate) mod shape_code_hook_specification;

pub(crate) mod shape_conditional_specification;

pub(crate) mod shape_custom_vocabulary_export_specification;

pub(crate) mod shape_custom_vocabulary_import_specification;

pub(crate) mod shape_dialog_code_hook_invocation_setting;

pub(crate) mod shape_dialog_state;

pub(crate) mod shape_elicitation_code_hook_invocation_setting;

pub(crate) mod shape_export_summary;

pub(crate) mod shape_failed_custom_vocabulary_item;

pub(crate) mod shape_fulfillment_updates_specification;

pub(crate) mod shape_grammar_slot_type_setting;

pub(crate) mod shape_import_summary;

pub(crate) mod shape_intent_summary;

pub(crate) mod shape_parent_bot_network;

pub(crate) mod shape_post_fulfillment_status_specification;

pub(crate) mod shape_prompt_specification;

pub(crate) mod shape_recommended_intent_summary;

pub(crate) mod shape_relative_aggregation_duration;

pub(crate) mod shape_response_specification;

pub(crate) mod shape_s3_bucket_transcript_source;

pub(crate) mod shape_sample_value;

pub(crate) mod shape_slot_capture_setting;

pub(crate) mod shape_slot_default_value_specification;

pub(crate) mod shape_slot_summary;

pub(crate) mod shape_slot_type_summary;

pub(crate) mod shape_slot_value_regex_filter;

pub(crate) mod shape_specifications;

pub(crate) mod shape_sub_slot_specification_map;

pub(crate) mod shape_sub_slot_type_composition;

pub(crate) mod shape_sub_slot_type_list;

pub(crate) mod shape_text_log_setting;

pub(crate) mod shape_text_log_settings_list;

pub(crate) mod shape_wait_and_continue_specification;

pub(crate) mod shape_audio_log_destination;

pub(crate) mod shape_conditional_branch;

pub(crate) mod shape_conditional_branches;

pub(crate) mod shape_default_conditional_branch;

pub(crate) mod shape_dialog_action;

pub(crate) mod shape_fulfillment_start_response_specification;

pub(crate) mod shape_fulfillment_update_response_specification;

pub(crate) mod shape_grammar_slot_type_source;

pub(crate) mod shape_intent_override;

pub(crate) mod shape_intent_statistics;

pub(crate) mod shape_lambda_code_hook;

pub(crate) mod shape_message_group;

pub(crate) mod shape_message_groups_list;

pub(crate) mod shape_path_format;

pub(crate) mod shape_post_dialog_code_hook_invocation_specification;

pub(crate) mod shape_prompt_attempt_specification;

pub(crate) mod shape_prompt_attempts_specification_map;

pub(crate) mod shape_slot_default_value;

pub(crate) mod shape_slot_default_value_list;

pub(crate) mod shape_slot_type_statistics;

pub(crate) mod shape_still_waiting_response_specification;

pub(crate) mod shape_string_map;

pub(crate) mod shape_sub_slot_value_elicitation_setting;

pub(crate) mod shape_synonym_list;

pub(crate) mod shape_text_log_destination;

pub(crate) mod shape_transcript_filter;

pub(crate) mod shape_allowed_input_types;

pub(crate) mod shape_audio_and_dtmf_input_specification;

pub(crate) mod shape_cloud_watch_log_group_log_destination;

pub(crate) mod shape_condition;

pub(crate) mod shape_lex_transcript_filter;

pub(crate) mod shape_message;

pub(crate) mod shape_object_prefixes;

pub(crate) mod shape_s3_bucket_log_destination;

pub(crate) mod shape_slot_value_override;

pub(crate) mod shape_slot_value_override_map;

pub(crate) mod shape_text_input_specification;

pub(crate) mod shape_audio_specification;

pub(crate) mod shape_custom_payload;

pub(crate) mod shape_date_range_filter;

pub(crate) mod shape_dtmf_specification;

pub(crate) mod shape_image_response_card;

pub(crate) mod shape_message_variations_list;

pub(crate) mod shape_plain_text_message;

pub(crate) mod shape_slot_value;

pub(crate) mod shape_ssml_message;

pub(crate) mod shape_button;

pub(crate) mod shape_slot_values;

pub(crate) mod shape_buttons_list;
