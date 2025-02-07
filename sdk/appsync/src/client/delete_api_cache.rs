// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteApiCache`](crate::operation::delete_api_cache::builders::DeleteApiCacheFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`api_id(impl ::std::convert::Into<String>)`](crate::operation::delete_api_cache::builders::DeleteApiCacheFluentBuilder::api_id) / [`set_api_id(Option<String>)`](crate::operation::delete_api_cache::builders::DeleteApiCacheFluentBuilder::set_api_id): <p>The API ID.</p>
    /// - On success, responds with [`DeleteApiCacheOutput`](crate::operation::delete_api_cache::DeleteApiCacheOutput)
    /// - On failure, responds with [`SdkError<DeleteApiCacheError>`](crate::operation::delete_api_cache::DeleteApiCacheError)
    pub fn delete_api_cache(
        &self,
    ) -> crate::operation::delete_api_cache::builders::DeleteApiCacheFluentBuilder {
        crate::operation::delete_api_cache::builders::DeleteApiCacheFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
