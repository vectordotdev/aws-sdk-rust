// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_bot_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::disassociate_bot::DisassociateBotInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.lex_bot {
        #[allow(unused_mut)]
        let mut object_2 = object.key("LexBot").start_object();
        crate::protocol_serde::shape_lex_bot::ser_lex_bot(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.lex_v2_bot {
        #[allow(unused_mut)]
        let mut object_4 = object.key("LexV2Bot").start_object();
        crate::protocol_serde::shape_lex_v2_bot::ser_lex_v2_bot(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
