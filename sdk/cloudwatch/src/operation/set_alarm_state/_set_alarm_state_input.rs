// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SetAlarmStateInput {
    /// <p>The name of the alarm.</p>
    #[doc(hidden)]
    pub alarm_name: ::std::option::Option<::std::string::String>,
    /// <p>The value of the state.</p>
    #[doc(hidden)]
    pub state_value: ::std::option::Option<crate::types::StateValue>,
    /// <p>The reason that this alarm is set to this specific state, in text format.</p>
    #[doc(hidden)]
    pub state_reason: ::std::option::Option<::std::string::String>,
    /// <p>The reason that this alarm is set to this specific state, in JSON format.</p>
    /// <p>For SNS or EC2 alarm actions, this is just informational. But for EC2 Auto Scaling or application Auto Scaling alarm actions, the Auto Scaling policy uses the information in this field to take the correct action.</p>
    #[doc(hidden)]
    pub state_reason_data: ::std::option::Option<::std::string::String>,
}
impl SetAlarmStateInput {
    /// <p>The name of the alarm.</p>
    pub fn alarm_name(&self) -> ::std::option::Option<&str> {
        self.alarm_name.as_deref()
    }
    /// <p>The value of the state.</p>
    pub fn state_value(&self) -> ::std::option::Option<&crate::types::StateValue> {
        self.state_value.as_ref()
    }
    /// <p>The reason that this alarm is set to this specific state, in text format.</p>
    pub fn state_reason(&self) -> ::std::option::Option<&str> {
        self.state_reason.as_deref()
    }
    /// <p>The reason that this alarm is set to this specific state, in JSON format.</p>
    /// <p>For SNS or EC2 alarm actions, this is just informational. But for EC2 Auto Scaling or application Auto Scaling alarm actions, the Auto Scaling policy uses the information in this field to take the correct action.</p>
    pub fn state_reason_data(&self) -> ::std::option::Option<&str> {
        self.state_reason_data.as_deref()
    }
}
impl SetAlarmStateInput {
    /// Creates a new builder-style object to manufacture [`SetAlarmStateInput`](crate::operation::set_alarm_state::SetAlarmStateInput).
    pub fn builder() -> crate::operation::set_alarm_state::builders::SetAlarmStateInputBuilder {
        crate::operation::set_alarm_state::builders::SetAlarmStateInputBuilder::default()
    }
}

/// A builder for [`SetAlarmStateInput`](crate::operation::set_alarm_state::SetAlarmStateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SetAlarmStateInputBuilder {
    pub(crate) alarm_name: ::std::option::Option<::std::string::String>,
    pub(crate) state_value: ::std::option::Option<crate::types::StateValue>,
    pub(crate) state_reason: ::std::option::Option<::std::string::String>,
    pub(crate) state_reason_data: ::std::option::Option<::std::string::String>,
}
impl SetAlarmStateInputBuilder {
    /// <p>The name of the alarm.</p>
    pub fn alarm_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.alarm_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the alarm.</p>
    pub fn set_alarm_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.alarm_name = input;
        self
    }
    /// <p>The value of the state.</p>
    pub fn state_value(mut self, input: crate::types::StateValue) -> Self {
        self.state_value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The value of the state.</p>
    pub fn set_state_value(
        mut self,
        input: ::std::option::Option<crate::types::StateValue>,
    ) -> Self {
        self.state_value = input;
        self
    }
    /// <p>The reason that this alarm is set to this specific state, in text format.</p>
    pub fn state_reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.state_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reason that this alarm is set to this specific state, in text format.</p>
    pub fn set_state_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.state_reason = input;
        self
    }
    /// <p>The reason that this alarm is set to this specific state, in JSON format.</p>
    /// <p>For SNS or EC2 alarm actions, this is just informational. But for EC2 Auto Scaling or application Auto Scaling alarm actions, the Auto Scaling policy uses the information in this field to take the correct action.</p>
    pub fn state_reason_data(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.state_reason_data = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reason that this alarm is set to this specific state, in JSON format.</p>
    /// <p>For SNS or EC2 alarm actions, this is just informational. But for EC2 Auto Scaling or application Auto Scaling alarm actions, the Auto Scaling policy uses the information in this field to take the correct action.</p>
    pub fn set_state_reason_data(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.state_reason_data = input;
        self
    }
    /// Consumes the builder and constructs a [`SetAlarmStateInput`](crate::operation::set_alarm_state::SetAlarmStateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::set_alarm_state::SetAlarmStateInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::set_alarm_state::SetAlarmStateInput {
            alarm_name: self.alarm_name,
            state_value: self.state_value,
            state_reason: self.state_reason,
            state_reason_data: self.state_reason_data,
        })
    }
}
