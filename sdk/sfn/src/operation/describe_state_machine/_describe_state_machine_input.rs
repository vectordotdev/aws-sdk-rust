// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeStateMachineInput {
    /// <p>The Amazon Resource Name (ARN) of the state machine to describe.</p>
    #[doc(hidden)]
    pub state_machine_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeStateMachineInput {
    /// <p>The Amazon Resource Name (ARN) of the state machine to describe.</p>
    pub fn state_machine_arn(&self) -> ::std::option::Option<&str> {
        self.state_machine_arn.as_deref()
    }
}
impl DescribeStateMachineInput {
    /// Creates a new builder-style object to manufacture [`DescribeStateMachineInput`](crate::operation::describe_state_machine::DescribeStateMachineInput).
    pub fn builder(
    ) -> crate::operation::describe_state_machine::builders::DescribeStateMachineInputBuilder {
        crate::operation::describe_state_machine::builders::DescribeStateMachineInputBuilder::default()
    }
}

/// A builder for [`DescribeStateMachineInput`](crate::operation::describe_state_machine::DescribeStateMachineInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeStateMachineInputBuilder {
    pub(crate) state_machine_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeStateMachineInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the state machine to describe.</p>
    pub fn state_machine_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.state_machine_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the state machine to describe.</p>
    pub fn set_state_machine_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.state_machine_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeStateMachineInput`](crate::operation::describe_state_machine::DescribeStateMachineInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_state_machine::DescribeStateMachineInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_state_machine::DescribeStateMachineInput {
                state_machine_arn: self.state_machine_arn,
            },
        )
    }
}
