// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_delete_message_batch_request_entry(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::DeleteMessageBatchRequestEntry,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Id");
    if let Some(var_2) = &input.id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ReceiptHandle");
    if let Some(var_4) = &input.receipt_handle {
        scope_3.string(var_4);
    }
    Ok(())
}
