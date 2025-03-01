// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListInstancesOutput {
    /// <p>Information about the instances.</p>
    #[doc(hidden)]
    pub instance_summary_list:
        ::std::option::Option<::std::vec::Vec<crate::types::InstanceSummary>>,
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListInstancesOutput {
    /// <p>Information about the instances.</p>
    pub fn instance_summary_list(&self) -> ::std::option::Option<&[crate::types::InstanceSummary]> {
        self.instance_summary_list.as_deref()
    }
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListInstancesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListInstancesOutput {
    /// Creates a new builder-style object to manufacture [`ListInstancesOutput`](crate::operation::list_instances::ListInstancesOutput).
    pub fn builder() -> crate::operation::list_instances::builders::ListInstancesOutputBuilder {
        crate::operation::list_instances::builders::ListInstancesOutputBuilder::default()
    }
}

/// A builder for [`ListInstancesOutput`](crate::operation::list_instances::ListInstancesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListInstancesOutputBuilder {
    pub(crate) instance_summary_list:
        ::std::option::Option<::std::vec::Vec<crate::types::InstanceSummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListInstancesOutputBuilder {
    /// Appends an item to `instance_summary_list`.
    ///
    /// To override the contents of this collection use [`set_instance_summary_list`](Self::set_instance_summary_list).
    ///
    /// <p>Information about the instances.</p>
    pub fn instance_summary_list(mut self, input: crate::types::InstanceSummary) -> Self {
        let mut v = self.instance_summary_list.unwrap_or_default();
        v.push(input);
        self.instance_summary_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the instances.</p>
    pub fn set_instance_summary_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceSummary>>,
    ) -> Self {
        self.instance_summary_list = input;
        self
    }
    /// <p>If there are additional results, this is the token for the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If there are additional results, this is the token for the next set of results.</p>
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
    /// Consumes the builder and constructs a [`ListInstancesOutput`](crate::operation::list_instances::ListInstancesOutput).
    pub fn build(self) -> crate::operation::list_instances::ListInstancesOutput {
        crate::operation::list_instances::ListInstancesOutput {
            instance_summary_list: self.instance_summary_list,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
