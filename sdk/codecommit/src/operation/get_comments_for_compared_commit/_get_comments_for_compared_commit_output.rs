// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetCommentsForComparedCommitOutput {
    /// <p>A list of comment objects on the compared commit.</p>
    #[doc(hidden)]
    pub comments_for_compared_commit_data:
        ::std::option::Option<::std::vec::Vec<crate::types::CommentsForComparedCommit>>,
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetCommentsForComparedCommitOutput {
    /// <p>A list of comment objects on the compared commit.</p>
    pub fn comments_for_compared_commit_data(
        &self,
    ) -> ::std::option::Option<&[crate::types::CommentsForComparedCommit]> {
        self.comments_for_compared_commit_data.as_deref()
    }
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetCommentsForComparedCommitOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetCommentsForComparedCommitOutput {
    /// Creates a new builder-style object to manufacture [`GetCommentsForComparedCommitOutput`](crate::operation::get_comments_for_compared_commit::GetCommentsForComparedCommitOutput).
    pub fn builder() -> crate::operation::get_comments_for_compared_commit::builders::GetCommentsForComparedCommitOutputBuilder{
        crate::operation::get_comments_for_compared_commit::builders::GetCommentsForComparedCommitOutputBuilder::default()
    }
}

/// A builder for [`GetCommentsForComparedCommitOutput`](crate::operation::get_comments_for_compared_commit::GetCommentsForComparedCommitOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetCommentsForComparedCommitOutputBuilder {
    pub(crate) comments_for_compared_commit_data:
        ::std::option::Option<::std::vec::Vec<crate::types::CommentsForComparedCommit>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetCommentsForComparedCommitOutputBuilder {
    /// Appends an item to `comments_for_compared_commit_data`.
    ///
    /// To override the contents of this collection use [`set_comments_for_compared_commit_data`](Self::set_comments_for_compared_commit_data).
    ///
    /// <p>A list of comment objects on the compared commit.</p>
    pub fn comments_for_compared_commit_data(
        mut self,
        input: crate::types::CommentsForComparedCommit,
    ) -> Self {
        let mut v = self.comments_for_compared_commit_data.unwrap_or_default();
        v.push(input);
        self.comments_for_compared_commit_data = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of comment objects on the compared commit.</p>
    pub fn set_comments_for_compared_commit_data(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CommentsForComparedCommit>>,
    ) -> Self {
        self.comments_for_compared_commit_data = input;
        self
    }
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An enumeration token that can be used in a request to return the next batch of the results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetCommentsForComparedCommitOutput`](crate::operation::get_comments_for_compared_commit::GetCommentsForComparedCommitOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_comments_for_compared_commit::GetCommentsForComparedCommitOutput
    {
        crate::operation::get_comments_for_compared_commit::GetCommentsForComparedCommitOutput {
            comments_for_compared_commit_data: self.comments_for_compared_commit_data,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
