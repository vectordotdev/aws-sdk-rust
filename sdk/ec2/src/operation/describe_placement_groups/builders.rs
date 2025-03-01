// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_placement_groups::_describe_placement_groups_output::DescribePlacementGroupsOutputBuilder;

pub use crate::operation::describe_placement_groups::_describe_placement_groups_input::DescribePlacementGroupsInputBuilder;

/// Fluent builder constructing a request to `DescribePlacementGroups`.
///
/// <p>Describes the specified placement groups or all of your placement groups. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html">Placement groups</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribePlacementGroupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::describe_placement_groups::builders::DescribePlacementGroupsInputBuilder,
}
impl DescribePlacementGroupsFluentBuilder {
    /// Creates a new `DescribePlacementGroups`.
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
            crate::operation::describe_placement_groups::DescribePlacementGroups,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_placement_groups::DescribePlacementGroupsError,
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
        crate::operation::describe_placement_groups::DescribePlacementGroupsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_placement_groups::DescribePlacementGroupsError,
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
        crate::operation::describe_placement_groups::DescribePlacementGroupsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_placement_groups::DescribePlacementGroupsError,
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
            crate::operation::describe_placement_groups::DescribePlacementGroups,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_placement_groups::DescribePlacementGroupsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>group-name</code> - The name of the placement group.</p> </li>
    /// <li> <p> <code>group-arn</code> - The Amazon Resource Name (ARN) of the placement group.</p> </li>
    /// <li> <p> <code>spread-level</code> - The spread level for the placement group (<code>host</code> | <code>rack</code>). </p> </li>
    /// <li> <p> <code>state</code> - The state of the placement group (<code>pending</code> | <code>available</code> | <code>deleting</code> | <code>deleted</code>).</p> </li>
    /// <li> <p> <code>strategy</code> - The strategy of the placement group (<code>cluster</code> | <code>spread</code> | <code>partition</code>).</p> </li>
    /// <li> <p> <code>tag:
    /// <key></key></code> - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources that have a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>group-name</code> - The name of the placement group.</p> </li>
    /// <li> <p> <code>group-arn</code> - The Amazon Resource Name (ARN) of the placement group.</p> </li>
    /// <li> <p> <code>spread-level</code> - The spread level for the placement group (<code>host</code> | <code>rack</code>). </p> </li>
    /// <li> <p> <code>state</code> - The state of the placement group (<code>pending</code> | <code>available</code> | <code>deleting</code> | <code>deleted</code>).</p> </li>
    /// <li> <p> <code>strategy</code> - The strategy of the placement group (<code>cluster</code> | <code>spread</code> | <code>partition</code>).</p> </li>
    /// <li> <p> <code>tag:
    /// <key></key></code> - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources that have a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// Appends an item to `GroupNames`.
    ///
    /// To override the contents of this collection use [`set_group_names`](Self::set_group_names).
    ///
    /// <p>The names of the placement groups.</p>
    /// <p>Default: Describes all your placement groups, or only those otherwise specified.</p>
    pub fn group_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_names(input.into());
        self
    }
    /// <p>The names of the placement groups.</p>
    /// <p>Default: Describes all your placement groups, or only those otherwise specified.</p>
    pub fn set_group_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_group_names(input);
        self
    }
    /// Appends an item to `GroupIds`.
    ///
    /// To override the contents of this collection use [`set_group_ids`](Self::set_group_ids).
    ///
    /// <p>The IDs of the placement groups.</p>
    pub fn group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_ids(input.into());
        self
    }
    /// <p>The IDs of the placement groups.</p>
    pub fn set_group_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_group_ids(input);
        self
    }
}
