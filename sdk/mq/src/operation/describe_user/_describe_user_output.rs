// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeUserOutput {
    /// <p>Required. The unique ID that Amazon MQ generates for the broker.</p>
    #[doc(hidden)]
    pub broker_id: ::std::option::Option<::std::string::String>,
    /// <p>Enables access to the the ActiveMQ Web Console for the ActiveMQ user.</p>
    #[doc(hidden)]
    pub console_access: ::std::option::Option<bool>,
    /// <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[doc(hidden)]
    pub groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The status of the changes pending for the ActiveMQ user.</p>
    #[doc(hidden)]
    pub pending: ::std::option::Option<crate::types::UserPendingChanges>,
    /// <p>Required. The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    #[doc(hidden)]
    pub username: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeUserOutput {
    /// <p>Required. The unique ID that Amazon MQ generates for the broker.</p>
    pub fn broker_id(&self) -> ::std::option::Option<&str> {
        self.broker_id.as_deref()
    }
    /// <p>Enables access to the the ActiveMQ Web Console for the ActiveMQ user.</p>
    pub fn console_access(&self) -> ::std::option::Option<bool> {
        self.console_access
    }
    /// <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    pub fn groups(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.groups.as_deref()
    }
    /// <p>The status of the changes pending for the ActiveMQ user.</p>
    pub fn pending(&self) -> ::std::option::Option<&crate::types::UserPendingChanges> {
        self.pending.as_ref()
    }
    /// <p>Required. The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    pub fn username(&self) -> ::std::option::Option<&str> {
        self.username.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeUserOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeUserOutput {
    /// Creates a new builder-style object to manufacture [`DescribeUserOutput`](crate::operation::describe_user::DescribeUserOutput).
    pub fn builder() -> crate::operation::describe_user::builders::DescribeUserOutputBuilder {
        crate::operation::describe_user::builders::DescribeUserOutputBuilder::default()
    }
}

/// A builder for [`DescribeUserOutput`](crate::operation::describe_user::DescribeUserOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeUserOutputBuilder {
    pub(crate) broker_id: ::std::option::Option<::std::string::String>,
    pub(crate) console_access: ::std::option::Option<bool>,
    pub(crate) groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) pending: ::std::option::Option<crate::types::UserPendingChanges>,
    pub(crate) username: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeUserOutputBuilder {
    /// <p>Required. The unique ID that Amazon MQ generates for the broker.</p>
    pub fn broker_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.broker_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Required. The unique ID that Amazon MQ generates for the broker.</p>
    pub fn set_broker_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.broker_id = input;
        self
    }
    /// <p>Enables access to the the ActiveMQ Web Console for the ActiveMQ user.</p>
    pub fn console_access(mut self, input: bool) -> Self {
        self.console_access = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enables access to the the ActiveMQ Web Console for the ActiveMQ user.</p>
    pub fn set_console_access(mut self, input: ::std::option::Option<bool>) -> Self {
        self.console_access = input;
        self
    }
    /// Appends an item to `groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    pub fn groups(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.groups.unwrap_or_default();
        v.push(input.into());
        self.groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of groups (20 maximum) to which the ActiveMQ user belongs. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    pub fn set_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.groups = input;
        self
    }
    /// <p>The status of the changes pending for the ActiveMQ user.</p>
    pub fn pending(mut self, input: crate::types::UserPendingChanges) -> Self {
        self.pending = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the changes pending for the ActiveMQ user.</p>
    pub fn set_pending(
        mut self,
        input: ::std::option::Option<crate::types::UserPendingChanges>,
    ) -> Self {
        self.pending = input;
        self
    }
    /// <p>Required. The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.username = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Required. The username of the ActiveMQ user. This value can contain only alphanumeric characters, dashes, periods, underscores, and tildes (- . _ ~). This value must be 2-100 characters long.</p>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.username = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeUserOutput`](crate::operation::describe_user::DescribeUserOutput).
    pub fn build(self) -> crate::operation::describe_user::DescribeUserOutput {
        crate::operation::describe_user::DescribeUserOutput {
            broker_id: self.broker_id,
            console_access: self.console_access,
            groups: self.groups,
            pending: self.pending,
            username: self.username,
            _request_id: self._request_id,
        }
    }
}
