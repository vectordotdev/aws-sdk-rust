// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains details about an execution.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExecutionListItem {
    /// <p>The Amazon Resource Name (ARN) that identifies the execution.</p>
    #[doc(hidden)]
    pub execution_arn: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the executed state machine.</p>
    #[doc(hidden)]
    pub state_machine_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the execution.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li> <p>white space</p> </li>
    /// <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li>
    /// <li> <p>wildcard characters <code>? *</code> </p> </li>
    /// <li> <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li>
    /// <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The current status of the execution.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::ExecutionStatus>,
    /// <p>The date the execution started.</p>
    #[doc(hidden)]
    pub start_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>If the execution already ended, the date the execution stopped.</p>
    #[doc(hidden)]
    pub stop_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The Amazon Resource Name (ARN) of a Map Run. This field is returned only if <code>mapRunArn</code> was specified in the <code>ListExecutions</code> API action. If <code>stateMachineArn</code> was specified in <code>ListExecutions</code>, the <code>mapRunArn</code> isn't returned.</p>
    #[doc(hidden)]
    pub map_run_arn: ::std::option::Option<::std::string::String>,
    /// <p>The total number of items processed in a child workflow execution. This field is returned only if <code>mapRunArn</code> was specified in the <code>ListExecutions</code> API action. If <code>stateMachineArn</code> was specified in <code>ListExecutions</code>, the <code>itemCount</code> field isn't returned.</p>
    #[doc(hidden)]
    pub item_count: ::std::option::Option<i32>,
}
impl ExecutionListItem {
    /// <p>The Amazon Resource Name (ARN) that identifies the execution.</p>
    pub fn execution_arn(&self) -> ::std::option::Option<&str> {
        self.execution_arn.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the executed state machine.</p>
    pub fn state_machine_arn(&self) -> ::std::option::Option<&str> {
        self.state_machine_arn.as_deref()
    }
    /// <p>The name of the execution.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li> <p>white space</p> </li>
    /// <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li>
    /// <li> <p>wildcard characters <code>? *</code> </p> </li>
    /// <li> <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li>
    /// <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The current status of the execution.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ExecutionStatus> {
        self.status.as_ref()
    }
    /// <p>The date the execution started.</p>
    pub fn start_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_date.as_ref()
    }
    /// <p>If the execution already ended, the date the execution stopped.</p>
    pub fn stop_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.stop_date.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of a Map Run. This field is returned only if <code>mapRunArn</code> was specified in the <code>ListExecutions</code> API action. If <code>stateMachineArn</code> was specified in <code>ListExecutions</code>, the <code>mapRunArn</code> isn't returned.</p>
    pub fn map_run_arn(&self) -> ::std::option::Option<&str> {
        self.map_run_arn.as_deref()
    }
    /// <p>The total number of items processed in a child workflow execution. This field is returned only if <code>mapRunArn</code> was specified in the <code>ListExecutions</code> API action. If <code>stateMachineArn</code> was specified in <code>ListExecutions</code>, the <code>itemCount</code> field isn't returned.</p>
    pub fn item_count(&self) -> ::std::option::Option<i32> {
        self.item_count
    }
}
impl ExecutionListItem {
    /// Creates a new builder-style object to manufacture [`ExecutionListItem`](crate::types::ExecutionListItem).
    pub fn builder() -> crate::types::builders::ExecutionListItemBuilder {
        crate::types::builders::ExecutionListItemBuilder::default()
    }
}

/// A builder for [`ExecutionListItem`](crate::types::ExecutionListItem).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExecutionListItemBuilder {
    pub(crate) execution_arn: ::std::option::Option<::std::string::String>,
    pub(crate) state_machine_arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ExecutionStatus>,
    pub(crate) start_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) stop_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) map_run_arn: ::std::option::Option<::std::string::String>,
    pub(crate) item_count: ::std::option::Option<i32>,
}
impl ExecutionListItemBuilder {
    /// <p>The Amazon Resource Name (ARN) that identifies the execution.</p>
    pub fn execution_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.execution_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that identifies the execution.</p>
    pub fn set_execution_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.execution_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the executed state machine.</p>
    pub fn state_machine_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.state_machine_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the executed state machine.</p>
    pub fn set_state_machine_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.state_machine_arn = input;
        self
    }
    /// <p>The name of the execution.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li> <p>white space</p> </li>
    /// <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li>
    /// <li> <p>wildcard characters <code>? *</code> </p> </li>
    /// <li> <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li>
    /// <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the execution.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li> <p>white space</p> </li>
    /// <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li>
    /// <li> <p>wildcard characters <code>? *</code> </p> </li>
    /// <li> <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li>
    /// <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The current status of the execution.</p>
    pub fn status(mut self, input: crate::types::ExecutionStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the execution.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::ExecutionStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The date the execution started.</p>
    pub fn start_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date the execution started.</p>
    pub fn set_start_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.start_date = input;
        self
    }
    /// <p>If the execution already ended, the date the execution stopped.</p>
    pub fn stop_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.stop_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>If the execution already ended, the date the execution stopped.</p>
    pub fn set_stop_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.stop_date = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a Map Run. This field is returned only if <code>mapRunArn</code> was specified in the <code>ListExecutions</code> API action. If <code>stateMachineArn</code> was specified in <code>ListExecutions</code>, the <code>mapRunArn</code> isn't returned.</p>
    pub fn map_run_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.map_run_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a Map Run. This field is returned only if <code>mapRunArn</code> was specified in the <code>ListExecutions</code> API action. If <code>stateMachineArn</code> was specified in <code>ListExecutions</code>, the <code>mapRunArn</code> isn't returned.</p>
    pub fn set_map_run_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.map_run_arn = input;
        self
    }
    /// <p>The total number of items processed in a child workflow execution. This field is returned only if <code>mapRunArn</code> was specified in the <code>ListExecutions</code> API action. If <code>stateMachineArn</code> was specified in <code>ListExecutions</code>, the <code>itemCount</code> field isn't returned.</p>
    pub fn item_count(mut self, input: i32) -> Self {
        self.item_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total number of items processed in a child workflow execution. This field is returned only if <code>mapRunArn</code> was specified in the <code>ListExecutions</code> API action. If <code>stateMachineArn</code> was specified in <code>ListExecutions</code>, the <code>itemCount</code> field isn't returned.</p>
    pub fn set_item_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.item_count = input;
        self
    }
    /// Consumes the builder and constructs a [`ExecutionListItem`](crate::types::ExecutionListItem).
    pub fn build(self) -> crate::types::ExecutionListItem {
        crate::types::ExecutionListItem {
            execution_arn: self.execution_arn,
            state_machine_arn: self.state_machine_arn,
            name: self.name,
            status: self.status,
            start_date: self.start_date,
            stop_date: self.stop_date,
            map_run_arn: self.map_run_arn,
            item_count: self.item_count,
        }
    }
}
