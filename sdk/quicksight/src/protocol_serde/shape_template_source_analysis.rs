// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_template_source_analysis(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TemplateSourceAnalysis,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.arn {
        object.key("Arn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.data_set_references {
        let mut array_3 = object.key("DataSetReferences").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_data_set_reference::ser_data_set_reference(
                    &mut object_5,
                    item_4,
                )?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}
