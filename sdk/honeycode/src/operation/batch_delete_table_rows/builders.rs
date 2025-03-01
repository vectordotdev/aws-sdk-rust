// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_delete_table_rows::_batch_delete_table_rows_output::BatchDeleteTableRowsOutputBuilder;

pub use crate::operation::batch_delete_table_rows::_batch_delete_table_rows_input::BatchDeleteTableRowsInputBuilder;

/// Fluent builder constructing a request to `BatchDeleteTableRows`.
///
/// <p> The BatchDeleteTableRows API allows you to delete one or more rows from a table in a workbook. You need to specify the ids of the rows that you want to delete from the table. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchDeleteTableRowsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_delete_table_rows::builders::BatchDeleteTableRowsInputBuilder,
}
impl BatchDeleteTableRowsFluentBuilder {
    /// Creates a new `BatchDeleteTableRows`.
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
            crate::operation::batch_delete_table_rows::BatchDeleteTableRows,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_delete_table_rows::BatchDeleteTableRowsError,
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
        crate::operation::batch_delete_table_rows::BatchDeleteTableRowsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_delete_table_rows::BatchDeleteTableRowsError,
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
        crate::operation::batch_delete_table_rows::BatchDeleteTableRowsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_delete_table_rows::BatchDeleteTableRowsError,
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
            crate::operation::batch_delete_table_rows::BatchDeleteTableRows,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::batch_delete_table_rows::BatchDeleteTableRowsError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the workbook where the rows are being deleted.</p>
    /// <p> If a workbook with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn workbook_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workbook_id(input.into());
        self
    }
    /// <p>The ID of the workbook where the rows are being deleted.</p>
    /// <p> If a workbook with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn set_workbook_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workbook_id(input);
        self
    }
    /// <p>The ID of the table where the rows are being deleted.</p>
    /// <p> If a table with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_id(input.into());
        self
    }
    /// <p>The ID of the table where the rows are being deleted.</p>
    /// <p> If a table with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn set_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_id(input);
        self
    }
    /// Appends an item to `rowIds`.
    ///
    /// To override the contents of this collection use [`set_row_ids`](Self::set_row_ids).
    ///
    /// <p> The list of row ids to delete from the table. You need to specify at least one row id in this list. </p>
    /// <p> Note that if one of the row ids provided in the request does not exist in the table, then the request fails and no rows are deleted from the table. </p>
    pub fn row_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.row_ids(input.into());
        self
    }
    /// <p> The list of row ids to delete from the table. You need to specify at least one row id in this list. </p>
    /// <p> Note that if one of the row ids provided in the request does not exist in the table, then the request fails and no rows are deleted from the table. </p>
    pub fn set_row_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_row_ids(input);
        self
    }
    /// <p> The request token for performing the delete action. Request tokens help to identify duplicate requests. If a call times out or fails due to a transient error like a failed network connection, you can retry the call with the same request token. The service ensures that if the first call using that request token is successfully performed, the second call will not perform the action again. </p>
    /// <p> Note that request tokens are valid only for a few minutes. You cannot use request tokens to dedupe requests spanning hours or days. </p>
    pub fn client_request_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p> The request token for performing the delete action. Request tokens help to identify duplicate requests. If a call times out or fails due to a transient error like a failed network connection, you can retry the call with the same request token. The service ensures that if the first call using that request token is successfully performed, the second call will not perform the action again. </p>
    /// <p> Note that request tokens are valid only for a few minutes. You cannot use request tokens to dedupe requests spanning hours or days. </p>
    pub fn set_client_request_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
}
