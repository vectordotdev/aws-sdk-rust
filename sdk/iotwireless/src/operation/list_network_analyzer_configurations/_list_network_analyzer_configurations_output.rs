// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListNetworkAnalyzerConfigurationsOutput {
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The list of network analyzer configurations.</p>
    #[doc(hidden)]
    pub network_analyzer_configuration_list:
        ::std::option::Option<::std::vec::Vec<crate::types::NetworkAnalyzerConfigurations>>,
    _request_id: Option<String>,
}
impl ListNetworkAnalyzerConfigurationsOutput {
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The list of network analyzer configurations.</p>
    pub fn network_analyzer_configuration_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::NetworkAnalyzerConfigurations]> {
        self.network_analyzer_configuration_list.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListNetworkAnalyzerConfigurationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListNetworkAnalyzerConfigurationsOutput {
    /// Creates a new builder-style object to manufacture [`ListNetworkAnalyzerConfigurationsOutput`](crate::operation::list_network_analyzer_configurations::ListNetworkAnalyzerConfigurationsOutput).
    pub fn builder() -> crate::operation::list_network_analyzer_configurations::builders::ListNetworkAnalyzerConfigurationsOutputBuilder{
        crate::operation::list_network_analyzer_configurations::builders::ListNetworkAnalyzerConfigurationsOutputBuilder::default()
    }
}

/// A builder for [`ListNetworkAnalyzerConfigurationsOutput`](crate::operation::list_network_analyzer_configurations::ListNetworkAnalyzerConfigurationsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListNetworkAnalyzerConfigurationsOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) network_analyzer_configuration_list:
        ::std::option::Option<::std::vec::Vec<crate::types::NetworkAnalyzerConfigurations>>,
    _request_id: Option<String>,
}
impl ListNetworkAnalyzerConfigurationsOutputBuilder {
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `network_analyzer_configuration_list`.
    ///
    /// To override the contents of this collection use [`set_network_analyzer_configuration_list`](Self::set_network_analyzer_configuration_list).
    ///
    /// <p>The list of network analyzer configurations.</p>
    pub fn network_analyzer_configuration_list(
        mut self,
        input: crate::types::NetworkAnalyzerConfigurations,
    ) -> Self {
        let mut v = self.network_analyzer_configuration_list.unwrap_or_default();
        v.push(input);
        self.network_analyzer_configuration_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of network analyzer configurations.</p>
    pub fn set_network_analyzer_configuration_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::NetworkAnalyzerConfigurations>>,
    ) -> Self {
        self.network_analyzer_configuration_list = input;
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
    /// Consumes the builder and constructs a [`ListNetworkAnalyzerConfigurationsOutput`](crate::operation::list_network_analyzer_configurations::ListNetworkAnalyzerConfigurationsOutput).
    pub fn build(self) -> crate::operation::list_network_analyzer_configurations::ListNetworkAnalyzerConfigurationsOutput{
        crate::operation::list_network_analyzer_configurations::ListNetworkAnalyzerConfigurationsOutput {
            next_token: self.next_token
            ,
            network_analyzer_configuration_list: self.network_analyzer_configuration_list
            ,
            _request_id: self._request_id,
        }
    }
}
