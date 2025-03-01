// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteParallelData`](crate::operation::delete_parallel_data::builders::DeleteParallelDataFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::delete_parallel_data::builders::DeleteParallelDataFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_parallel_data::builders::DeleteParallelDataFluentBuilder::set_name): <p>The name of the parallel data resource that is being deleted.</p>
    /// - On success, responds with [`DeleteParallelDataOutput`](crate::operation::delete_parallel_data::DeleteParallelDataOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::operation::delete_parallel_data::DeleteParallelDataOutput::name): <p>The name of the parallel data resource that is being deleted.</p>
    ///   - [`status(Option<ParallelDataStatus>)`](crate::operation::delete_parallel_data::DeleteParallelDataOutput::status): <p>The status of the parallel data deletion.</p>
    /// - On failure, responds with [`SdkError<DeleteParallelDataError>`](crate::operation::delete_parallel_data::DeleteParallelDataError)
    pub fn delete_parallel_data(
        &self,
    ) -> crate::operation::delete_parallel_data::builders::DeleteParallelDataFluentBuilder {
        crate::operation::delete_parallel_data::builders::DeleteParallelDataFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
