// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_blue_green_deployments::_describe_blue_green_deployments_output::DescribeBlueGreenDeploymentsOutputBuilder;

pub use crate::operation::describe_blue_green_deployments::_describe_blue_green_deployments_input::DescribeBlueGreenDeploymentsInputBuilder;

/// Fluent builder constructing a request to `DescribeBlueGreenDeployments`.
///
/// <p>Returns information about blue/green deployments.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/blue-green-deployments.html">Using Amazon RDS Blue/Green Deployments for database updates</a> in the <i>Amazon RDS User Guide</i> and <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/blue-green-deployments.html"> Using Amazon RDS Blue/Green Deployments for database updates</a> in the <i>Amazon Aurora User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeBlueGreenDeploymentsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_blue_green_deployments::builders::DescribeBlueGreenDeploymentsInputBuilder,
}
impl DescribeBlueGreenDeploymentsFluentBuilder {
    /// Creates a new `DescribeBlueGreenDeployments`.
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
            crate::operation::describe_blue_green_deployments::DescribeBlueGreenDeployments,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_blue_green_deployments::DescribeBlueGreenDeploymentsError,
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
        crate::operation::describe_blue_green_deployments::DescribeBlueGreenDeploymentsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_blue_green_deployments::DescribeBlueGreenDeploymentsError,
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
        crate::operation::describe_blue_green_deployments::DescribeBlueGreenDeploymentsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_blue_green_deployments::DescribeBlueGreenDeploymentsError,
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
            crate::operation::describe_blue_green_deployments::DescribeBlueGreenDeployments,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_blue_green_deployments::DescribeBlueGreenDeploymentsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_blue_green_deployments::paginator::DescribeBlueGreenDeploymentsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_blue_green_deployments::paginator::DescribeBlueGreenDeploymentsPaginator{
        crate::operation::describe_blue_green_deployments::paginator::DescribeBlueGreenDeploymentsPaginator::new(self.handle, self.inner)
    }
    /// <p>The blue/green deployment identifier. If this parameter is specified, information from only the specific blue/green deployment is returned. This parameter isn't case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>If supplied, must match an existing blue/green deployment identifier.</p> </li>
    /// </ul>
    pub fn blue_green_deployment_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.blue_green_deployment_identifier(input.into());
        self
    }
    /// <p>The blue/green deployment identifier. If this parameter is specified, information from only the specific blue/green deployment is returned. This parameter isn't case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>If supplied, must match an existing blue/green deployment identifier.</p> </li>
    /// </ul>
    pub fn set_blue_green_deployment_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_blue_green_deployment_identifier(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>A filter that specifies one or more blue/green deployments to describe.</p>
    /// <p>Supported filters:</p>
    /// <ul>
    /// <li> <p> <code>blue-green-deployment-identifier</code> - Accepts system-generated identifiers for blue/green deployments. The results list only includes information about the blue/green deployments with the specified identifiers.</p> </li>
    /// <li> <p> <code>blue-green-deployment-name</code> - Accepts user-supplied names for blue/green deployments. The results list only includes information about the blue/green deployments with the specified names.</p> </li>
    /// <li> <p> <code>source</code> - Accepts source databases for a blue/green deployment. The results list only includes information about the blue/green deployments with the specified source databases.</p> </li>
    /// <li> <p> <code>target</code> - Accepts target databases for a blue/green deployment. The results list only includes information about the blue/green deployments with the specified target databases.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>A filter that specifies one or more blue/green deployments to describe.</p>
    /// <p>Supported filters:</p>
    /// <ul>
    /// <li> <p> <code>blue-green-deployment-identifier</code> - Accepts system-generated identifiers for blue/green deployments. The results list only includes information about the blue/green deployments with the specified identifiers.</p> </li>
    /// <li> <p> <code>blue-green-deployment-name</code> - Accepts user-supplied names for blue/green deployments. The results list only includes information about the blue/green deployments with the specified names.</p> </li>
    /// <li> <p> <code>source</code> - Accepts source databases for a blue/green deployment. The results list only includes information about the blue/green deployments with the specified source databases.</p> </li>
    /// <li> <p> <code>target</code> - Accepts target databases for a blue/green deployment. The results list only includes information about the blue/green deployments with the specified target databases.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>An optional pagination token provided by a previous <code>DescribeBlueGreenDeployments</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>An optional pagination token provided by a previous <code>DescribeBlueGreenDeployments</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so you can retrieve the remaining results.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so you can retrieve the remaining results.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn set_max_records(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
}
