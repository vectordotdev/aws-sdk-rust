// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RegisterTargets`](crate::operation::register_targets::builders::RegisterTargetsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`target_group_arn(impl ::std::convert::Into<String>)`](crate::operation::register_targets::builders::RegisterTargetsFluentBuilder::target_group_arn) / [`set_target_group_arn(Option<String>)`](crate::operation::register_targets::builders::RegisterTargetsFluentBuilder::set_target_group_arn): <p>The Amazon Resource Name (ARN) of the target group.</p>
    ///   - [`targets(Vec<TargetDescription>)`](crate::operation::register_targets::builders::RegisterTargetsFluentBuilder::targets) / [`set_targets(Option<Vec<TargetDescription>>)`](crate::operation::register_targets::builders::RegisterTargetsFluentBuilder::set_targets): <p>The targets.</p>
    /// - On success, responds with [`RegisterTargetsOutput`](crate::operation::register_targets::RegisterTargetsOutput)
    /// - On failure, responds with [`SdkError<RegisterTargetsError>`](crate::operation::register_targets::RegisterTargetsError)
    pub fn register_targets(
        &self,
    ) -> crate::operation::register_targets::builders::RegisterTargetsFluentBuilder {
        crate::operation::register_targets::builders::RegisterTargetsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
