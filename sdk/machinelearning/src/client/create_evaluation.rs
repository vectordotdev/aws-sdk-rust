// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateEvaluation`](crate::operation::create_evaluation::builders::CreateEvaluationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`evaluation_id(impl ::std::convert::Into<String>)`](crate::operation::create_evaluation::builders::CreateEvaluationFluentBuilder::evaluation_id) / [`set_evaluation_id(Option<String>)`](crate::operation::create_evaluation::builders::CreateEvaluationFluentBuilder::set_evaluation_id): <p>A user-supplied ID that uniquely identifies the <code>Evaluation</code>.</p>
    ///   - [`evaluation_name(impl ::std::convert::Into<String>)`](crate::operation::create_evaluation::builders::CreateEvaluationFluentBuilder::evaluation_name) / [`set_evaluation_name(Option<String>)`](crate::operation::create_evaluation::builders::CreateEvaluationFluentBuilder::set_evaluation_name): <p>A user-supplied name or description of the <code>Evaluation</code>.</p>
    ///   - [`ml_model_id(impl ::std::convert::Into<String>)`](crate::operation::create_evaluation::builders::CreateEvaluationFluentBuilder::ml_model_id) / [`set_ml_model_id(Option<String>)`](crate::operation::create_evaluation::builders::CreateEvaluationFluentBuilder::set_ml_model_id): <p>The ID of the <code>MLModel</code> to evaluate.</p>  <p>The schema used in creating the <code>MLModel</code> must match the schema of the <code>DataSource</code> used in the <code>Evaluation</code>.</p>
    ///   - [`evaluation_data_source_id(impl ::std::convert::Into<String>)`](crate::operation::create_evaluation::builders::CreateEvaluationFluentBuilder::evaluation_data_source_id) / [`set_evaluation_data_source_id(Option<String>)`](crate::operation::create_evaluation::builders::CreateEvaluationFluentBuilder::set_evaluation_data_source_id): <p>The ID of the <code>DataSource</code> for the evaluation. The schema of the <code>DataSource</code> must match the schema used to create the <code>MLModel</code>.</p>
    /// - On success, responds with [`CreateEvaluationOutput`](crate::operation::create_evaluation::CreateEvaluationOutput) with field(s):
    ///   - [`evaluation_id(Option<String>)`](crate::operation::create_evaluation::CreateEvaluationOutput::evaluation_id): <p>The user-supplied ID that uniquely identifies the <code>Evaluation</code>. This value should be identical to the value of the <code>EvaluationId</code> in the request.</p>
    /// - On failure, responds with [`SdkError<CreateEvaluationError>`](crate::operation::create_evaluation::CreateEvaluationError)
    pub fn create_evaluation(
        &self,
    ) -> crate::operation::create_evaluation::builders::CreateEvaluationFluentBuilder {
        crate::operation::create_evaluation::builders::CreateEvaluationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
