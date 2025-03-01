// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_maintenance_windows_for_target::_describe_maintenance_windows_for_target_output::DescribeMaintenanceWindowsForTargetOutputBuilder;

pub use crate::operation::describe_maintenance_windows_for_target::_describe_maintenance_windows_for_target_input::DescribeMaintenanceWindowsForTargetInputBuilder;

/// Fluent builder constructing a request to `DescribeMaintenanceWindowsForTarget`.
///
/// <p>Retrieves information about the maintenance window targets or tasks that a managed node is associated with.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeMaintenanceWindowsForTargetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_maintenance_windows_for_target::builders::DescribeMaintenanceWindowsForTargetInputBuilder,
}
impl DescribeMaintenanceWindowsForTargetFluentBuilder {
    /// Creates a new `DescribeMaintenanceWindowsForTarget`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTarget, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetOutput, ::aws_smithy_http::result::SdkError<crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetOutput, ::aws_smithy_http::result::SdkError<crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTarget, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetError>
    >{
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_maintenance_windows_for_target::paginator::DescribeMaintenanceWindowsForTargetPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_maintenance_windows_for_target::paginator::DescribeMaintenanceWindowsForTargetPaginator{
        crate::operation::describe_maintenance_windows_for_target::paginator::DescribeMaintenanceWindowsForTargetPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `Targets`.
    ///
    /// To override the contents of this collection use [`set_targets`](Self::set_targets).
    ///
    /// <p>The managed node ID or key-value pair to retrieve information about.</p>
    pub fn targets(mut self, input: crate::types::Target) -> Self {
        self.inner = self.inner.targets(input);
        self
    }
    /// <p>The managed node ID or key-value pair to retrieve information about.</p>
    pub fn set_targets(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Target>>,
    ) -> Self {
        self.inner = self.inner.set_targets(input);
        self
    }
    /// <p>The type of resource you want to retrieve information about. For example, <code>INSTANCE</code>.</p>
    pub fn resource_type(mut self, input: crate::types::MaintenanceWindowResourceType) -> Self {
        self.inner = self.inner.resource_type(input);
        self
    }
    /// <p>The type of resource you want to retrieve information about. For example, <code>INSTANCE</code>.</p>
    pub fn set_resource_type(
        mut self,
        input: ::std::option::Option<crate::types::MaintenanceWindowResourceType>,
    ) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
