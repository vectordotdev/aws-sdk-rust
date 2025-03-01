// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBatchPrediction`](crate::operation::delete_batch_prediction::builders::DeleteBatchPredictionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`batch_prediction_id(impl ::std::convert::Into<String>)`](crate::operation::delete_batch_prediction::builders::DeleteBatchPredictionFluentBuilder::batch_prediction_id) / [`set_batch_prediction_id(Option<String>)`](crate::operation::delete_batch_prediction::builders::DeleteBatchPredictionFluentBuilder::set_batch_prediction_id): <p>A user-supplied ID that uniquely identifies the <code>BatchPrediction</code>.</p>
    /// - On success, responds with [`DeleteBatchPredictionOutput`](crate::operation::delete_batch_prediction::DeleteBatchPredictionOutput) with field(s):
    ///   - [`batch_prediction_id(Option<String>)`](crate::operation::delete_batch_prediction::DeleteBatchPredictionOutput::batch_prediction_id): <p>A user-supplied ID that uniquely identifies the <code>BatchPrediction</code>. This value should be identical to the value of the <code>BatchPredictionID</code> in the request.</p>
    /// - On failure, responds with [`SdkError<DeleteBatchPredictionError>`](crate::operation::delete_batch_prediction::DeleteBatchPredictionError)
    pub fn delete_batch_prediction(
        &self,
    ) -> crate::operation::delete_batch_prediction::builders::DeleteBatchPredictionFluentBuilder
    {
        crate::operation::delete_batch_prediction::builders::DeleteBatchPredictionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
