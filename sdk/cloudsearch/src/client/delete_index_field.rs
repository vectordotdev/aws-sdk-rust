// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteIndexField`](crate::operation::delete_index_field::builders::DeleteIndexFieldFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::delete_index_field::builders::DeleteIndexFieldFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::delete_index_field::builders::DeleteIndexFieldFluentBuilder::set_domain_name): <p>A string that represents the name of a domain. Domain names are unique across the domains owned by an account within an AWS region. Domain names start with a letter or number and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    ///   - [`index_field_name(impl ::std::convert::Into<String>)`](crate::operation::delete_index_field::builders::DeleteIndexFieldFluentBuilder::index_field_name) / [`set_index_field_name(Option<String>)`](crate::operation::delete_index_field::builders::DeleteIndexFieldFluentBuilder::set_index_field_name): <p>The name of the index field your want to remove from the domain's indexing options.</p>
    /// - On success, responds with [`DeleteIndexFieldOutput`](crate::operation::delete_index_field::DeleteIndexFieldOutput) with field(s):
    ///   - [`index_field(Option<IndexFieldStatus>)`](crate::operation::delete_index_field::DeleteIndexFieldOutput::index_field): <p>The status of the index field being deleted.</p>
    /// - On failure, responds with [`SdkError<DeleteIndexFieldError>`](crate::operation::delete_index_field::DeleteIndexFieldError)
    pub fn delete_index_field(
        &self,
    ) -> crate::operation::delete_index_field::builders::DeleteIndexFieldFluentBuilder {
        crate::operation::delete_index_field::builders::DeleteIndexFieldFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
