// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_notebook_execution::_start_notebook_execution_output::StartNotebookExecutionOutputBuilder;

pub use crate::operation::start_notebook_execution::_start_notebook_execution_input::StartNotebookExecutionInputBuilder;

/// Fluent builder constructing a request to `StartNotebookExecution`.
///
/// <p>Starts a notebook execution.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartNotebookExecutionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_notebook_execution::builders::StartNotebookExecutionInputBuilder,
}
impl StartNotebookExecutionFluentBuilder {
    /// Creates a new `StartNotebookExecution`.
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
            crate::operation::start_notebook_execution::StartNotebookExecution,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_notebook_execution::StartNotebookExecutionError,
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
        crate::operation::start_notebook_execution::StartNotebookExecutionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_notebook_execution::StartNotebookExecutionError,
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
        crate::operation::start_notebook_execution::StartNotebookExecutionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_notebook_execution::StartNotebookExecutionError,
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
            crate::operation::start_notebook_execution::StartNotebookExecution,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_notebook_execution::StartNotebookExecutionError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique identifier of the Amazon EMR Notebook to use for notebook execution.</p>
    pub fn editor_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.editor_id(input.into());
        self
    }
    /// <p>The unique identifier of the Amazon EMR Notebook to use for notebook execution.</p>
    pub fn set_editor_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_editor_id(input);
        self
    }
    /// <p>The path and file name of the notebook file for this execution, relative to the path specified for the Amazon EMR Notebook. For example, if you specify a path of <code>s3://MyBucket/MyNotebooks</code> when you create an Amazon EMR Notebook for a notebook with an ID of <code>e-ABCDEFGHIJK1234567890ABCD</code> (the <code>EditorID</code> of this request), and you specify a <code>RelativePath</code> of <code>my_notebook_executions/notebook_execution.ipynb</code>, the location of the file for the notebook execution is <code>s3://MyBucket/MyNotebooks/e-ABCDEFGHIJK1234567890ABCD/my_notebook_executions/notebook_execution.ipynb</code>.</p>
    pub fn relative_path(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.relative_path(input.into());
        self
    }
    /// <p>The path and file name of the notebook file for this execution, relative to the path specified for the Amazon EMR Notebook. For example, if you specify a path of <code>s3://MyBucket/MyNotebooks</code> when you create an Amazon EMR Notebook for a notebook with an ID of <code>e-ABCDEFGHIJK1234567890ABCD</code> (the <code>EditorID</code> of this request), and you specify a <code>RelativePath</code> of <code>my_notebook_executions/notebook_execution.ipynb</code>, the location of the file for the notebook execution is <code>s3://MyBucket/MyNotebooks/e-ABCDEFGHIJK1234567890ABCD/my_notebook_executions/notebook_execution.ipynb</code>.</p>
    pub fn set_relative_path(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_relative_path(input);
        self
    }
    /// <p>An optional name for the notebook execution.</p>
    pub fn notebook_execution_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.notebook_execution_name(input.into());
        self
    }
    /// <p>An optional name for the notebook execution.</p>
    pub fn set_notebook_execution_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_notebook_execution_name(input);
        self
    }
    /// <p>Input parameters in JSON format passed to the Amazon EMR Notebook at runtime for execution.</p>
    pub fn notebook_params(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.notebook_params(input.into());
        self
    }
    /// <p>Input parameters in JSON format passed to the Amazon EMR Notebook at runtime for execution.</p>
    pub fn set_notebook_params(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_notebook_params(input);
        self
    }
    /// <p>Specifies the execution engine (cluster) that runs the notebook execution.</p>
    pub fn execution_engine(mut self, input: crate::types::ExecutionEngineConfig) -> Self {
        self.inner = self.inner.execution_engine(input);
        self
    }
    /// <p>Specifies the execution engine (cluster) that runs the notebook execution.</p>
    pub fn set_execution_engine(
        mut self,
        input: ::std::option::Option<crate::types::ExecutionEngineConfig>,
    ) -> Self {
        self.inner = self.inner.set_execution_engine(input);
        self
    }
    /// <p>The name or ARN of the IAM role that is used as the service role for Amazon EMR (the Amazon EMR role) for the notebook execution.</p>
    pub fn service_role(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_role(input.into());
        self
    }
    /// <p>The name or ARN of the IAM role that is used as the service role for Amazon EMR (the Amazon EMR role) for the notebook execution.</p>
    pub fn set_service_role(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_role(input);
        self
    }
    /// <p>The unique identifier of the Amazon EC2 security group to associate with the Amazon EMR Notebook for this notebook execution.</p>
    pub fn notebook_instance_security_group_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.notebook_instance_security_group_id(input.into());
        self
    }
    /// <p>The unique identifier of the Amazon EC2 security group to associate with the Amazon EMR Notebook for this notebook execution.</p>
    pub fn set_notebook_instance_security_group_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_notebook_instance_security_group_id(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags associated with a notebook execution. Tags are user-defined key-value pairs that consist of a required key string with a maximum of 128 characters and an optional value string with a maximum of 256 characters.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of tags associated with a notebook execution. Tags are user-defined key-value pairs that consist of a required key string with a maximum of 128 characters and an optional value string with a maximum of 256 characters.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The Amazon S3 location for the notebook execution input.</p>
    pub fn notebook_s3_location(
        mut self,
        input: crate::types::NotebookS3LocationFromInput,
    ) -> Self {
        self.inner = self.inner.notebook_s3_location(input);
        self
    }
    /// <p>The Amazon S3 location for the notebook execution input.</p>
    pub fn set_notebook_s3_location(
        mut self,
        input: ::std::option::Option<crate::types::NotebookS3LocationFromInput>,
    ) -> Self {
        self.inner = self.inner.set_notebook_s3_location(input);
        self
    }
    /// <p>The Amazon S3 location for the notebook execution output.</p>
    pub fn output_notebook_s3_location(
        mut self,
        input: crate::types::OutputNotebookS3LocationFromInput,
    ) -> Self {
        self.inner = self.inner.output_notebook_s3_location(input);
        self
    }
    /// <p>The Amazon S3 location for the notebook execution output.</p>
    pub fn set_output_notebook_s3_location(
        mut self,
        input: ::std::option::Option<crate::types::OutputNotebookS3LocationFromInput>,
    ) -> Self {
        self.inner = self.inner.set_output_notebook_s3_location(input);
        self
    }
    /// <p>The output format for the notebook execution.</p>
    pub fn output_notebook_format(mut self, input: crate::types::OutputNotebookFormat) -> Self {
        self.inner = self.inner.output_notebook_format(input);
        self
    }
    /// <p>The output format for the notebook execution.</p>
    pub fn set_output_notebook_format(
        mut self,
        input: ::std::option::Option<crate::types::OutputNotebookFormat>,
    ) -> Self {
        self.inner = self.inner.set_output_notebook_format(input);
        self
    }
    /// Adds a key-value pair to `EnvironmentVariables`.
    ///
    /// To override the contents of this collection use [`set_environment_variables`](Self::set_environment_variables).
    ///
    /// <p>The environment variables associated with the notebook execution.</p>
    pub fn environment_variables(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.environment_variables(k.into(), v.into());
        self
    }
    /// <p>The environment variables associated with the notebook execution.</p>
    pub fn set_environment_variables(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_environment_variables(input);
        self
    }
}
