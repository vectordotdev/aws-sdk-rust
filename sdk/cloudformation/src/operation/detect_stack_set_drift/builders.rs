// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::detect_stack_set_drift::_detect_stack_set_drift_output::DetectStackSetDriftOutputBuilder;

pub use crate::operation::detect_stack_set_drift::_detect_stack_set_drift_input::DetectStackSetDriftInputBuilder;

/// Fluent builder constructing a request to `DetectStackSetDrift`.
///
/// <p>Detect drift on a stack set. When CloudFormation performs drift detection on a stack set, it performs drift detection on the stack associated with each stack instance in the stack set. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-drift.html">How CloudFormation performs drift detection on a stack set</a>.</p>
/// <p> <code>DetectStackSetDrift</code> returns the <code>OperationId</code> of the stack set drift detection operation. Use this operation id with <code> <code>DescribeStackSetOperation</code> </code> to monitor the progress of the drift detection operation. The drift detection operation may take some time, depending on the number of stack instances included in the stack set, in addition to the number of resources included in each stack.</p>
/// <p>Once the operation has completed, use the following actions to return drift information:</p>
/// <ul>
/// <li> <p>Use <code> <code>DescribeStackSet</code> </code> to return detailed information about the stack set, including detailed information about the last <i>completed</i> drift operation performed on the stack set. (Information about drift operations that are in progress isn't included.)</p> </li>
/// <li> <p>Use <code> <code>ListStackInstances</code> </code> to return a list of stack instances belonging to the stack set, including the drift status and last drift time checked of each instance.</p> </li>
/// <li> <p>Use <code> <code>DescribeStackInstance</code> </code> to return detailed information about a specific stack instance, including its drift status and last drift time checked.</p> </li>
/// </ul>
/// <p>For more information about performing a drift detection operation on a stack set, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-drift.html">Detecting unmanaged changes in stack sets</a>.</p>
/// <p>You can only run a single drift detection operation on a given stack set at one time.</p>
/// <p>To stop a drift detection stack set operation, use <code> <code>StopStackSetOperation</code> </code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DetectStackSetDriftFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::detect_stack_set_drift::builders::DetectStackSetDriftInputBuilder,
}
impl DetectStackSetDriftFluentBuilder {
    /// Creates a new `DetectStackSetDrift`.
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
            crate::operation::detect_stack_set_drift::DetectStackSetDrift,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::detect_stack_set_drift::DetectStackSetDriftError,
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
        crate::operation::detect_stack_set_drift::DetectStackSetDriftOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::detect_stack_set_drift::DetectStackSetDriftError,
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
        crate::operation::detect_stack_set_drift::DetectStackSetDriftOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::detect_stack_set_drift::DetectStackSetDriftError,
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
            crate::operation::detect_stack_set_drift::DetectStackSetDrift,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::detect_stack_set_drift::DetectStackSetDriftError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the stack set on which to perform the drift detection operation.</p>
    pub fn stack_set_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.stack_set_name(input.into());
        self
    }
    /// <p>The name of the stack set on which to perform the drift detection operation.</p>
    pub fn set_stack_set_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_stack_set_name(input);
        self
    }
    /// <p>The user-specified preferences for how CloudFormation performs a stack set operation.</p>
    /// <p>For more information about maximum concurrent accounts and failure tolerance, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-concepts.html#stackset-ops-options">Stack set operation options</a>.</p>
    pub fn operation_preferences(
        mut self,
        input: crate::types::StackSetOperationPreferences,
    ) -> Self {
        self.inner = self.inner.operation_preferences(input);
        self
    }
    /// <p>The user-specified preferences for how CloudFormation performs a stack set operation.</p>
    /// <p>For more information about maximum concurrent accounts and failure tolerance, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-concepts.html#stackset-ops-options">Stack set operation options</a>.</p>
    pub fn set_operation_preferences(
        mut self,
        input: ::std::option::Option<crate::types::StackSetOperationPreferences>,
    ) -> Self {
        self.inner = self.inner.set_operation_preferences(input);
        self
    }
    /// <p> <i>The ID of the stack set operation.</i> </p>
    pub fn operation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.operation_id(input.into());
        self
    }
    /// <p> <i>The ID of the stack set operation.</i> </p>
    pub fn set_operation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_operation_id(input);
        self
    }
    /// <p>[Service-managed permissions] Specifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account.</p>
    /// <p>By default, <code>SELF</code> is specified. Use <code>SELF</code> for stack sets with self-managed permissions.</p>
    /// <ul>
    /// <li> <p>If you are signed in to the management account, specify <code>SELF</code>.</p> </li>
    /// <li> <p>If you are signed in to a delegated administrator account, specify <code>DELEGATED_ADMIN</code>.</p> <p>Your Amazon Web Services account must be registered as a delegated administrator in the management account. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-orgs-delegated-admin.html">Register a delegated administrator</a> in the <i>CloudFormation User Guide</i>.</p> </li>
    /// </ul>
    pub fn call_as(mut self, input: crate::types::CallAs) -> Self {
        self.inner = self.inner.call_as(input);
        self
    }
    /// <p>[Service-managed permissions] Specifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account.</p>
    /// <p>By default, <code>SELF</code> is specified. Use <code>SELF</code> for stack sets with self-managed permissions.</p>
    /// <ul>
    /// <li> <p>If you are signed in to the management account, specify <code>SELF</code>.</p> </li>
    /// <li> <p>If you are signed in to a delegated administrator account, specify <code>DELEGATED_ADMIN</code>.</p> <p>Your Amazon Web Services account must be registered as a delegated administrator in the management account. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-orgs-delegated-admin.html">Register a delegated administrator</a> in the <i>CloudFormation User Guide</i>.</p> </li>
    /// </ul>
    pub fn set_call_as(mut self, input: ::std::option::Option<crate::types::CallAs>) -> Self {
        self.inner = self.inner.set_call_as(input);
        self
    }
}
