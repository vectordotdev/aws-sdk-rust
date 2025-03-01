// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_lte_obj(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::LteObj,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.mcc {
        object.key("Mcc").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.mnc {
        object.key("Mnc").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.eutran_cid {
        object.key("EutranCid").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.tac {
        object.key("Tac").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.lte_local_id {
        #[allow(unused_mut)]
        let mut object_6 = object.key("LteLocalId").start_object();
        crate::protocol_serde::shape_lte_local_id::ser_lte_local_id(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.lte_timing_advance {
        object.key("LteTimingAdvance").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.rsrp {
        object.key("Rsrp").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.rsrq {
        object.key("Rsrq").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_9).into()),
        );
    }
    if input.nr_capable {
        object.key("NrCapable").boolean(input.nr_capable);
    }
    if let Some(var_10) = &input.lte_nmr {
        let mut array_11 = object.key("LteNmr").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_lte_nmr_obj::ser_lte_nmr_obj(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}
