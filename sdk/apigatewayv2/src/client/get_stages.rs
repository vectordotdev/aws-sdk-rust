// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetStages`](crate::operation::get_stages::builders::GetStagesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`api_id(impl ::std::convert::Into<String>)`](crate::operation::get_stages::builders::GetStagesFluentBuilder::api_id) / [`set_api_id(Option<String>)`](crate::operation::get_stages::builders::GetStagesFluentBuilder::set_api_id): <p>The API identifier.</p>
    ///   - [`max_results(impl ::std::convert::Into<String>)`](crate::operation::get_stages::builders::GetStagesFluentBuilder::max_results) / [`set_max_results(Option<String>)`](crate::operation::get_stages::builders::GetStagesFluentBuilder::set_max_results): <p>The maximum number of elements to be returned for this resource.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::get_stages::builders::GetStagesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_stages::builders::GetStagesFluentBuilder::set_next_token): <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    /// - On success, responds with [`GetStagesOutput`](crate::operation::get_stages::GetStagesOutput) with field(s):
    ///   - [`items(Option<Vec<Stage>>)`](crate::operation::get_stages::GetStagesOutput::items): <p>The elements from this collection.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_stages::GetStagesOutput::next_token): <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    /// - On failure, responds with [`SdkError<GetStagesError>`](crate::operation::get_stages::GetStagesError)
    pub fn get_stages(&self) -> crate::operation::get_stages::builders::GetStagesFluentBuilder {
        crate::operation::get_stages::builders::GetStagesFluentBuilder::new(self.handle.clone())
    }
}
