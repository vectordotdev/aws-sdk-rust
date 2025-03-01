// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListCustomPluginsOutput {
    /// <p>An array of custom plugin descriptions.</p>
    #[doc(hidden)]
    pub custom_plugins: ::std::option::Option<::std::vec::Vec<crate::types::CustomPluginSummary>>,
    /// <p>If the response of a ListCustomPlugins operation is truncated, it will include a NextToken. Send this NextToken in a subsequent request to continue listing from where the previous operation left off.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListCustomPluginsOutput {
    /// <p>An array of custom plugin descriptions.</p>
    pub fn custom_plugins(&self) -> ::std::option::Option<&[crate::types::CustomPluginSummary]> {
        self.custom_plugins.as_deref()
    }
    /// <p>If the response of a ListCustomPlugins operation is truncated, it will include a NextToken. Send this NextToken in a subsequent request to continue listing from where the previous operation left off.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListCustomPluginsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListCustomPluginsOutput {
    /// Creates a new builder-style object to manufacture [`ListCustomPluginsOutput`](crate::operation::list_custom_plugins::ListCustomPluginsOutput).
    pub fn builder(
    ) -> crate::operation::list_custom_plugins::builders::ListCustomPluginsOutputBuilder {
        crate::operation::list_custom_plugins::builders::ListCustomPluginsOutputBuilder::default()
    }
}

/// A builder for [`ListCustomPluginsOutput`](crate::operation::list_custom_plugins::ListCustomPluginsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListCustomPluginsOutputBuilder {
    pub(crate) custom_plugins:
        ::std::option::Option<::std::vec::Vec<crate::types::CustomPluginSummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListCustomPluginsOutputBuilder {
    /// Appends an item to `custom_plugins`.
    ///
    /// To override the contents of this collection use [`set_custom_plugins`](Self::set_custom_plugins).
    ///
    /// <p>An array of custom plugin descriptions.</p>
    pub fn custom_plugins(mut self, input: crate::types::CustomPluginSummary) -> Self {
        let mut v = self.custom_plugins.unwrap_or_default();
        v.push(input);
        self.custom_plugins = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of custom plugin descriptions.</p>
    pub fn set_custom_plugins(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CustomPluginSummary>>,
    ) -> Self {
        self.custom_plugins = input;
        self
    }
    /// <p>If the response of a ListCustomPlugins operation is truncated, it will include a NextToken. Send this NextToken in a subsequent request to continue listing from where the previous operation left off.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the response of a ListCustomPlugins operation is truncated, it will include a NextToken. Send this NextToken in a subsequent request to continue listing from where the previous operation left off.</p>
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
    /// Consumes the builder and constructs a [`ListCustomPluginsOutput`](crate::operation::list_custom_plugins::ListCustomPluginsOutput).
    pub fn build(self) -> crate::operation::list_custom_plugins::ListCustomPluginsOutput {
        crate::operation::list_custom_plugins::ListCustomPluginsOutput {
            custom_plugins: self.custom_plugins,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
