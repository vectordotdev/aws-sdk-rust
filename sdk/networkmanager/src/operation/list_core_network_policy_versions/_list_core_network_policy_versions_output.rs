// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListCoreNetworkPolicyVersionsOutput {
    /// <p>Describes core network policy versions.</p>
    #[doc(hidden)]
    pub core_network_policy_versions:
        ::std::option::Option<::std::vec::Vec<crate::types::CoreNetworkPolicyVersion>>,
    /// <p>The token for the next page of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListCoreNetworkPolicyVersionsOutput {
    /// <p>Describes core network policy versions.</p>
    pub fn core_network_policy_versions(
        &self,
    ) -> ::std::option::Option<&[crate::types::CoreNetworkPolicyVersion]> {
        self.core_network_policy_versions.as_deref()
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListCoreNetworkPolicyVersionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListCoreNetworkPolicyVersionsOutput {
    /// Creates a new builder-style object to manufacture [`ListCoreNetworkPolicyVersionsOutput`](crate::operation::list_core_network_policy_versions::ListCoreNetworkPolicyVersionsOutput).
    pub fn builder() -> crate::operation::list_core_network_policy_versions::builders::ListCoreNetworkPolicyVersionsOutputBuilder{
        crate::operation::list_core_network_policy_versions::builders::ListCoreNetworkPolicyVersionsOutputBuilder::default()
    }
}

/// A builder for [`ListCoreNetworkPolicyVersionsOutput`](crate::operation::list_core_network_policy_versions::ListCoreNetworkPolicyVersionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListCoreNetworkPolicyVersionsOutputBuilder {
    pub(crate) core_network_policy_versions:
        ::std::option::Option<::std::vec::Vec<crate::types::CoreNetworkPolicyVersion>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListCoreNetworkPolicyVersionsOutputBuilder {
    /// Appends an item to `core_network_policy_versions`.
    ///
    /// To override the contents of this collection use [`set_core_network_policy_versions`](Self::set_core_network_policy_versions).
    ///
    /// <p>Describes core network policy versions.</p>
    pub fn core_network_policy_versions(
        mut self,
        input: crate::types::CoreNetworkPolicyVersion,
    ) -> Self {
        let mut v = self.core_network_policy_versions.unwrap_or_default();
        v.push(input);
        self.core_network_policy_versions = ::std::option::Option::Some(v);
        self
    }
    /// <p>Describes core network policy versions.</p>
    pub fn set_core_network_policy_versions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CoreNetworkPolicyVersion>>,
    ) -> Self {
        self.core_network_policy_versions = input;
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
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
    /// Consumes the builder and constructs a [`ListCoreNetworkPolicyVersionsOutput`](crate::operation::list_core_network_policy_versions::ListCoreNetworkPolicyVersionsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_core_network_policy_versions::ListCoreNetworkPolicyVersionsOutput
    {
        crate::operation::list_core_network_policy_versions::ListCoreNetworkPolicyVersionsOutput {
            core_network_policy_versions: self.core_network_policy_versions,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
