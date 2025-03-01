// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_backups::_list_backups_output::ListBackupsOutputBuilder;

pub use crate::operation::list_backups::_list_backups_input::ListBackupsInputBuilder;

/// Fluent builder constructing a request to `ListBackups`.
///
/// <p>List backups associated with an Amazon Web Services account. To list backups for a given table, specify <code>TableName</code>. <code>ListBackups</code> returns a paginated list of results with at most 1 MB worth of items in a page. You can also specify a maximum number of entries to be returned in a page.</p>
/// <p>In the request, start time is inclusive, but end time is exclusive. Note that these boundaries are for the time at which the original backup was requested.</p>
/// <p>You can call <code>ListBackups</code> a maximum of five times per second.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListBackupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_backups::builders::ListBackupsInputBuilder,
}
impl ListBackupsFluentBuilder {
    /// Creates a new `ListBackups`.
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
            crate::operation::list_backups::ListBackups,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_backups::ListBackupsError>,
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
        crate::operation::list_backups::ListBackupsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_backups::ListBackupsError>,
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
        crate::operation::list_backups::ListBackupsOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::list_backups::ListBackupsError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_backups::ListBackups,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_backups::ListBackupsError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The backups from the table specified by <code>TableName</code> are listed. </p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The backups from the table specified by <code>TableName</code> are listed. </p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>Maximum number of backups to return at once.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>Maximum number of backups to return at once.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>Only backups created after this time are listed. <code>TimeRangeLowerBound</code> is inclusive.</p>
    pub fn time_range_lower_bound(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.time_range_lower_bound(input);
        self
    }
    /// <p>Only backups created after this time are listed. <code>TimeRangeLowerBound</code> is inclusive.</p>
    pub fn set_time_range_lower_bound(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_time_range_lower_bound(input);
        self
    }
    /// <p>Only backups created before this time are listed. <code>TimeRangeUpperBound</code> is exclusive. </p>
    pub fn time_range_upper_bound(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.time_range_upper_bound(input);
        self
    }
    /// <p>Only backups created before this time are listed. <code>TimeRangeUpperBound</code> is exclusive. </p>
    pub fn set_time_range_upper_bound(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_time_range_upper_bound(input);
        self
    }
    /// <p> <code>LastEvaluatedBackupArn</code> is the Amazon Resource Name (ARN) of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results. </p>
    pub fn exclusive_start_backup_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.exclusive_start_backup_arn(input.into());
        self
    }
    /// <p> <code>LastEvaluatedBackupArn</code> is the Amazon Resource Name (ARN) of the backup last evaluated when the current page of results was returned, inclusive of the current page of results. This value may be specified as the <code>ExclusiveStartBackupArn</code> of a new <code>ListBackups</code> operation in order to fetch the next page of results. </p>
    pub fn set_exclusive_start_backup_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_exclusive_start_backup_arn(input);
        self
    }
    /// <p>The backups from the table specified by <code>BackupType</code> are listed.</p>
    /// <p>Where <code>BackupType</code> can be:</p>
    /// <ul>
    /// <li> <p> <code>USER</code> - On-demand backup created by you. (The default setting if no other backup types are specified.)</p> </li>
    /// <li> <p> <code>SYSTEM</code> - On-demand backup automatically created by DynamoDB.</p> </li>
    /// <li> <p> <code>ALL</code> - All types of on-demand backups (USER and SYSTEM).</p> </li>
    /// </ul>
    pub fn backup_type(mut self, input: crate::types::BackupTypeFilter) -> Self {
        self.inner = self.inner.backup_type(input);
        self
    }
    /// <p>The backups from the table specified by <code>BackupType</code> are listed.</p>
    /// <p>Where <code>BackupType</code> can be:</p>
    /// <ul>
    /// <li> <p> <code>USER</code> - On-demand backup created by you. (The default setting if no other backup types are specified.)</p> </li>
    /// <li> <p> <code>SYSTEM</code> - On-demand backup automatically created by DynamoDB.</p> </li>
    /// <li> <p> <code>ALL</code> - All types of on-demand backups (USER and SYSTEM).</p> </li>
    /// </ul>
    pub fn set_backup_type(
        mut self,
        input: ::std::option::Option<crate::types::BackupTypeFilter>,
    ) -> Self {
        self.inner = self.inner.set_backup_type(input);
        self
    }
}
