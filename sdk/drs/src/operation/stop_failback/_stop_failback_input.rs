// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StopFailbackInput {
    /// <p>The ID of the Recovery Instance we want to stop failback for.</p>
    #[doc(hidden)]
    pub recovery_instance_id: ::std::option::Option<::std::string::String>,
}
impl StopFailbackInput {
    /// <p>The ID of the Recovery Instance we want to stop failback for.</p>
    pub fn recovery_instance_id(&self) -> ::std::option::Option<&str> {
        self.recovery_instance_id.as_deref()
    }
}
impl StopFailbackInput {
    /// Creates a new builder-style object to manufacture [`StopFailbackInput`](crate::operation::stop_failback::StopFailbackInput).
    pub fn builder() -> crate::operation::stop_failback::builders::StopFailbackInputBuilder {
        crate::operation::stop_failback::builders::StopFailbackInputBuilder::default()
    }
}

/// A builder for [`StopFailbackInput`](crate::operation::stop_failback::StopFailbackInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StopFailbackInputBuilder {
    pub(crate) recovery_instance_id: ::std::option::Option<::std::string::String>,
}
impl StopFailbackInputBuilder {
    /// <p>The ID of the Recovery Instance we want to stop failback for.</p>
    pub fn recovery_instance_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.recovery_instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Recovery Instance we want to stop failback for.</p>
    pub fn set_recovery_instance_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.recovery_instance_id = input;
        self
    }
    /// Consumes the builder and constructs a [`StopFailbackInput`](crate::operation::stop_failback::StopFailbackInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::stop_failback::StopFailbackInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::stop_failback::StopFailbackInput {
            recovery_instance_id: self.recovery_instance_id,
        })
    }
}
