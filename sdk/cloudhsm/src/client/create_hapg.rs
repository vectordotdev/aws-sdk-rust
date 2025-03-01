// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateHapg`](crate::operation::create_hapg::builders::CreateHapgFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`label(impl ::std::convert::Into<String>)`](crate::operation::create_hapg::builders::CreateHapgFluentBuilder::label) / [`set_label(Option<String>)`](crate::operation::create_hapg::builders::CreateHapgFluentBuilder::set_label): <p>The label of the new high-availability partition group.</p>
    /// - On success, responds with [`CreateHapgOutput`](crate::operation::create_hapg::CreateHapgOutput) with field(s):
    ///   - [`hapg_arn(Option<String>)`](crate::operation::create_hapg::CreateHapgOutput::hapg_arn): <p>The ARN of the high-availability partition group.</p>
    /// - On failure, responds with [`SdkError<CreateHapgError>`](crate::operation::create_hapg::CreateHapgError)
    pub fn create_hapg(&self) -> crate::operation::create_hapg::builders::CreateHapgFluentBuilder {
        crate::operation::create_hapg::builders::CreateHapgFluentBuilder::new(self.handle.clone())
    }
}
