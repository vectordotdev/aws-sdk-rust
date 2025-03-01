// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_prepared_statement::_create_prepared_statement_output::CreatePreparedStatementOutputBuilder;

pub use crate::operation::create_prepared_statement::_create_prepared_statement_input::CreatePreparedStatementInputBuilder;

/// Fluent builder constructing a request to `CreatePreparedStatement`.
///
/// <p>Creates a prepared statement for use with SQL queries in Athena.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreatePreparedStatementFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::create_prepared_statement::builders::CreatePreparedStatementInputBuilder,
}
impl CreatePreparedStatementFluentBuilder {
    /// Creates a new `CreatePreparedStatement`.
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
            crate::operation::create_prepared_statement::CreatePreparedStatement,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_prepared_statement::CreatePreparedStatementError,
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
        crate::operation::create_prepared_statement::CreatePreparedStatementOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_prepared_statement::CreatePreparedStatementError,
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
        crate::operation::create_prepared_statement::CreatePreparedStatementOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_prepared_statement::CreatePreparedStatementError,
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
            crate::operation::create_prepared_statement::CreatePreparedStatement,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_prepared_statement::CreatePreparedStatementError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the prepared statement.</p>
    pub fn statement_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.statement_name(input.into());
        self
    }
    /// <p>The name of the prepared statement.</p>
    pub fn set_statement_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_statement_name(input);
        self
    }
    /// <p>The name of the workgroup to which the prepared statement belongs.</p>
    pub fn work_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.work_group(input.into());
        self
    }
    /// <p>The name of the workgroup to which the prepared statement belongs.</p>
    pub fn set_work_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_work_group(input);
        self
    }
    /// <p>The query string for the prepared statement.</p>
    pub fn query_statement(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.query_statement(input.into());
        self
    }
    /// <p>The query string for the prepared statement.</p>
    pub fn set_query_statement(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_query_statement(input);
        self
    }
    /// <p>The description of the prepared statement.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the prepared statement.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
}
