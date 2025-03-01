// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeregisterApplicationInput {
    /// <p>The ID of the application.</p>
    #[doc(hidden)]
    pub application_id: ::std::option::Option<::std::string::String>,
}
impl DeregisterApplicationInput {
    /// <p>The ID of the application.</p>
    pub fn application_id(&self) -> ::std::option::Option<&str> {
        self.application_id.as_deref()
    }
}
impl DeregisterApplicationInput {
    /// Creates a new builder-style object to manufacture [`DeregisterApplicationInput`](crate::operation::deregister_application::DeregisterApplicationInput).
    pub fn builder(
    ) -> crate::operation::deregister_application::builders::DeregisterApplicationInputBuilder {
        crate::operation::deregister_application::builders::DeregisterApplicationInputBuilder::default()
    }
}

/// A builder for [`DeregisterApplicationInput`](crate::operation::deregister_application::DeregisterApplicationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeregisterApplicationInputBuilder {
    pub(crate) application_id: ::std::option::Option<::std::string::String>,
}
impl DeregisterApplicationInputBuilder {
    /// <p>The ID of the application.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the application.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeregisterApplicationInput`](crate::operation::deregister_application::DeregisterApplicationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::deregister_application::DeregisterApplicationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::deregister_application::DeregisterApplicationInput {
                application_id: self.application_id,
            },
        )
    }
}
