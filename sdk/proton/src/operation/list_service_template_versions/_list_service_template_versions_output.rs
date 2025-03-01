// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListServiceTemplateVersionsOutput {
    /// <p>A token that indicates the location of the next major or minor version in the array of major or minor versions of a service template, after the current requested list of service major or minor versions.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>An array of major or minor versions of a service template with detail data.</p>
    #[doc(hidden)]
    pub template_versions:
        ::std::option::Option<::std::vec::Vec<crate::types::ServiceTemplateVersionSummary>>,
    _request_id: Option<String>,
}
impl ListServiceTemplateVersionsOutput {
    /// <p>A token that indicates the location of the next major or minor version in the array of major or minor versions of a service template, after the current requested list of service major or minor versions.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>An array of major or minor versions of a service template with detail data.</p>
    pub fn template_versions(
        &self,
    ) -> ::std::option::Option<&[crate::types::ServiceTemplateVersionSummary]> {
        self.template_versions.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListServiceTemplateVersionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListServiceTemplateVersionsOutput {
    /// Creates a new builder-style object to manufacture [`ListServiceTemplateVersionsOutput`](crate::operation::list_service_template_versions::ListServiceTemplateVersionsOutput).
    pub fn builder() -> crate::operation::list_service_template_versions::builders::ListServiceTemplateVersionsOutputBuilder{
        crate::operation::list_service_template_versions::builders::ListServiceTemplateVersionsOutputBuilder::default()
    }
}

/// A builder for [`ListServiceTemplateVersionsOutput`](crate::operation::list_service_template_versions::ListServiceTemplateVersionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListServiceTemplateVersionsOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) template_versions:
        ::std::option::Option<::std::vec::Vec<crate::types::ServiceTemplateVersionSummary>>,
    _request_id: Option<String>,
}
impl ListServiceTemplateVersionsOutputBuilder {
    /// <p>A token that indicates the location of the next major or minor version in the array of major or minor versions of a service template, after the current requested list of service major or minor versions.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token that indicates the location of the next major or minor version in the array of major or minor versions of a service template, after the current requested list of service major or minor versions.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `template_versions`.
    ///
    /// To override the contents of this collection use [`set_template_versions`](Self::set_template_versions).
    ///
    /// <p>An array of major or minor versions of a service template with detail data.</p>
    pub fn template_versions(mut self, input: crate::types::ServiceTemplateVersionSummary) -> Self {
        let mut v = self.template_versions.unwrap_or_default();
        v.push(input);
        self.template_versions = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of major or minor versions of a service template with detail data.</p>
    pub fn set_template_versions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ServiceTemplateVersionSummary>>,
    ) -> Self {
        self.template_versions = input;
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
    /// Consumes the builder and constructs a [`ListServiceTemplateVersionsOutput`](crate::operation::list_service_template_versions::ListServiceTemplateVersionsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_service_template_versions::ListServiceTemplateVersionsOutput {
        crate::operation::list_service_template_versions::ListServiceTemplateVersionsOutput {
            next_token: self.next_token,
            template_versions: self.template_versions,
            _request_id: self._request_id,
        }
    }
}
