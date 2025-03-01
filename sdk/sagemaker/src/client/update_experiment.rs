// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateExperiment`](crate::operation::update_experiment::builders::UpdateExperimentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`experiment_name(impl ::std::convert::Into<String>)`](crate::operation::update_experiment::builders::UpdateExperimentFluentBuilder::experiment_name) / [`set_experiment_name(Option<String>)`](crate::operation::update_experiment::builders::UpdateExperimentFluentBuilder::set_experiment_name): <p>The name of the experiment to update.</p>
    ///   - [`display_name(impl ::std::convert::Into<String>)`](crate::operation::update_experiment::builders::UpdateExperimentFluentBuilder::display_name) / [`set_display_name(Option<String>)`](crate::operation::update_experiment::builders::UpdateExperimentFluentBuilder::set_display_name): <p>The name of the experiment as displayed. The name doesn't need to be unique. If <code>DisplayName</code> isn't specified, <code>ExperimentName</code> is displayed.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_experiment::builders::UpdateExperimentFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_experiment::builders::UpdateExperimentFluentBuilder::set_description): <p>The description of the experiment.</p>
    /// - On success, responds with [`UpdateExperimentOutput`](crate::operation::update_experiment::UpdateExperimentOutput) with field(s):
    ///   - [`experiment_arn(Option<String>)`](crate::operation::update_experiment::UpdateExperimentOutput::experiment_arn): <p>The Amazon Resource Name (ARN) of the experiment.</p>
    /// - On failure, responds with [`SdkError<UpdateExperimentError>`](crate::operation::update_experiment::UpdateExperimentError)
    pub fn update_experiment(
        &self,
    ) -> crate::operation::update_experiment::builders::UpdateExperimentFluentBuilder {
        crate::operation::update_experiment::builders::UpdateExperimentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
