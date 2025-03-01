// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for CancelBundleTask.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelBundleTaskInput {
    /// <p>The ID of the bundle task.</p>
    #[doc(hidden)]
    pub bundle_id: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
}
impl CancelBundleTaskInput {
    /// <p>The ID of the bundle task.</p>
    pub fn bundle_id(&self) -> ::std::option::Option<&str> {
        self.bundle_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl CancelBundleTaskInput {
    /// Creates a new builder-style object to manufacture [`CancelBundleTaskInput`](crate::operation::cancel_bundle_task::CancelBundleTaskInput).
    pub fn builder() -> crate::operation::cancel_bundle_task::builders::CancelBundleTaskInputBuilder
    {
        crate::operation::cancel_bundle_task::builders::CancelBundleTaskInputBuilder::default()
    }
}

/// A builder for [`CancelBundleTaskInput`](crate::operation::cancel_bundle_task::CancelBundleTaskInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CancelBundleTaskInputBuilder {
    pub(crate) bundle_id: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl CancelBundleTaskInputBuilder {
    /// <p>The ID of the bundle task.</p>
    pub fn bundle_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bundle_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the bundle task.</p>
    pub fn set_bundle_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bundle_id = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`CancelBundleTaskInput`](crate::operation::cancel_bundle_task::CancelBundleTaskInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::cancel_bundle_task::CancelBundleTaskInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::cancel_bundle_task::CancelBundleTaskInput {
                bundle_id: self.bundle_id,
                dry_run: self.dry_run,
            },
        )
    }
}
