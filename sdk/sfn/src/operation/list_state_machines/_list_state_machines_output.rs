// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListStateMachinesOutput {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub state_machines: ::std::option::Option<::std::vec::Vec<crate::types::StateMachineListItem>>,
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListStateMachinesOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn state_machines(&self) -> ::std::option::Option<&[crate::types::StateMachineListItem]> {
        self.state_machines.as_deref()
    }
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListStateMachinesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListStateMachinesOutput {
    /// Creates a new builder-style object to manufacture [`ListStateMachinesOutput`](crate::operation::list_state_machines::ListStateMachinesOutput).
    pub fn builder(
    ) -> crate::operation::list_state_machines::builders::ListStateMachinesOutputBuilder {
        crate::operation::list_state_machines::builders::ListStateMachinesOutputBuilder::default()
    }
}

/// A builder for [`ListStateMachinesOutput`](crate::operation::list_state_machines::ListStateMachinesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListStateMachinesOutputBuilder {
    pub(crate) state_machines:
        ::std::option::Option<::std::vec::Vec<crate::types::StateMachineListItem>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListStateMachinesOutputBuilder {
    /// Appends an item to `state_machines`.
    ///
    /// To override the contents of this collection use [`set_state_machines`](Self::set_state_machines).
    ///
    pub fn state_machines(mut self, input: crate::types::StateMachineListItem) -> Self {
        let mut v = self.state_machines.unwrap_or_default();
        v.push(input);
        self.state_machines = ::std::option::Option::Some(v);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_state_machines(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::StateMachineListItem>>,
    ) -> Self {
        self.state_machines = input;
        self
    }
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p>
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
    /// Consumes the builder and constructs a [`ListStateMachinesOutput`](crate::operation::list_state_machines::ListStateMachinesOutput).
    pub fn build(self) -> crate::operation::list_state_machines::ListStateMachinesOutput {
        crate::operation::list_state_machines::ListStateMachinesOutput {
            state_machines: self.state_machines,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
