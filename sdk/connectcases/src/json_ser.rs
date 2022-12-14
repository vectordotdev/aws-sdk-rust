// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_get_field_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetFieldInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.fields {
        let mut array_2 = object.key("fields").start_array();
        for item_3 in var_1 {
            {
                let mut object_4 = array_2.value().start_object();
                crate::json_ser::serialize_structure_crate_model_field_identifier(
                    &mut object_4,
                    item_3,
                )?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_put_field_options_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchPutFieldOptionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_5) = &input.options {
        let mut array_6 = object.key("options").start_array();
        for item_7 in var_5 {
            {
                let mut object_8 = array_6.value().start_object();
                crate::json_ser::serialize_structure_crate_model_field_option(
                    &mut object_8,
                    item_7,
                )?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_case_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCaseInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_9) = &input.client_token {
        object.key("clientToken").string(var_9.as_str());
    }
    if let Some(var_10) = &input.fields {
        let mut array_11 = object.key("fields").start_array();
        for item_12 in var_10 {
            {
                let mut object_13 = array_11.value().start_object();
                crate::json_ser::serialize_structure_crate_model_field_value(
                    &mut object_13,
                    item_12,
                )?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.template_id {
        object.key("templateId").string(var_14.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_15) = &input.name {
        object.key("name").string(var_15.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_field_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateFieldInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_16) = &input.description {
        object.key("description").string(var_16.as_str());
    }
    if let Some(var_17) = &input.name {
        object.key("name").string(var_17.as_str());
    }
    if let Some(var_18) = &input.r#type {
        object.key("type").string(var_18.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_layout_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLayoutInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_19) = &input.content {
        let mut object_20 = object.key("content").start_object();
        crate::json_ser::serialize_union_crate_model_layout_content(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.name {
        object.key("name").string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_related_item_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRelatedItemInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.content {
        let mut object_23 = object.key("content").start_object();
        crate::json_ser::serialize_union_crate_model_related_item_input_content(
            &mut object_23,
            var_22,
        )?;
        object_23.finish();
    }
    if let Some(var_24) = &input.r#type {
        object.key("type").string(var_24.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTemplateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_25) = &input.description {
        object.key("description").string(var_25.as_str());
    }
    if let Some(var_26) = &input.layout_configuration {
        let mut object_27 = object.key("layoutConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_layout_configuration(
            &mut object_27,
            var_26,
        )?;
        object_27.finish();
    }
    if let Some(var_28) = &input.name {
        object.key("name").string(var_28.as_str());
    }
    if let Some(var_29) = &input.required_fields {
        let mut array_30 = object.key("requiredFields").start_array();
        for item_31 in var_29 {
            {
                let mut object_32 = array_30.value().start_object();
                crate::json_ser::serialize_structure_crate_model_required_field(
                    &mut object_32,
                    item_31,
                )?;
                object_32.finish();
            }
        }
        array_30.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_case_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetCaseInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_33) = &input.fields {
        let mut array_34 = object.key("fields").start_array();
        for item_35 in var_33 {
            {
                let mut object_36 = array_34.value().start_object();
                crate::json_ser::serialize_structure_crate_model_field_identifier(
                    &mut object_36,
                    item_35,
                )?;
                object_36.finish();
            }
        }
        array_34.finish();
    }
    if let Some(var_37) = &input.next_token {
        object.key("nextToken").string(var_37.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_cases_for_contact_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListCasesForContactInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_38) = &input.contact_arn {
        object.key("contactArn").string(var_38.as_str());
    }
    if let Some(var_39) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_39).into()),
        );
    }
    if let Some(var_40) = &input.next_token {
        object.key("nextToken").string(var_40.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_case_event_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutCaseEventConfigurationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_41) = &input.event_bridge {
        let mut object_42 = object.key("eventBridge").start_object();
        crate::json_ser::serialize_structure_crate_model_event_bridge_configuration(
            &mut object_42,
            var_41,
        )?;
        object_42.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_search_cases_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SearchCasesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_43) = &input.fields {
        let mut array_44 = object.key("fields").start_array();
        for item_45 in var_43 {
            {
                let mut object_46 = array_44.value().start_object();
                crate::json_ser::serialize_structure_crate_model_field_identifier(
                    &mut object_46,
                    item_45,
                )?;
                object_46.finish();
            }
        }
        array_44.finish();
    }
    if let Some(var_47) = &input.filter {
        let mut object_48 = object.key("filter").start_object();
        crate::json_ser::serialize_union_crate_model_case_filter(&mut object_48, var_47)?;
        object_48.finish();
    }
    if let Some(var_49) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_49).into()),
        );
    }
    if let Some(var_50) = &input.next_token {
        object.key("nextToken").string(var_50.as_str());
    }
    if let Some(var_51) = &input.search_term {
        object.key("searchTerm").string(var_51.as_str());
    }
    if let Some(var_52) = &input.sorts {
        let mut array_53 = object.key("sorts").start_array();
        for item_54 in var_52 {
            {
                let mut object_55 = array_53.value().start_object();
                crate::json_ser::serialize_structure_crate_model_sort(&mut object_55, item_54)?;
                object_55.finish();
            }
        }
        array_53.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_search_related_items_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SearchRelatedItemsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_56) = &input.filters {
        let mut array_57 = object.key("filters").start_array();
        for item_58 in var_56 {
            {
                let mut object_59 = array_57.value().start_object();
                crate::json_ser::serialize_union_crate_model_related_item_type_filter(
                    &mut object_59,
                    item_58,
                )?;
                object_59.finish();
            }
        }
        array_57.finish();
    }
    if let Some(var_60) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_60).into()),
        );
    }
    if let Some(var_61) = &input.next_token {
        object.key("nextToken").string(var_61.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_62) = &input.tags {
        let mut object_63 = object.key("tags").start_object();
        for (key_64, value_65) in var_62 {
            if let Some(var_66) = value_65 {
                object_63.key(key_64.as_str()).string(var_66.as_str());
            } else {
                object_63.key(key_64.as_str()).null();
            }
        }
        object_63.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_case_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCaseInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_67) = &input.fields {
        let mut array_68 = object.key("fields").start_array();
        for item_69 in var_67 {
            {
                let mut object_70 = array_68.value().start_object();
                crate::json_ser::serialize_structure_crate_model_field_value(
                    &mut object_70,
                    item_69,
                )?;
                object_70.finish();
            }
        }
        array_68.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_field_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateFieldInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_71) = &input.description {
        object.key("description").string(var_71.as_str());
    }
    if let Some(var_72) = &input.name {
        object.key("name").string(var_72.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_layout_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLayoutInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_73) = &input.content {
        let mut object_74 = object.key("content").start_object();
        crate::json_ser::serialize_union_crate_model_layout_content(&mut object_74, var_73)?;
        object_74.finish();
    }
    if let Some(var_75) = &input.name {
        object.key("name").string(var_75.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTemplateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_76) = &input.description {
        object.key("description").string(var_76.as_str());
    }
    if let Some(var_77) = &input.layout_configuration {
        let mut object_78 = object.key("layoutConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_layout_configuration(
            &mut object_78,
            var_77,
        )?;
        object_78.finish();
    }
    if let Some(var_79) = &input.name {
        object.key("name").string(var_79.as_str());
    }
    if let Some(var_80) = &input.required_fields {
        let mut array_81 = object.key("requiredFields").start_array();
        for item_82 in var_80 {
            {
                let mut object_83 = array_81.value().start_object();
                crate::json_ser::serialize_structure_crate_model_required_field(
                    &mut object_83,
                    item_82,
                )?;
                object_83.finish();
            }
        }
        array_81.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_field_identifier(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FieldIdentifier,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_84) = &input.id {
        object.key("id").string(var_84.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_field_option(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FieldOption,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_85) = &input.name {
        object.key("name").string(var_85.as_str());
    }
    if let Some(var_86) = &input.value {
        object.key("value").string(var_86.as_str());
    }
    if let Some(var_87) = &input.active {
        object.key("active").boolean(*var_87);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_field_value(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FieldValue,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_88) = &input.id {
        object.key("id").string(var_88.as_str());
    }
    if let Some(var_89) = &input.value {
        let mut object_90 = object.key("value").start_object();
        crate::json_ser::serialize_union_crate_model_field_value_union(&mut object_90, var_89)?;
        object_90.finish();
    }
    Ok(())
}

pub fn serialize_union_crate_model_layout_content(
    object_20: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LayoutContent,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::LayoutContent::Basic(inner) => {
            let mut object_91 = object_20.key("basic").start_object();
            crate::json_ser::serialize_structure_crate_model_basic_layout(&mut object_91, inner)?;
            object_91.finish();
        }
        crate::model::LayoutContent::Unknown => {
            return Err(
                aws_smithy_http::operation::error::SerializationError::unknown_variant(
                    "LayoutContent",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_union_crate_model_related_item_input_content(
    object_23: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RelatedItemInputContent,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::RelatedItemInputContent::Contact(inner) => {
            let mut object_92 = object_23.key("contact").start_object();
            crate::json_ser::serialize_structure_crate_model_contact(&mut object_92, inner)?;
            object_92.finish();
        }
        crate::model::RelatedItemInputContent::Comment(inner) => {
            let mut object_93 = object_23.key("comment").start_object();
            crate::json_ser::serialize_structure_crate_model_comment_content(
                &mut object_93,
                inner,
            )?;
            object_93.finish();
        }
        crate::model::RelatedItemInputContent::Unknown => {
            return Err(
                aws_smithy_http::operation::error::SerializationError::unknown_variant(
                    "RelatedItemInputContent",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_layout_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LayoutConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_94) = &input.default_layout {
        object.key("defaultLayout").string(var_94.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_required_field(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RequiredField,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_95) = &input.field_id {
        object.key("fieldId").string(var_95.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_event_bridge_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EventBridgeConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_96) = &input.enabled {
        object.key("enabled").boolean(*var_96);
    }
    if let Some(var_97) = &input.included_data {
        let mut object_98 = object.key("includedData").start_object();
        crate::json_ser::serialize_structure_crate_model_event_included_data(
            &mut object_98,
            var_97,
        )?;
        object_98.finish();
    }
    Ok(())
}

pub fn serialize_union_crate_model_case_filter(
    object_48: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CaseFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::CaseFilter::Field(inner) => {
            let mut object_99 = object_48.key("field").start_object();
            crate::json_ser::serialize_union_crate_model_field_filter(&mut object_99, inner)?;
            object_99.finish();
        }
        crate::model::CaseFilter::Not(inner) => {
            let mut object_100 = object_48.key("not").start_object();
            crate::json_ser::serialize_union_crate_model_case_filter(&mut object_100, inner)?;
            object_100.finish();
        }
        crate::model::CaseFilter::AndAll(inner) => {
            let mut array_101 = object_48.key("andAll").start_array();
            for item_102 in inner {
                {
                    let mut object_103 = array_101.value().start_object();
                    crate::json_ser::serialize_union_crate_model_case_filter(
                        &mut object_103,
                        item_102,
                    )?;
                    object_103.finish();
                }
            }
            array_101.finish();
        }
        crate::model::CaseFilter::Unknown => {
            return Err(
                aws_smithy_http::operation::error::SerializationError::unknown_variant(
                    "CaseFilter",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_sort(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Sort,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_104) = &input.field_id {
        object.key("fieldId").string(var_104.as_str());
    }
    if let Some(var_105) = &input.sort_order {
        object.key("sortOrder").string(var_105.as_str());
    }
    Ok(())
}

pub fn serialize_union_crate_model_related_item_type_filter(
    object_59: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RelatedItemTypeFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::RelatedItemTypeFilter::Contact(inner) => {
            let mut object_106 = object_59.key("contact").start_object();
            crate::json_ser::serialize_structure_crate_model_contact_filter(
                &mut object_106,
                inner,
            )?;
            object_106.finish();
        }
        crate::model::RelatedItemTypeFilter::Comment(inner) => {
            let mut object_107 = object_59.key("comment").start_object();
            crate::json_ser::serialize_structure_crate_model_comment_filter(
                &mut object_107,
                inner,
            )?;
            object_107.finish();
        }
        crate::model::RelatedItemTypeFilter::Unknown => {
            return Err(
                aws_smithy_http::operation::error::SerializationError::unknown_variant(
                    "RelatedItemTypeFilter",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_union_crate_model_field_value_union(
    object_90: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FieldValueUnion,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::FieldValueUnion::StringValue(inner) => {
            object_90.key("stringValue").string(inner.as_str());
        }
        crate::model::FieldValueUnion::DoubleValue(inner) => {
            object_90.key("doubleValue").number(
                #[allow(clippy::useless_conversion)]
                aws_smithy_types::Number::Float((*inner).into()),
            );
        }
        crate::model::FieldValueUnion::BooleanValue(inner) => {
            object_90.key("booleanValue").boolean(*inner);
        }
        crate::model::FieldValueUnion::Unknown => {
            return Err(
                aws_smithy_http::operation::error::SerializationError::unknown_variant(
                    "FieldValueUnion",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_basic_layout(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BasicLayout,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_108) = &input.top_panel {
        let mut object_109 = object.key("topPanel").start_object();
        crate::json_ser::serialize_structure_crate_model_layout_sections(&mut object_109, var_108)?;
        object_109.finish();
    }
    if let Some(var_110) = &input.more_info {
        let mut object_111 = object.key("moreInfo").start_object();
        crate::json_ser::serialize_structure_crate_model_layout_sections(&mut object_111, var_110)?;
        object_111.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_contact(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Contact,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_112) = &input.contact_arn {
        object.key("contactArn").string(var_112.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_comment_content(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CommentContent,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_113) = &input.body {
        object.key("body").string(var_113.as_str());
    }
    if let Some(var_114) = &input.content_type {
        object.key("contentType").string(var_114.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_event_included_data(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EventIncludedData,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_115) = &input.case_data {
        let mut object_116 = object.key("caseData").start_object();
        crate::json_ser::serialize_structure_crate_model_case_event_included_data(
            &mut object_116,
            var_115,
        )?;
        object_116.finish();
    }
    if let Some(var_117) = &input.related_item_data {
        let mut object_118 = object.key("relatedItemData").start_object();
        crate::json_ser::serialize_structure_crate_model_related_item_event_included_data(
            &mut object_118,
            var_117,
        )?;
        object_118.finish();
    }
    Ok(())
}

pub fn serialize_union_crate_model_field_filter(
    object_99: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FieldFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::FieldFilter::EqualTo(inner) => {
            let mut object_119 = object_99.key("equalTo").start_object();
            crate::json_ser::serialize_structure_crate_model_field_value(&mut object_119, inner)?;
            object_119.finish();
        }
        crate::model::FieldFilter::Contains(inner) => {
            let mut object_120 = object_99.key("contains").start_object();
            crate::json_ser::serialize_structure_crate_model_field_value(&mut object_120, inner)?;
            object_120.finish();
        }
        crate::model::FieldFilter::GreaterThan(inner) => {
            let mut object_121 = object_99.key("greaterThan").start_object();
            crate::json_ser::serialize_structure_crate_model_field_value(&mut object_121, inner)?;
            object_121.finish();
        }
        crate::model::FieldFilter::GreaterThanOrEqualTo(inner) => {
            let mut object_122 = object_99.key("greaterThanOrEqualTo").start_object();
            crate::json_ser::serialize_structure_crate_model_field_value(&mut object_122, inner)?;
            object_122.finish();
        }
        crate::model::FieldFilter::LessThan(inner) => {
            let mut object_123 = object_99.key("lessThan").start_object();
            crate::json_ser::serialize_structure_crate_model_field_value(&mut object_123, inner)?;
            object_123.finish();
        }
        crate::model::FieldFilter::LessThanOrEqualTo(inner) => {
            let mut object_124 = object_99.key("lessThanOrEqualTo").start_object();
            crate::json_ser::serialize_structure_crate_model_field_value(&mut object_124, inner)?;
            object_124.finish();
        }
        crate::model::FieldFilter::Unknown => {
            return Err(
                aws_smithy_http::operation::error::SerializationError::unknown_variant(
                    "FieldFilter",
                ),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_contact_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ContactFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_125) = &input.channel {
        let mut array_126 = object.key("channel").start_array();
        for item_127 in var_125 {
            {
                array_126.value().string(item_127.as_str());
            }
        }
        array_126.finish();
    }
    if let Some(var_128) = &input.contact_arn {
        object.key("contactArn").string(var_128.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_comment_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CommentFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    let (_, _) = (object, input);
    Ok(())
}

pub fn serialize_structure_crate_model_layout_sections(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LayoutSections,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_129) = &input.sections {
        let mut array_130 = object.key("sections").start_array();
        for item_131 in var_129 {
            {
                let mut object_132 = array_130.value().start_object();
                crate::json_ser::serialize_union_crate_model_section(&mut object_132, item_131)?;
                object_132.finish();
            }
        }
        array_130.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_case_event_included_data(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CaseEventIncludedData,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_133) = &input.fields {
        let mut array_134 = object.key("fields").start_array();
        for item_135 in var_133 {
            {
                let mut object_136 = array_134.value().start_object();
                crate::json_ser::serialize_structure_crate_model_field_identifier(
                    &mut object_136,
                    item_135,
                )?;
                object_136.finish();
            }
        }
        array_134.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_related_item_event_included_data(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RelatedItemEventIncludedData,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_137) = &input.include_content {
        object.key("includeContent").boolean(*var_137);
    }
    Ok(())
}

pub fn serialize_union_crate_model_section(
    object_132: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Section,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::Section::FieldGroup(inner) => {
            let mut object_138 = object_132.key("fieldGroup").start_object();
            crate::json_ser::serialize_structure_crate_model_field_group(&mut object_138, inner)?;
            object_138.finish();
        }
        crate::model::Section::Unknown => {
            return Err(
                aws_smithy_http::operation::error::SerializationError::unknown_variant("Section"),
            )
        }
    }
    Ok(())
}

pub fn serialize_structure_crate_model_field_group(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FieldGroup,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_139) = &input.name {
        object.key("name").string(var_139.as_str());
    }
    if let Some(var_140) = &input.fields {
        let mut array_141 = object.key("fields").start_array();
        for item_142 in var_140 {
            {
                let mut object_143 = array_141.value().start_object();
                crate::json_ser::serialize_structure_crate_model_field_item(
                    &mut object_143,
                    item_142,
                )?;
                object_143.finish();
            }
        }
        array_141.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_field_item(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FieldItem,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_144) = &input.id {
        object.key("id").string(var_144.as_str());
    }
    Ok(())
}
