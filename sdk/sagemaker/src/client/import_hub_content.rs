// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ImportHubContent`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`hub_content_name(impl ::std::convert::Into<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::hub_content_name) / [`set_hub_content_name(Option<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::set_hub_content_name): <p>The name of the hub content to import.</p>
    ///   - [`hub_content_version(impl ::std::convert::Into<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::hub_content_version) / [`set_hub_content_version(Option<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::set_hub_content_version): <p>The version of the hub content to import.</p>
    ///   - [`hub_content_type(HubContentType)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::hub_content_type) / [`set_hub_content_type(Option<HubContentType>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::set_hub_content_type): <p>The type of hub content to import.</p>
    ///   - [`document_schema_version(impl ::std::convert::Into<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::document_schema_version) / [`set_document_schema_version(Option<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::set_document_schema_version): <p>The version of the hub content schema to import.</p>
    ///   - [`hub_name(impl ::std::convert::Into<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::hub_name) / [`set_hub_name(Option<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::set_hub_name): <p>The name of the hub to import content into.</p>
    ///   - [`hub_content_display_name(impl ::std::convert::Into<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::hub_content_display_name) / [`set_hub_content_display_name(Option<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::set_hub_content_display_name): <p>The display name of the hub content to import.</p>
    ///   - [`hub_content_description(impl ::std::convert::Into<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::hub_content_description) / [`set_hub_content_description(Option<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::set_hub_content_description): <p>A description of the hub content to import.</p>
    ///   - [`hub_content_markdown(impl ::std::convert::Into<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::hub_content_markdown) / [`set_hub_content_markdown(Option<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::set_hub_content_markdown): <p>A string that provides a description of the hub content. This string can include links, tables, and standard markdown formating.</p>
    ///   - [`hub_content_document(impl ::std::convert::Into<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::hub_content_document) / [`set_hub_content_document(Option<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::set_hub_content_document): <p>The hub content document that describes information about the hub content such as type, associated containers, scripts, and more.</p>
    ///   - [`hub_content_search_keywords(Vec<String>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::hub_content_search_keywords) / [`set_hub_content_search_keywords(Option<Vec<String>>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::set_hub_content_search_keywords): <p>The searchable keywords of the hub content.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::set_tags): <p>Any tags associated with the hub content.</p>
    /// - On success, responds with [`ImportHubContentOutput`](crate::operation::import_hub_content::ImportHubContentOutput) with field(s):
    ///   - [`hub_arn(Option<String>)`](crate::operation::import_hub_content::ImportHubContentOutput::hub_arn): <p>The ARN of the hub that the content was imported into.</p>
    ///   - [`hub_content_arn(Option<String>)`](crate::operation::import_hub_content::ImportHubContentOutput::hub_content_arn): <p>The ARN of the hub content that was imported.</p>
    /// - On failure, responds with [`SdkError<ImportHubContentError>`](crate::operation::import_hub_content::ImportHubContentError)
    pub fn import_hub_content(
        &self,
    ) -> crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder {
        crate::operation::import_hub_content::builders::ImportHubContentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
