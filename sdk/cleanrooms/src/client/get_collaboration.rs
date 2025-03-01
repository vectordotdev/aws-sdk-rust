// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetCollaboration`](crate::operation::get_collaboration::builders::GetCollaborationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`collaboration_identifier(impl ::std::convert::Into<String>)`](crate::operation::get_collaboration::builders::GetCollaborationFluentBuilder::collaboration_identifier) / [`set_collaboration_identifier(Option<String>)`](crate::operation::get_collaboration::builders::GetCollaborationFluentBuilder::set_collaboration_identifier): <p>The identifier for the collaboration.</p>
    /// - On success, responds with [`GetCollaborationOutput`](crate::operation::get_collaboration::GetCollaborationOutput) with field(s):
    ///   - [`collaboration(Option<Collaboration>)`](crate::operation::get_collaboration::GetCollaborationOutput::collaboration): <p>The entire collaboration for this identifier.</p>
    /// - On failure, responds with [`SdkError<GetCollaborationError>`](crate::operation::get_collaboration::GetCollaborationError)
    pub fn get_collaboration(
        &self,
    ) -> crate::operation::get_collaboration::builders::GetCollaborationFluentBuilder {
        crate::operation::get_collaboration::builders::GetCollaborationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
