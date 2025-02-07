// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateStage`](crate::operation::update_stage::builders::UpdateStageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl ::std::convert::Into<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::set_arn): <p>ARN of the stage to be updated.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_stage::builders::UpdateStageFluentBuilder::set_name): <p>Name of the stage to be updated.</p>
    /// - On success, responds with [`UpdateStageOutput`](crate::operation::update_stage::UpdateStageOutput) with field(s):
    ///   - [`stage(Option<Stage>)`](crate::operation::update_stage::UpdateStageOutput::stage): <p>The updated stage.</p>
    /// - On failure, responds with [`SdkError<UpdateStageError>`](crate::operation::update_stage::UpdateStageError)
    pub fn update_stage(
        &self,
    ) -> crate::operation::update_stage::builders::UpdateStageFluentBuilder {
        crate::operation::update_stage::builders::UpdateStageFluentBuilder::new(self.handle.clone())
    }
}
