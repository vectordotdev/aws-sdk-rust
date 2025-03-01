// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_model_package_groups::_list_model_package_groups_output::ListModelPackageGroupsOutputBuilder;

pub use crate::operation::list_model_package_groups::_list_model_package_groups_input::ListModelPackageGroupsInputBuilder;

/// Fluent builder constructing a request to `ListModelPackageGroups`.
///
/// <p>Gets a list of the model groups in your Amazon Web Services account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListModelPackageGroupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::list_model_package_groups::builders::ListModelPackageGroupsInputBuilder,
}
impl ListModelPackageGroupsFluentBuilder {
    /// Creates a new `ListModelPackageGroups`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_model_package_groups::ListModelPackageGroups,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_model_package_groups::ListModelPackageGroupsError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_model_package_groups::ListModelPackageGroupsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_model_package_groups::ListModelPackageGroupsError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_model_package_groups::ListModelPackageGroupsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_model_package_groups::ListModelPackageGroupsError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_model_package_groups::ListModelPackageGroups,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_model_package_groups::ListModelPackageGroupsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_model_package_groups::paginator::ListModelPackageGroupsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_model_package_groups::paginator::ListModelPackageGroupsPaginator
    {
        crate::operation::list_model_package_groups::paginator::ListModelPackageGroupsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>A filter that returns only model groups created after the specified time.</p>
    pub fn creation_time_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_after(input);
        self
    }
    /// <p>A filter that returns only model groups created after the specified time.</p>
    pub fn set_creation_time_after(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_creation_time_after(input);
        self
    }
    /// <p>A filter that returns only model groups created before the specified time.</p>
    pub fn creation_time_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_before(input);
        self
    }
    /// <p>A filter that returns only model groups created before the specified time.</p>
    pub fn set_creation_time_before(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_creation_time_before(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>A string in the model group name. This filter returns only model groups whose name contains the specified string.</p>
    pub fn name_contains(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.name_contains(input.into());
        self
    }
    /// <p>A string in the model group name. This filter returns only model groups whose name contains the specified string.</p>
    pub fn set_name_contains(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_name_contains(input);
        self
    }
    /// <p>If the result of the previous <code>ListModelPackageGroups</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of model groups, use the token in the next request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the result of the previous <code>ListModelPackageGroups</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of model groups, use the token in the next request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The field to sort results by. The default is <code>CreationTime</code>.</p>
    pub fn sort_by(mut self, input: crate::types::ModelPackageGroupSortBy) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>The field to sort results by. The default is <code>CreationTime</code>.</p>
    pub fn set_sort_by(
        mut self,
        input: ::std::option::Option<crate::types::ModelPackageGroupSortBy>,
    ) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>The sort order for results. The default is <code>Ascending</code>.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrder) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>The sort order for results. The default is <code>Ascending</code>.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::SortOrder>) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
}
