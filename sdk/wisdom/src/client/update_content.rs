// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateContent`](crate::operation::update_content::builders::UpdateContentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`knowledge_base_id(impl ::std::convert::Into<String>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::knowledge_base_id) / [`set_knowledge_base_id(Option<String>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::set_knowledge_base_id): <p>The identifier of the knowledge base. Can be either the ID or the ARN</p>
    ///   - [`content_id(impl ::std::convert::Into<String>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::content_id) / [`set_content_id(Option<String>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::set_content_id): <p>The identifier of the content. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    ///   - [`revision_id(impl ::std::convert::Into<String>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::revision_id) / [`set_revision_id(Option<String>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::set_revision_id): <p>The <code>revisionId</code> of the content resource to update, taken from an earlier call to <code>GetContent</code>, <code>GetContentSummary</code>, <code>SearchContent</code>, or <code>ListContents</code>. If included, this argument acts as an optimistic lock to ensure content was not modified since it was last read. If it has been modified, this API throws a <code>PreconditionFailedException</code>.</p>
    ///   - [`title(impl ::std::convert::Into<String>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::title) / [`set_title(Option<String>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::set_title): <p>The title of the content.</p>
    ///   - [`override_link_out_uri(impl ::std::convert::Into<String>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::override_link_out_uri) / [`set_override_link_out_uri(Option<String>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::set_override_link_out_uri): <p>The URI for the article. If the knowledge base has a templateUri, setting this argument overrides it for this piece of content. To remove an existing <code>overrideLinkOurUri</code>, exclude this argument and set <code>removeOverrideLinkOutUri</code> to true.</p>
    ///   - [`remove_override_link_out_uri(bool)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::remove_override_link_out_uri) / [`set_remove_override_link_out_uri(Option<bool>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::set_remove_override_link_out_uri): <p>Unset the existing <code>overrideLinkOutUri</code> if it exists.</p>
    ///   - [`metadata(HashMap<String, String>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::metadata) / [`set_metadata(Option<HashMap<String, String>>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::set_metadata): <p>A key/value map to store attributes without affecting tagging or recommendations. For example, when synchronizing data between an external system and Wisdom, you can store an external version identifier as metadata to utilize for determining drift.</p>
    ///   - [`upload_id(impl ::std::convert::Into<String>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::upload_id) / [`set_upload_id(Option<String>)`](crate::operation::update_content::builders::UpdateContentFluentBuilder::set_upload_id): <p>A pointer to the uploaded asset. This value is returned by <a href="https://docs.aws.amazon.com/wisdom/latest/APIReference/API_StartContentUpload.html">StartContentUpload</a>. </p>
    /// - On success, responds with [`UpdateContentOutput`](crate::operation::update_content::UpdateContentOutput) with field(s):
    ///   - [`content(Option<ContentData>)`](crate::operation::update_content::UpdateContentOutput::content): <p>The content.</p>
    /// - On failure, responds with [`SdkError<UpdateContentError>`](crate::operation::update_content::UpdateContentError)
    pub fn update_content(
        &self,
    ) -> crate::operation::update_content::builders::UpdateContentFluentBuilder {
        crate::operation::update_content::builders::UpdateContentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
