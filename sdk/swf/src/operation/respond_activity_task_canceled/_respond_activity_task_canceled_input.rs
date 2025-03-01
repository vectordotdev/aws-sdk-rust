// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RespondActivityTaskCanceledInput {
    /// <p>The <code>taskToken</code> of the <code>ActivityTask</code>.</p> <important>
    /// <p> <code>taskToken</code> is generated by the service and should be treated as an opaque value. If the task is passed to another process, its <code>taskToken</code> must also be passed. This enables it to provide its progress and respond with results.</p>
    /// </important>
    #[doc(hidden)]
    pub task_token: ::std::option::Option<::std::string::String>,
    /// <p> Information about the cancellation.</p>
    #[doc(hidden)]
    pub details: ::std::option::Option<::std::string::String>,
}
impl RespondActivityTaskCanceledInput {
    /// <p>The <code>taskToken</code> of the <code>ActivityTask</code>.</p> <important>
    /// <p> <code>taskToken</code> is generated by the service and should be treated as an opaque value. If the task is passed to another process, its <code>taskToken</code> must also be passed. This enables it to provide its progress and respond with results.</p>
    /// </important>
    pub fn task_token(&self) -> ::std::option::Option<&str> {
        self.task_token.as_deref()
    }
    /// <p> Information about the cancellation.</p>
    pub fn details(&self) -> ::std::option::Option<&str> {
        self.details.as_deref()
    }
}
impl RespondActivityTaskCanceledInput {
    /// Creates a new builder-style object to manufacture [`RespondActivityTaskCanceledInput`](crate::operation::respond_activity_task_canceled::RespondActivityTaskCanceledInput).
    pub fn builder() -> crate::operation::respond_activity_task_canceled::builders::RespondActivityTaskCanceledInputBuilder{
        crate::operation::respond_activity_task_canceled::builders::RespondActivityTaskCanceledInputBuilder::default()
    }
}

/// A builder for [`RespondActivityTaskCanceledInput`](crate::operation::respond_activity_task_canceled::RespondActivityTaskCanceledInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RespondActivityTaskCanceledInputBuilder {
    pub(crate) task_token: ::std::option::Option<::std::string::String>,
    pub(crate) details: ::std::option::Option<::std::string::String>,
}
impl RespondActivityTaskCanceledInputBuilder {
    /// <p>The <code>taskToken</code> of the <code>ActivityTask</code>.</p> <important>
    /// <p> <code>taskToken</code> is generated by the service and should be treated as an opaque value. If the task is passed to another process, its <code>taskToken</code> must also be passed. This enables it to provide its progress and respond with results.</p>
    /// </important>
    pub fn task_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.task_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>taskToken</code> of the <code>ActivityTask</code>.</p> <important>
    /// <p> <code>taskToken</code> is generated by the service and should be treated as an opaque value. If the task is passed to another process, its <code>taskToken</code> must also be passed. This enables it to provide its progress and respond with results.</p>
    /// </important>
    pub fn set_task_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.task_token = input;
        self
    }
    /// <p> Information about the cancellation.</p>
    pub fn details(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.details = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> Information about the cancellation.</p>
    pub fn set_details(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.details = input;
        self
    }
    /// Consumes the builder and constructs a [`RespondActivityTaskCanceledInput`](crate::operation::respond_activity_task_canceled::RespondActivityTaskCanceledInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::respond_activity_task_canceled::RespondActivityTaskCanceledInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::respond_activity_task_canceled::RespondActivityTaskCanceledInput {
                task_token: self.task_token,
                details: self.details,
            },
        )
    }
}
