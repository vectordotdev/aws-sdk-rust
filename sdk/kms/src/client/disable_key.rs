// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisableKey`](crate::operation::disable_key::builders::DisableKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl ::std::convert::Into<String>)`](crate::operation::disable_key::builders::DisableKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::disable_key::builders::DisableKeyFluentBuilder::set_key_id): <p>Identifies the KMS key to disable.</p>  <p>Specify the key ID or key ARN of the KMS key.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    /// - On success, responds with [`DisableKeyOutput`](crate::operation::disable_key::DisableKeyOutput)
    /// - On failure, responds with [`SdkError<DisableKeyError>`](crate::operation::disable_key::DisableKeyError)
    pub fn disable_key(&self) -> crate::operation::disable_key::builders::DisableKeyFluentBuilder {
        crate::operation::disable_key::builders::DisableKeyFluentBuilder::new(self.handle.clone())
    }
}
