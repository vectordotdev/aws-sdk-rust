// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListModels`](crate::operation::list_models::builders::ListModelsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_models::builders::ListModelsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`project_name(impl ::std::convert::Into<String>)`](crate::operation::list_models::builders::ListModelsFluentBuilder::project_name) / [`set_project_name(Option<String>)`](crate::operation::list_models::builders::ListModelsFluentBuilder::set_project_name): <p>The name of the project that contains the model versions that you want to list.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_models::builders::ListModelsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_models::builders::ListModelsFluentBuilder::set_next_token): <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Lookout for Vision returns a pagination token in the response. You can use this pagination token to retrieve the next set of models.</p>
    ///   - [`max_results(i32)`](crate::operation::list_models::builders::ListModelsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_models::builders::ListModelsFluentBuilder::set_max_results): <p>The maximum number of results to return per paginated call. The largest value you can specify is 100. If you specify a value greater than 100, a ValidationException error occurs. The default value is 100.</p>
    /// - On success, responds with [`ListModelsOutput`](crate::operation::list_models::ListModelsOutput) with field(s):
    ///   - [`models(Option<Vec<ModelMetadata>>)`](crate::operation::list_models::ListModelsOutput::models): <p>A list of model versions in the specified project. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_models::ListModelsOutput::next_token): <p>If the response is truncated, Amazon Lookout for Vision returns this token that you can use in the subsequent request to retrieve the next set of models. </p>
    /// - On failure, responds with [`SdkError<ListModelsError>`](crate::operation::list_models::ListModelsError)
    pub fn list_models(&self) -> crate::operation::list_models::builders::ListModelsFluentBuilder {
        crate::operation::list_models::builders::ListModelsFluentBuilder::new(self.handle.clone())
    }
}
