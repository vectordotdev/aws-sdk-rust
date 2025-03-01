// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An Amazon Web Services account that is the administrator account of or a member of a behavior graph.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Account {
    /// <p>The account identifier of the Amazon Web Services account.</p>
    #[doc(hidden)]
    pub account_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services account root user email address for the Amazon Web Services account.</p>
    #[doc(hidden)]
    pub email_address: ::std::option::Option<::std::string::String>,
}
impl Account {
    /// <p>The account identifier of the Amazon Web Services account.</p>
    pub fn account_id(&self) -> ::std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The Amazon Web Services account root user email address for the Amazon Web Services account.</p>
    pub fn email_address(&self) -> ::std::option::Option<&str> {
        self.email_address.as_deref()
    }
}
impl Account {
    /// Creates a new builder-style object to manufacture [`Account`](crate::types::Account).
    pub fn builder() -> crate::types::builders::AccountBuilder {
        crate::types::builders::AccountBuilder::default()
    }
}

/// A builder for [`Account`](crate::types::Account).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AccountBuilder {
    pub(crate) account_id: ::std::option::Option<::std::string::String>,
    pub(crate) email_address: ::std::option::Option<::std::string::String>,
}
impl AccountBuilder {
    /// <p>The account identifier of the Amazon Web Services account.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The account identifier of the Amazon Web Services account.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The Amazon Web Services account root user email address for the Amazon Web Services account.</p>
    pub fn email_address(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.email_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account root user email address for the Amazon Web Services account.</p>
    pub fn set_email_address(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.email_address = input;
        self
    }
    /// Consumes the builder and constructs a [`Account`](crate::types::Account).
    pub fn build(self) -> crate::types::Account {
        crate::types::Account {
            account_id: self.account_id,
            email_address: self.email_address,
        }
    }
}
