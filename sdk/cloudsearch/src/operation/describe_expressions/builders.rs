// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_expressions::_describe_expressions_output::DescribeExpressionsOutputBuilder;

pub use crate::operation::describe_expressions::_describe_expressions_input::DescribeExpressionsInputBuilder;

/// Fluent builder constructing a request to `DescribeExpressions`.
///
/// <p>Gets the expressions configured for the search domain. Can be limited to specific expressions by name. By default, shows all expressions and includes any pending changes to the configuration. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-expressions.html" target="_blank">Configuring Expressions</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeExpressionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_expressions::builders::DescribeExpressionsInputBuilder,
}
impl DescribeExpressionsFluentBuilder {
    /// Creates a new `DescribeExpressions`.
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
            crate::operation::describe_expressions::DescribeExpressions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_expressions::DescribeExpressionsError,
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
        crate::operation::describe_expressions::DescribeExpressionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_expressions::DescribeExpressionsError,
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
        crate::operation::describe_expressions::DescribeExpressionsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_expressions::DescribeExpressionsError,
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
            crate::operation::describe_expressions::DescribeExpressions,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_expressions::DescribeExpressionsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the domain you want to describe.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of the domain you want to describe.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// Appends an item to `ExpressionNames`.
    ///
    /// To override the contents of this collection use [`set_expression_names`](Self::set_expression_names).
    ///
    /// <p>Limits the <code><code>DescribeExpressions</code></code> response to the specified expressions. If not specified, all expressions are shown.</p>
    pub fn expression_names(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.expression_names(input.into());
        self
    }
    /// <p>Limits the <code><code>DescribeExpressions</code></code> response to the specified expressions. If not specified, all expressions are shown.</p>
    pub fn set_expression_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_expression_names(input);
        self
    }
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub fn deployed(mut self, input: bool) -> Self {
        self.inner = self.inner.deployed(input);
        self
    }
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub fn set_deployed(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_deployed(input);
        self
    }
}
