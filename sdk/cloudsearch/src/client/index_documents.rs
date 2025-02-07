// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`IndexDocuments`](crate::operation::index_documents::builders::IndexDocumentsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::index_documents::builders::IndexDocumentsFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::index_documents::builders::IndexDocumentsFluentBuilder::set_domain_name): <p>A string that represents the name of a domain. Domain names are unique across the domains owned by an account within an AWS region. Domain names start with a letter or number and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    /// - On success, responds with [`IndexDocumentsOutput`](crate::operation::index_documents::IndexDocumentsOutput) with field(s):
    ///   - [`field_names(Option<Vec<String>>)`](crate::operation::index_documents::IndexDocumentsOutput::field_names): <p>The names of the fields that are currently being indexed.</p>
    /// - On failure, responds with [`SdkError<IndexDocumentsError>`](crate::operation::index_documents::IndexDocumentsError)
    pub fn index_documents(
        &self,
    ) -> crate::operation::index_documents::builders::IndexDocumentsFluentBuilder {
        crate::operation::index_documents::builders::IndexDocumentsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
