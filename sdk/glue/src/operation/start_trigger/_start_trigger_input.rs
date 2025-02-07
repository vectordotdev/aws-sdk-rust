// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartTriggerInput {
    /// <p>The name of the trigger to start.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl StartTriggerInput {
    /// <p>The name of the trigger to start.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl StartTriggerInput {
    /// Creates a new builder-style object to manufacture [`StartTriggerInput`](crate::operation::start_trigger::StartTriggerInput).
    pub fn builder() -> crate::operation::start_trigger::builders::StartTriggerInputBuilder {
        crate::operation::start_trigger::builders::StartTriggerInputBuilder::default()
    }
}

/// A builder for [`StartTriggerInput`](crate::operation::start_trigger::StartTriggerInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartTriggerInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl StartTriggerInputBuilder {
    /// <p>The name of the trigger to start.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the trigger to start.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`StartTriggerInput`](crate::operation::start_trigger::StartTriggerInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_trigger::StartTriggerInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::start_trigger::StartTriggerInput {
            name: self.name,
        })
    }
}
