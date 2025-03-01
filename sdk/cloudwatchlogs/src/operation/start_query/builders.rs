// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_query::_start_query_output::StartQueryOutputBuilder;

pub use crate::operation::start_query::_start_query_input::StartQueryInputBuilder;

/// Fluent builder constructing a request to `StartQuery`.
///
/// <p>Schedules a query of a log group using CloudWatch Logs Insights. You specify the log group and time range to query and the query string to use.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs Insights Query Syntax</a>.</p>
/// <p>Queries time out after 15 minutes of runtime. If your queries are timing out, reduce the time range being searched or partition your query into a number of queries.</p>
/// <p>If you are using CloudWatch cross-account observability, you can use this operation in a monitoring account to start a query in a linked source account. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Unified-Cross-Account.html">CloudWatch cross-account observability</a>. For a cross-account <code>StartQuery</code> operation, the query definition must be defined in the monitoring account.</p>
/// <p>You can have up to 20 concurrent CloudWatch Logs insights queries, including queries that have been added to dashboards. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartQueryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_query::builders::StartQueryInputBuilder,
}
impl StartQueryFluentBuilder {
    /// Creates a new `StartQuery`.
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
            crate::operation::start_query::StartQuery,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::start_query::StartQueryError>,
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
        crate::operation::start_query::StartQueryOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::start_query::StartQueryError>,
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
        crate::operation::start_query::StartQueryOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::start_query::StartQueryError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::start_query::StartQuery,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::start_query::StartQueryError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The log group on which to perform the query.</p> <note>
    /// <p>A <code>StartQuery</code> operation must include exactly one of the following parameters: <code>logGroupName</code>, <code>logGroupNames</code> or <code>logGroupIdentifiers</code>. </p>
    /// </note>
    pub fn log_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.log_group_name(input.into());
        self
    }
    /// <p>The log group on which to perform the query.</p> <note>
    /// <p>A <code>StartQuery</code> operation must include exactly one of the following parameters: <code>logGroupName</code>, <code>logGroupNames</code> or <code>logGroupIdentifiers</code>. </p>
    /// </note>
    pub fn set_log_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_log_group_name(input);
        self
    }
    /// Appends an item to `logGroupNames`.
    ///
    /// To override the contents of this collection use [`set_log_group_names`](Self::set_log_group_names).
    ///
    /// <p>The list of log groups to be queried. You can include up to 50 log groups.</p> <note>
    /// <p>A <code>StartQuery</code> operation must include exactly one of the following parameters: <code>logGroupName</code>, <code>logGroupNames</code> or <code>logGroupIdentifiers</code>. </p>
    /// </note>
    pub fn log_group_names(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.log_group_names(input.into());
        self
    }
    /// <p>The list of log groups to be queried. You can include up to 50 log groups.</p> <note>
    /// <p>A <code>StartQuery</code> operation must include exactly one of the following parameters: <code>logGroupName</code>, <code>logGroupNames</code> or <code>logGroupIdentifiers</code>. </p>
    /// </note>
    pub fn set_log_group_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_log_group_names(input);
        self
    }
    /// Appends an item to `logGroupIdentifiers`.
    ///
    /// To override the contents of this collection use [`set_log_group_identifiers`](Self::set_log_group_identifiers).
    ///
    /// <p>The list of log groups to query. You can include up to 50 log groups.</p>
    /// <p>You can specify them by the log group name or ARN. If a log group that you're querying is in a source account and you're using a monitoring account, you must specify the ARN of the log group here. The query definition must also be defined in the monitoring account.</p>
    /// <p>If you specify an ARN, the ARN can't end with an asterisk (*).</p>
    /// <p>A <code>StartQuery</code> operation must include exactly one of the following parameters: <code>logGroupName</code>, <code>logGroupNames</code> or <code>logGroupIdentifiers</code>. </p>
    pub fn log_group_identifiers(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.log_group_identifiers(input.into());
        self
    }
    /// <p>The list of log groups to query. You can include up to 50 log groups.</p>
    /// <p>You can specify them by the log group name or ARN. If a log group that you're querying is in a source account and you're using a monitoring account, you must specify the ARN of the log group here. The query definition must also be defined in the monitoring account.</p>
    /// <p>If you specify an ARN, the ARN can't end with an asterisk (*).</p>
    /// <p>A <code>StartQuery</code> operation must include exactly one of the following parameters: <code>logGroupName</code>, <code>logGroupNames</code> or <code>logGroupIdentifiers</code>. </p>
    pub fn set_log_group_identifiers(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_log_group_identifiers(input);
        self
    }
    /// <p>The beginning of the time range to query. The range is inclusive, so the specified start time is included in the query. Specified as epoch time, the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn start_time(mut self, input: i64) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The beginning of the time range to query. The range is inclusive, so the specified start time is included in the query. Specified as epoch time, the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The end of the time range to query. The range is inclusive, so the specified end time is included in the query. Specified as epoch time, the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn end_time(mut self, input: i64) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The end of the time range to query. The range is inclusive, so the specified end time is included in the query. Specified as epoch time, the number of seconds since <code>January 1, 1970, 00:00:00 UTC</code>.</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The query string to use. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs Insights Query Syntax</a>.</p>
    pub fn query_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.query_string(input.into());
        self
    }
    /// <p>The query string to use. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html">CloudWatch Logs Insights Query Syntax</a>.</p>
    pub fn set_query_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_query_string(input);
        self
    }
    /// <p>The maximum number of log events to return in the query. If the query string uses the <code>fields</code> command, only the specified fields and their values are returned. The default is 1000.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of log events to return in the query. If the query string uses the <code>fields</code> command, only the specified fields and their values are returned. The default is 1000.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
}
