// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Status details of a conformance pack.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ConformancePackStatusDetail {
    /// <p>Name of the conformance pack.</p>
    #[doc(hidden)]
    pub conformance_pack_name: ::std::option::Option<::std::string::String>,
    /// <p>ID of the conformance pack.</p>
    #[doc(hidden)]
    pub conformance_pack_id: ::std::option::Option<::std::string::String>,
    /// <p>Amazon Resource Name (ARN) of comformance pack.</p>
    #[doc(hidden)]
    pub conformance_pack_arn: ::std::option::Option<::std::string::String>,
    /// <p>Indicates deployment status of conformance pack.</p>
    /// <p>Config sets the state of the conformance pack to:</p>
    /// <ul>
    /// <li> <p>CREATE_IN_PROGRESS when a conformance pack creation is in progress for an account.</p> </li>
    /// <li> <p>CREATE_COMPLETE when a conformance pack has been successfully created in your account.</p> </li>
    /// <li> <p>CREATE_FAILED when a conformance pack creation failed in your account.</p> </li>
    /// <li> <p>DELETE_IN_PROGRESS when a conformance pack deletion is in progress. </p> </li>
    /// <li> <p>DELETE_FAILED when a conformance pack deletion failed in your account.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub conformance_pack_state: ::std::option::Option<crate::types::ConformancePackState>,
    /// <p>Amazon Resource Name (ARN) of CloudFormation stack. </p>
    #[doc(hidden)]
    pub stack_arn: ::std::option::Option<::std::string::String>,
    /// <p>The reason of conformance pack creation failure.</p>
    #[doc(hidden)]
    pub conformance_pack_status_reason: ::std::option::Option<::std::string::String>,
    /// <p>Last time when conformation pack creation and update was requested.</p>
    #[doc(hidden)]
    pub last_update_requested_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Last time when conformation pack creation and update was successful.</p>
    #[doc(hidden)]
    pub last_update_completed_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ConformancePackStatusDetail {
    /// <p>Name of the conformance pack.</p>
    pub fn conformance_pack_name(&self) -> ::std::option::Option<&str> {
        self.conformance_pack_name.as_deref()
    }
    /// <p>ID of the conformance pack.</p>
    pub fn conformance_pack_id(&self) -> ::std::option::Option<&str> {
        self.conformance_pack_id.as_deref()
    }
    /// <p>Amazon Resource Name (ARN) of comformance pack.</p>
    pub fn conformance_pack_arn(&self) -> ::std::option::Option<&str> {
        self.conformance_pack_arn.as_deref()
    }
    /// <p>Indicates deployment status of conformance pack.</p>
    /// <p>Config sets the state of the conformance pack to:</p>
    /// <ul>
    /// <li> <p>CREATE_IN_PROGRESS when a conformance pack creation is in progress for an account.</p> </li>
    /// <li> <p>CREATE_COMPLETE when a conformance pack has been successfully created in your account.</p> </li>
    /// <li> <p>CREATE_FAILED when a conformance pack creation failed in your account.</p> </li>
    /// <li> <p>DELETE_IN_PROGRESS when a conformance pack deletion is in progress. </p> </li>
    /// <li> <p>DELETE_FAILED when a conformance pack deletion failed in your account.</p> </li>
    /// </ul>
    pub fn conformance_pack_state(
        &self,
    ) -> ::std::option::Option<&crate::types::ConformancePackState> {
        self.conformance_pack_state.as_ref()
    }
    /// <p>Amazon Resource Name (ARN) of CloudFormation stack. </p>
    pub fn stack_arn(&self) -> ::std::option::Option<&str> {
        self.stack_arn.as_deref()
    }
    /// <p>The reason of conformance pack creation failure.</p>
    pub fn conformance_pack_status_reason(&self) -> ::std::option::Option<&str> {
        self.conformance_pack_status_reason.as_deref()
    }
    /// <p>Last time when conformation pack creation and update was requested.</p>
    pub fn last_update_requested_time(
        &self,
    ) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_update_requested_time.as_ref()
    }
    /// <p>Last time when conformation pack creation and update was successful.</p>
    pub fn last_update_completed_time(
        &self,
    ) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_update_completed_time.as_ref()
    }
}
impl ConformancePackStatusDetail {
    /// Creates a new builder-style object to manufacture [`ConformancePackStatusDetail`](crate::types::ConformancePackStatusDetail).
    pub fn builder() -> crate::types::builders::ConformancePackStatusDetailBuilder {
        crate::types::builders::ConformancePackStatusDetailBuilder::default()
    }
}

/// A builder for [`ConformancePackStatusDetail`](crate::types::ConformancePackStatusDetail).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ConformancePackStatusDetailBuilder {
    pub(crate) conformance_pack_name: ::std::option::Option<::std::string::String>,
    pub(crate) conformance_pack_id: ::std::option::Option<::std::string::String>,
    pub(crate) conformance_pack_arn: ::std::option::Option<::std::string::String>,
    pub(crate) conformance_pack_state: ::std::option::Option<crate::types::ConformancePackState>,
    pub(crate) stack_arn: ::std::option::Option<::std::string::String>,
    pub(crate) conformance_pack_status_reason: ::std::option::Option<::std::string::String>,
    pub(crate) last_update_requested_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_update_completed_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ConformancePackStatusDetailBuilder {
    /// <p>Name of the conformance pack.</p>
    pub fn conformance_pack_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.conformance_pack_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the conformance pack.</p>
    pub fn set_conformance_pack_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.conformance_pack_name = input;
        self
    }
    /// <p>ID of the conformance pack.</p>
    pub fn conformance_pack_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.conformance_pack_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ID of the conformance pack.</p>
    pub fn set_conformance_pack_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.conformance_pack_id = input;
        self
    }
    /// <p>Amazon Resource Name (ARN) of comformance pack.</p>
    pub fn conformance_pack_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.conformance_pack_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of comformance pack.</p>
    pub fn set_conformance_pack_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.conformance_pack_arn = input;
        self
    }
    /// <p>Indicates deployment status of conformance pack.</p>
    /// <p>Config sets the state of the conformance pack to:</p>
    /// <ul>
    /// <li> <p>CREATE_IN_PROGRESS when a conformance pack creation is in progress for an account.</p> </li>
    /// <li> <p>CREATE_COMPLETE when a conformance pack has been successfully created in your account.</p> </li>
    /// <li> <p>CREATE_FAILED when a conformance pack creation failed in your account.</p> </li>
    /// <li> <p>DELETE_IN_PROGRESS when a conformance pack deletion is in progress. </p> </li>
    /// <li> <p>DELETE_FAILED when a conformance pack deletion failed in your account.</p> </li>
    /// </ul>
    pub fn conformance_pack_state(mut self, input: crate::types::ConformancePackState) -> Self {
        self.conformance_pack_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates deployment status of conformance pack.</p>
    /// <p>Config sets the state of the conformance pack to:</p>
    /// <ul>
    /// <li> <p>CREATE_IN_PROGRESS when a conformance pack creation is in progress for an account.</p> </li>
    /// <li> <p>CREATE_COMPLETE when a conformance pack has been successfully created in your account.</p> </li>
    /// <li> <p>CREATE_FAILED when a conformance pack creation failed in your account.</p> </li>
    /// <li> <p>DELETE_IN_PROGRESS when a conformance pack deletion is in progress. </p> </li>
    /// <li> <p>DELETE_FAILED when a conformance pack deletion failed in your account.</p> </li>
    /// </ul>
    pub fn set_conformance_pack_state(
        mut self,
        input: ::std::option::Option<crate::types::ConformancePackState>,
    ) -> Self {
        self.conformance_pack_state = input;
        self
    }
    /// <p>Amazon Resource Name (ARN) of CloudFormation stack. </p>
    pub fn stack_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stack_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of CloudFormation stack. </p>
    pub fn set_stack_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stack_arn = input;
        self
    }
    /// <p>The reason of conformance pack creation failure.</p>
    pub fn conformance_pack_status_reason(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.conformance_pack_status_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reason of conformance pack creation failure.</p>
    pub fn set_conformance_pack_status_reason(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.conformance_pack_status_reason = input;
        self
    }
    /// <p>Last time when conformation pack creation and update was requested.</p>
    pub fn last_update_requested_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_update_requested_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Last time when conformation pack creation and update was requested.</p>
    pub fn set_last_update_requested_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_update_requested_time = input;
        self
    }
    /// <p>Last time when conformation pack creation and update was successful.</p>
    pub fn last_update_completed_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_update_completed_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Last time when conformation pack creation and update was successful.</p>
    pub fn set_last_update_completed_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_update_completed_time = input;
        self
    }
    /// Consumes the builder and constructs a [`ConformancePackStatusDetail`](crate::types::ConformancePackStatusDetail).
    pub fn build(self) -> crate::types::ConformancePackStatusDetail {
        crate::types::ConformancePackStatusDetail {
            conformance_pack_name: self.conformance_pack_name,
            conformance_pack_id: self.conformance_pack_id,
            conformance_pack_arn: self.conformance_pack_arn,
            conformance_pack_state: self.conformance_pack_state,
            stack_arn: self.stack_arn,
            conformance_pack_status_reason: self.conformance_pack_status_reason,
            last_update_requested_time: self.last_update_requested_time,
            last_update_completed_time: self.last_update_completed_time,
        }
    }
}
