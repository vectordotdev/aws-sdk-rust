// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTemplateActionsOutput {
    /// <p>List of template post migration custom actions.</p>
    #[doc(hidden)]
    pub items: ::std::option::Option<::std::vec::Vec<crate::types::TemplateActionDocument>>,
    /// <p>Next token returned when listing template post migration custom actions.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTemplateActionsOutput {
    /// <p>List of template post migration custom actions.</p>
    pub fn items(&self) -> ::std::option::Option<&[crate::types::TemplateActionDocument]> {
        self.items.as_deref()
    }
    /// <p>Next token returned when listing template post migration custom actions.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListTemplateActionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTemplateActionsOutput {
    /// Creates a new builder-style object to manufacture [`ListTemplateActionsOutput`](crate::operation::list_template_actions::ListTemplateActionsOutput).
    pub fn builder(
    ) -> crate::operation::list_template_actions::builders::ListTemplateActionsOutputBuilder {
        crate::operation::list_template_actions::builders::ListTemplateActionsOutputBuilder::default(
        )
    }
}

/// A builder for [`ListTemplateActionsOutput`](crate::operation::list_template_actions::ListTemplateActionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListTemplateActionsOutputBuilder {
    pub(crate) items: ::std::option::Option<::std::vec::Vec<crate::types::TemplateActionDocument>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListTemplateActionsOutputBuilder {
    /// Appends an item to `items`.
    ///
    /// To override the contents of this collection use [`set_items`](Self::set_items).
    ///
    /// <p>List of template post migration custom actions.</p>
    pub fn items(mut self, input: crate::types::TemplateActionDocument) -> Self {
        let mut v = self.items.unwrap_or_default();
        v.push(input);
        self.items = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of template post migration custom actions.</p>
    pub fn set_items(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TemplateActionDocument>>,
    ) -> Self {
        self.items = input;
        self
    }
    /// <p>Next token returned when listing template post migration custom actions.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Next token returned when listing template post migration custom actions.</p>
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
    /// Consumes the builder and constructs a [`ListTemplateActionsOutput`](crate::operation::list_template_actions::ListTemplateActionsOutput).
    pub fn build(self) -> crate::operation::list_template_actions::ListTemplateActionsOutput {
        crate::operation::list_template_actions::ListTemplateActionsOutput {
            items: self.items,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
