// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetComment`](crate::operation::get_comment::builders::GetCommentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`comment_id(impl ::std::convert::Into<String>)`](crate::operation::get_comment::builders::GetCommentFluentBuilder::comment_id) / [`set_comment_id(Option<String>)`](crate::operation::get_comment::builders::GetCommentFluentBuilder::set_comment_id): <p>The unique, system-generated ID of the comment. To get this ID, use <code>GetCommentsForComparedCommit</code> or <code>GetCommentsForPullRequest</code>.</p>
    /// - On success, responds with [`GetCommentOutput`](crate::operation::get_comment::GetCommentOutput) with field(s):
    ///   - [`comment(Option<Comment>)`](crate::operation::get_comment::GetCommentOutput::comment): <p>The contents of the comment.</p>
    /// - On failure, responds with [`SdkError<GetCommentError>`](crate::operation::get_comment::GetCommentError)
    pub fn get_comment(&self) -> crate::operation::get_comment::builders::GetCommentFluentBuilder {
        crate::operation::get_comment::builders::GetCommentFluentBuilder::new(self.handle.clone())
    }
}
