// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The response to the request.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListAnalyzersOutput {
    /// <p>The analyzers retrieved.</p>
    #[doc(hidden)]
    pub analyzers: ::std::option::Option<::std::vec::Vec<crate::types::AnalyzerSummary>>,
    /// <p>A token used for pagination of results returned.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListAnalyzersOutput {
    /// <p>The analyzers retrieved.</p>
    pub fn analyzers(&self) -> ::std::option::Option<&[crate::types::AnalyzerSummary]> {
        self.analyzers.as_deref()
    }
    /// <p>A token used for pagination of results returned.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListAnalyzersOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListAnalyzersOutput {
    /// Creates a new builder-style object to manufacture [`ListAnalyzersOutput`](crate::operation::list_analyzers::ListAnalyzersOutput).
    pub fn builder() -> crate::operation::list_analyzers::builders::ListAnalyzersOutputBuilder {
        crate::operation::list_analyzers::builders::ListAnalyzersOutputBuilder::default()
    }
}

/// A builder for [`ListAnalyzersOutput`](crate::operation::list_analyzers::ListAnalyzersOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListAnalyzersOutputBuilder {
    pub(crate) analyzers: ::std::option::Option<::std::vec::Vec<crate::types::AnalyzerSummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListAnalyzersOutputBuilder {
    /// Appends an item to `analyzers`.
    ///
    /// To override the contents of this collection use [`set_analyzers`](Self::set_analyzers).
    ///
    /// <p>The analyzers retrieved.</p>
    pub fn analyzers(mut self, input: crate::types::AnalyzerSummary) -> Self {
        let mut v = self.analyzers.unwrap_or_default();
        v.push(input);
        self.analyzers = ::std::option::Option::Some(v);
        self
    }
    /// <p>The analyzers retrieved.</p>
    pub fn set_analyzers(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AnalyzerSummary>>,
    ) -> Self {
        self.analyzers = input;
        self
    }
    /// <p>A token used for pagination of results returned.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token used for pagination of results returned.</p>
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
    /// Consumes the builder and constructs a [`ListAnalyzersOutput`](crate::operation::list_analyzers::ListAnalyzersOutput).
    pub fn build(self) -> crate::operation::list_analyzers::ListAnalyzersOutput {
        crate::operation::list_analyzers::ListAnalyzersOutput {
            analyzers: self.analyzers,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
