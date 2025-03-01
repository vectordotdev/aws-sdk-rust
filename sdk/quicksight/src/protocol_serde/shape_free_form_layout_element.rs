// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_free_form_layout_element(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::FreeFormLayoutElement,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.element_id {
        object.key("ElementId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.element_type {
        object.key("ElementType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.x_axis_location {
        object.key("XAxisLocation").string(var_3.as_str());
    }
    if let Some(var_4) = &input.y_axis_location {
        object.key("YAxisLocation").string(var_4.as_str());
    }
    if let Some(var_5) = &input.width {
        object.key("Width").string(var_5.as_str());
    }
    if let Some(var_6) = &input.height {
        object.key("Height").string(var_6.as_str());
    }
    if let Some(var_7) = &input.visibility {
        object.key("Visibility").string(var_7.as_str());
    }
    if let Some(var_8) = &input.rendering_rules {
        let mut array_9 = object.key("RenderingRules").start_array();
        for item_10 in var_8 {
            {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_sheet_element_rendering_rule::ser_sheet_element_rendering_rule(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.border_style {
        #[allow(unused_mut)]
        let mut object_13 = object.key("BorderStyle").start_object();
        crate::protocol_serde::shape_free_form_layout_element_border_style::ser_free_form_layout_element_border_style(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.selected_border_style {
        #[allow(unused_mut)]
        let mut object_15 = object.key("SelectedBorderStyle").start_object();
        crate::protocol_serde::shape_free_form_layout_element_border_style::ser_free_form_layout_element_border_style(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.background_style {
        #[allow(unused_mut)]
        let mut object_17 = object.key("BackgroundStyle").start_object();
        crate::protocol_serde::shape_free_form_layout_element_background_style::ser_free_form_layout_element_background_style(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.loading_animation {
        #[allow(unused_mut)]
        let mut object_19 = object.key("LoadingAnimation").start_object();
        crate::protocol_serde::shape_loading_animation::ser_loading_animation(
            &mut object_19,
            var_18,
        )?;
        object_19.finish();
    }
    Ok(())
}

pub(crate) fn de_free_form_layout_element<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::FreeFormLayoutElement>,
    ::aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<
        Item = Result<
            ::aws_smithy_json::deserialize::Token<'a>,
            ::aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::FreeFormLayoutElementBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ElementId" => {
                                builder = builder.set_element_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "ElementType" => {
                                builder = builder.set_element_type(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::LayoutElementType::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "XAxisLocation" => {
                                builder = builder.set_x_axis_location(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "YAxisLocation" => {
                                builder = builder.set_y_axis_location(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Width" => {
                                builder = builder.set_width(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Height" => {
                                builder = builder.set_height(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "Visibility" => {
                                builder = builder.set_visibility(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::Visibility::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "RenderingRules" => {
                                builder = builder.set_rendering_rules(
                                    crate::protocol_serde::shape_sheet_element_rendering_rule_list::de_sheet_element_rendering_rule_list(tokens)?
                                );
                            }
                            "BorderStyle" => {
                                builder = builder.set_border_style(
                                    crate::protocol_serde::shape_free_form_layout_element_border_style::de_free_form_layout_element_border_style(tokens)?
                                );
                            }
                            "SelectedBorderStyle" => {
                                builder = builder.set_selected_border_style(
                                    crate::protocol_serde::shape_free_form_layout_element_border_style::de_free_form_layout_element_border_style(tokens)?
                                );
                            }
                            "BackgroundStyle" => {
                                builder = builder.set_background_style(
                                    crate::protocol_serde::shape_free_form_layout_element_background_style::de_free_form_layout_element_background_style(tokens)?
                                );
                            }
                            "LoadingAnimation" => {
                                builder = builder.set_loading_animation(
                                    crate::protocol_serde::shape_loading_animation::de_loading_animation(tokens)?
                                );
                            }
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    other => {
                        return Err(
                            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                format!("expected object key or end object, found: {:?}", other),
                            ),
                        )
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(
            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ),
        ),
    }
}
