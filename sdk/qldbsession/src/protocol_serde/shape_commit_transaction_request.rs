// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_commit_transaction_request(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CommitTransactionRequest,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.transaction_id {
        object.key("TransactionId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.commit_digest {
        object
            .key("CommitDigest")
            .string_unchecked(&::aws_smithy_types::base64::encode(var_2));
    }
    Ok(())
}
