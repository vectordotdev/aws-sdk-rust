// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RegisterToWorkMailInput {
    /// <p>The identifier for the organization under which the user, group, or resource exists.</p>
    #[doc(hidden)]
    pub organization_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier for the user, group, or resource to be updated.</p>
    #[doc(hidden)]
    pub entity_id: ::std::option::Option<::std::string::String>,
    /// <p>The email for the user, group, or resource to be updated.</p>
    #[doc(hidden)]
    pub email: ::std::option::Option<::std::string::String>,
}
impl RegisterToWorkMailInput {
    /// <p>The identifier for the organization under which the user, group, or resource exists.</p>
    pub fn organization_id(&self) -> ::std::option::Option<&str> {
        self.organization_id.as_deref()
    }
    /// <p>The identifier for the user, group, or resource to be updated.</p>
    pub fn entity_id(&self) -> ::std::option::Option<&str> {
        self.entity_id.as_deref()
    }
    /// <p>The email for the user, group, or resource to be updated.</p>
    pub fn email(&self) -> ::std::option::Option<&str> {
        self.email.as_deref()
    }
}
impl RegisterToWorkMailInput {
    /// Creates a new builder-style object to manufacture [`RegisterToWorkMailInput`](crate::operation::register_to_work_mail::RegisterToWorkMailInput).
    pub fn builder(
    ) -> crate::operation::register_to_work_mail::builders::RegisterToWorkMailInputBuilder {
        crate::operation::register_to_work_mail::builders::RegisterToWorkMailInputBuilder::default()
    }
}

/// A builder for [`RegisterToWorkMailInput`](crate::operation::register_to_work_mail::RegisterToWorkMailInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RegisterToWorkMailInputBuilder {
    pub(crate) organization_id: ::std::option::Option<::std::string::String>,
    pub(crate) entity_id: ::std::option::Option<::std::string::String>,
    pub(crate) email: ::std::option::Option<::std::string::String>,
}
impl RegisterToWorkMailInputBuilder {
    /// <p>The identifier for the organization under which the user, group, or resource exists.</p>
    pub fn organization_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.organization_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the organization under which the user, group, or resource exists.</p>
    pub fn set_organization_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.organization_id = input;
        self
    }
    /// <p>The identifier for the user, group, or resource to be updated.</p>
    pub fn entity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.entity_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the user, group, or resource to be updated.</p>
    pub fn set_entity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.entity_id = input;
        self
    }
    /// <p>The email for the user, group, or resource to be updated.</p>
    pub fn email(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.email = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The email for the user, group, or resource to be updated.</p>
    pub fn set_email(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.email = input;
        self
    }
    /// Consumes the builder and constructs a [`RegisterToWorkMailInput`](crate::operation::register_to_work_mail::RegisterToWorkMailInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::register_to_work_mail::RegisterToWorkMailInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::register_to_work_mail::RegisterToWorkMailInput {
                organization_id: self.organization_id,
                entity_id: self.entity_id,
                email: self.email,
            },
        )
    }
}
