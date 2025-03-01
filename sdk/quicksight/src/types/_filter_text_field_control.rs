// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A control to display a text box that is used to enter a single entry.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FilterTextFieldControl {
    /// <p>The ID of the <code>FilterTextFieldControl</code>.</p>
    #[doc(hidden)]
    pub filter_control_id: ::std::option::Option<::std::string::String>,
    /// <p>The title of the <code>FilterTextFieldControl</code>.</p>
    #[doc(hidden)]
    pub title: ::std::option::Option<::std::string::String>,
    /// <p>The source filter ID of the <code>FilterTextFieldControl</code>.</p>
    #[doc(hidden)]
    pub source_filter_id: ::std::option::Option<::std::string::String>,
    /// <p>The display options of a control.</p>
    #[doc(hidden)]
    pub display_options: ::std::option::Option<crate::types::TextFieldControlDisplayOptions>,
}
impl FilterTextFieldControl {
    /// <p>The ID of the <code>FilterTextFieldControl</code>.</p>
    pub fn filter_control_id(&self) -> ::std::option::Option<&str> {
        self.filter_control_id.as_deref()
    }
    /// <p>The title of the <code>FilterTextFieldControl</code>.</p>
    pub fn title(&self) -> ::std::option::Option<&str> {
        self.title.as_deref()
    }
    /// <p>The source filter ID of the <code>FilterTextFieldControl</code>.</p>
    pub fn source_filter_id(&self) -> ::std::option::Option<&str> {
        self.source_filter_id.as_deref()
    }
    /// <p>The display options of a control.</p>
    pub fn display_options(
        &self,
    ) -> ::std::option::Option<&crate::types::TextFieldControlDisplayOptions> {
        self.display_options.as_ref()
    }
}
impl FilterTextFieldControl {
    /// Creates a new builder-style object to manufacture [`FilterTextFieldControl`](crate::types::FilterTextFieldControl).
    pub fn builder() -> crate::types::builders::FilterTextFieldControlBuilder {
        crate::types::builders::FilterTextFieldControlBuilder::default()
    }
}

/// A builder for [`FilterTextFieldControl`](crate::types::FilterTextFieldControl).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FilterTextFieldControlBuilder {
    pub(crate) filter_control_id: ::std::option::Option<::std::string::String>,
    pub(crate) title: ::std::option::Option<::std::string::String>,
    pub(crate) source_filter_id: ::std::option::Option<::std::string::String>,
    pub(crate) display_options: ::std::option::Option<crate::types::TextFieldControlDisplayOptions>,
}
impl FilterTextFieldControlBuilder {
    /// <p>The ID of the <code>FilterTextFieldControl</code>.</p>
    pub fn filter_control_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.filter_control_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the <code>FilterTextFieldControl</code>.</p>
    pub fn set_filter_control_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.filter_control_id = input;
        self
    }
    /// <p>The title of the <code>FilterTextFieldControl</code>.</p>
    pub fn title(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.title = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The title of the <code>FilterTextFieldControl</code>.</p>
    pub fn set_title(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.title = input;
        self
    }
    /// <p>The source filter ID of the <code>FilterTextFieldControl</code>.</p>
    pub fn source_filter_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.source_filter_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source filter ID of the <code>FilterTextFieldControl</code>.</p>
    pub fn set_source_filter_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.source_filter_id = input;
        self
    }
    /// <p>The display options of a control.</p>
    pub fn display_options(mut self, input: crate::types::TextFieldControlDisplayOptions) -> Self {
        self.display_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The display options of a control.</p>
    pub fn set_display_options(
        mut self,
        input: ::std::option::Option<crate::types::TextFieldControlDisplayOptions>,
    ) -> Self {
        self.display_options = input;
        self
    }
    /// Consumes the builder and constructs a [`FilterTextFieldControl`](crate::types::FilterTextFieldControl).
    pub fn build(self) -> crate::types::FilterTextFieldControl {
        crate::types::FilterTextFieldControl {
            filter_control_id: self.filter_control_id,
            title: self.title,
            source_filter_id: self.source_filter_id,
            display_options: self.display_options,
        }
    }
}
