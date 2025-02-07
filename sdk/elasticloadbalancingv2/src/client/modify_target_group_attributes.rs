// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyTargetGroupAttributes`](crate::operation::modify_target_group_attributes::builders::ModifyTargetGroupAttributesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`target_group_arn(impl ::std::convert::Into<String>)`](crate::operation::modify_target_group_attributes::builders::ModifyTargetGroupAttributesFluentBuilder::target_group_arn) / [`set_target_group_arn(Option<String>)`](crate::operation::modify_target_group_attributes::builders::ModifyTargetGroupAttributesFluentBuilder::set_target_group_arn): <p>The Amazon Resource Name (ARN) of the target group.</p>
    ///   - [`attributes(Vec<TargetGroupAttribute>)`](crate::operation::modify_target_group_attributes::builders::ModifyTargetGroupAttributesFluentBuilder::attributes) / [`set_attributes(Option<Vec<TargetGroupAttribute>>)`](crate::operation::modify_target_group_attributes::builders::ModifyTargetGroupAttributesFluentBuilder::set_attributes): <p>The attributes.</p>
    /// - On success, responds with [`ModifyTargetGroupAttributesOutput`](crate::operation::modify_target_group_attributes::ModifyTargetGroupAttributesOutput) with field(s):
    ///   - [`attributes(Option<Vec<TargetGroupAttribute>>)`](crate::operation::modify_target_group_attributes::ModifyTargetGroupAttributesOutput::attributes): <p>Information about the attributes.</p>
    /// - On failure, responds with [`SdkError<ModifyTargetGroupAttributesError>`](crate::operation::modify_target_group_attributes::ModifyTargetGroupAttributesError)
    pub fn modify_target_group_attributes(&self) -> crate::operation::modify_target_group_attributes::builders::ModifyTargetGroupAttributesFluentBuilder{
        crate::operation::modify_target_group_attributes::builders::ModifyTargetGroupAttributesFluentBuilder::new(self.handle.clone())
    }
}
