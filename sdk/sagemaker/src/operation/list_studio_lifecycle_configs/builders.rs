// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_studio_lifecycle_configs::_list_studio_lifecycle_configs_output::ListStudioLifecycleConfigsOutputBuilder;

pub use crate::operation::list_studio_lifecycle_configs::_list_studio_lifecycle_configs_input::ListStudioLifecycleConfigsInputBuilder;

/// Fluent builder constructing a request to `ListStudioLifecycleConfigs`.
///
/// <p>Lists the Studio Lifecycle Configurations in your Amazon Web Services Account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListStudioLifecycleConfigsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::list_studio_lifecycle_configs::builders::ListStudioLifecycleConfigsInputBuilder,
}
impl ListStudioLifecycleConfigsFluentBuilder {
    /// Creates a new `ListStudioLifecycleConfigs`.
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
            crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigs,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsError,
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
        crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsError,
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
        crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsError,
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
            crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigs,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_studio_lifecycle_configs::ListStudioLifecycleConfigsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_studio_lifecycle_configs::paginator::ListStudioLifecycleConfigsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_studio_lifecycle_configs::paginator::ListStudioLifecycleConfigsPaginator{
        crate::operation::list_studio_lifecycle_configs::paginator::ListStudioLifecycleConfigsPaginator::new(self.handle, self.inner)
    }
    /// <p>The maximum number of Studio Lifecycle Configurations to return in the response. The default value is 10.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of Studio Lifecycle Configurations to return in the response. The default value is 10.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>If the previous call to ListStudioLifecycleConfigs didn't return the full set of Lifecycle Configurations, the call returns a token for getting the next set of Lifecycle Configurations.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the previous call to ListStudioLifecycleConfigs didn't return the full set of Lifecycle Configurations, the call returns a token for getting the next set of Lifecycle Configurations.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A string in the Lifecycle Configuration name. This filter returns only Lifecycle Configurations whose name contains the specified string.</p>
    pub fn name_contains(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.name_contains(input.into());
        self
    }
    /// <p>A string in the Lifecycle Configuration name. This filter returns only Lifecycle Configurations whose name contains the specified string.</p>
    pub fn set_name_contains(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_name_contains(input);
        self
    }
    /// <p>A parameter to search for the App Type to which the Lifecycle Configuration is attached.</p>
    pub fn app_type_equals(mut self, input: crate::types::StudioLifecycleConfigAppType) -> Self {
        self.inner = self.inner.app_type_equals(input);
        self
    }
    /// <p>A parameter to search for the App Type to which the Lifecycle Configuration is attached.</p>
    pub fn set_app_type_equals(
        mut self,
        input: ::std::option::Option<crate::types::StudioLifecycleConfigAppType>,
    ) -> Self {
        self.inner = self.inner.set_app_type_equals(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations created on or before the specified time.</p>
    pub fn creation_time_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_before(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations created on or before the specified time.</p>
    pub fn set_creation_time_before(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_creation_time_before(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations created on or after the specified time.</p>
    pub fn creation_time_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_after(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations created on or after the specified time.</p>
    pub fn set_creation_time_after(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_creation_time_after(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations modified before the specified time.</p>
    pub fn modified_time_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.modified_time_before(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations modified before the specified time.</p>
    pub fn set_modified_time_before(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_modified_time_before(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations modified after the specified time.</p>
    pub fn modified_time_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.modified_time_after(input);
        self
    }
    /// <p>A filter that returns only Lifecycle Configurations modified after the specified time.</p>
    pub fn set_modified_time_after(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_modified_time_after(input);
        self
    }
    /// <p>The property used to sort results. The default value is CreationTime.</p>
    pub fn sort_by(mut self, input: crate::types::StudioLifecycleConfigSortKey) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>The property used to sort results. The default value is CreationTime.</p>
    pub fn set_sort_by(
        mut self,
        input: ::std::option::Option<crate::types::StudioLifecycleConfigSortKey>,
    ) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>The sort order. The default value is Descending.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrder) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>The sort order. The default value is Descending.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::SortOrder>) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
}
