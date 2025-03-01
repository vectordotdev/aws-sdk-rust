// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_inference_experiment_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::stop_inference_experiment::StopInferenceExperimentInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.model_variant_actions {
        #[allow(unused_mut)]
        let mut object_3 = object.key("ModelVariantActions").start_object();
        for (key_4, value_5) in var_2 {
            {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.desired_model_variants {
        let mut array_7 = object.key("DesiredModelVariants").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_model_variant_config::ser_model_variant_config(
                    &mut object_9,
                    item_8,
                )?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.desired_state {
        object.key("DesiredState").string(var_10.as_str());
    }
    if let Some(var_11) = &input.reason {
        object.key("Reason").string(var_11.as_str());
    }
    Ok(())
}
