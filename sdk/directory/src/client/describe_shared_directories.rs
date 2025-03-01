// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeSharedDirectories`](crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`owner_directory_id(impl ::std::convert::Into<String>)`](crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesFluentBuilder::owner_directory_id) / [`set_owner_directory_id(Option<String>)`](crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesFluentBuilder::set_owner_directory_id): <p>Returns the identifier of the directory in the directory owner account. </p>
    ///   - [`shared_directory_ids(Vec<String>)`](crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesFluentBuilder::shared_directory_ids) / [`set_shared_directory_ids(Option<Vec<String>>)`](crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesFluentBuilder::set_shared_directory_ids): <p>A list of identifiers of all shared directories in your account. </p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesFluentBuilder::set_next_token): <p>The <code>DescribeSharedDirectoriesResult.NextToken</code> value from a previous call to <code>DescribeSharedDirectories</code>. Pass null if this is the first call. </p>
    ///   - [`limit(i32)`](crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesFluentBuilder::set_limit): <p>The number of shared directories to return in the response object.</p>
    /// - On success, responds with [`DescribeSharedDirectoriesOutput`](crate::operation::describe_shared_directories::DescribeSharedDirectoriesOutput) with field(s):
    ///   - [`shared_directories(Option<Vec<SharedDirectory>>)`](crate::operation::describe_shared_directories::DescribeSharedDirectoriesOutput::shared_directories): <p>A list of all shared directories in your account.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_shared_directories::DescribeSharedDirectoriesOutput::next_token): <p>If not null, token that indicates that more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to <code>DescribeSharedDirectories</code> to retrieve the next set of items.</p>
    /// - On failure, responds with [`SdkError<DescribeSharedDirectoriesError>`](crate::operation::describe_shared_directories::DescribeSharedDirectoriesError)
    pub fn describe_shared_directories(&self) -> crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesFluentBuilder{
        crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesFluentBuilder::new(self.handle.clone())
    }
}
