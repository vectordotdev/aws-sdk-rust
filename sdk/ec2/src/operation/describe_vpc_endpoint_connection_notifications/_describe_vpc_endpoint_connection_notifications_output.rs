// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeVpcEndpointConnectionNotificationsOutput {
    /// <p>The notifications.</p>
    #[doc(hidden)]
    pub connection_notification_set:
        ::std::option::Option<::std::vec::Vec<crate::types::ConnectionNotification>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeVpcEndpointConnectionNotificationsOutput {
    /// <p>The notifications.</p>
    pub fn connection_notification_set(
        &self,
    ) -> ::std::option::Option<&[crate::types::ConnectionNotification]> {
        self.connection_notification_set.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeVpcEndpointConnectionNotificationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeVpcEndpointConnectionNotificationsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeVpcEndpointConnectionNotificationsOutput`](crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsOutput).
    pub fn builder() -> crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsOutputBuilder{
        crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsOutputBuilder::default()
    }
}

/// A builder for [`DescribeVpcEndpointConnectionNotificationsOutput`](crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeVpcEndpointConnectionNotificationsOutputBuilder {
    pub(crate) connection_notification_set:
        ::std::option::Option<::std::vec::Vec<crate::types::ConnectionNotification>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeVpcEndpointConnectionNotificationsOutputBuilder {
    /// Appends an item to `connection_notification_set`.
    ///
    /// To override the contents of this collection use [`set_connection_notification_set`](Self::set_connection_notification_set).
    ///
    /// <p>The notifications.</p>
    pub fn connection_notification_set(
        mut self,
        input: crate::types::ConnectionNotification,
    ) -> Self {
        let mut v = self.connection_notification_set.unwrap_or_default();
        v.push(input);
        self.connection_notification_set = ::std::option::Option::Some(v);
        self
    }
    /// <p>The notifications.</p>
    pub fn set_connection_notification_set(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ConnectionNotification>>,
    ) -> Self {
        self.connection_notification_set = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`DescribeVpcEndpointConnectionNotificationsOutput`](crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsOutput).
    pub fn build(self) -> crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsOutput{
        crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsOutput {
            connection_notification_set: self.connection_notification_set
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
