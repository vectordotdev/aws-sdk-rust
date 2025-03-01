// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListEventConfigurationsOutput {
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>Event configurations of all events for a single resource.</p>
    #[doc(hidden)]
    pub event_configurations_list:
        ::std::option::Option<::std::vec::Vec<crate::types::EventConfigurationItem>>,
    _request_id: Option<String>,
}
impl ListEventConfigurationsOutput {
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Event configurations of all events for a single resource.</p>
    pub fn event_configurations_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::EventConfigurationItem]> {
        self.event_configurations_list.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListEventConfigurationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListEventConfigurationsOutput {
    /// Creates a new builder-style object to manufacture [`ListEventConfigurationsOutput`](crate::operation::list_event_configurations::ListEventConfigurationsOutput).
    pub fn builder(
    ) -> crate::operation::list_event_configurations::builders::ListEventConfigurationsOutputBuilder
    {
        crate::operation::list_event_configurations::builders::ListEventConfigurationsOutputBuilder::default()
    }
}

/// A builder for [`ListEventConfigurationsOutput`](crate::operation::list_event_configurations::ListEventConfigurationsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListEventConfigurationsOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) event_configurations_list:
        ::std::option::Option<::std::vec::Vec<crate::types::EventConfigurationItem>>,
    _request_id: Option<String>,
}
impl ListEventConfigurationsOutputBuilder {
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `event_configurations_list`.
    ///
    /// To override the contents of this collection use [`set_event_configurations_list`](Self::set_event_configurations_list).
    ///
    /// <p>Event configurations of all events for a single resource.</p>
    pub fn event_configurations_list(
        mut self,
        input: crate::types::EventConfigurationItem,
    ) -> Self {
        let mut v = self.event_configurations_list.unwrap_or_default();
        v.push(input);
        self.event_configurations_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>Event configurations of all events for a single resource.</p>
    pub fn set_event_configurations_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EventConfigurationItem>>,
    ) -> Self {
        self.event_configurations_list = input;
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
    /// Consumes the builder and constructs a [`ListEventConfigurationsOutput`](crate::operation::list_event_configurations::ListEventConfigurationsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_event_configurations::ListEventConfigurationsOutput {
        crate::operation::list_event_configurations::ListEventConfigurationsOutput {
            next_token: self.next_token,
            event_configurations_list: self.event_configurations_list,
            _request_id: self._request_id,
        }
    }
}
