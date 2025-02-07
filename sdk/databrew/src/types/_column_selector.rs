// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Selector of a column from a dataset for profile job configuration. One selector includes either a column name or a regular expression.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ColumnSelector {
    /// <p>A regular expression for selecting a column from a dataset.</p>
    #[doc(hidden)]
    pub regex: ::std::option::Option<::std::string::String>,
    /// <p>The name of a column from a dataset.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ColumnSelector {
    /// <p>A regular expression for selecting a column from a dataset.</p>
    pub fn regex(&self) -> ::std::option::Option<&str> {
        self.regex.as_deref()
    }
    /// <p>The name of a column from a dataset.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl ColumnSelector {
    /// Creates a new builder-style object to manufacture [`ColumnSelector`](crate::types::ColumnSelector).
    pub fn builder() -> crate::types::builders::ColumnSelectorBuilder {
        crate::types::builders::ColumnSelectorBuilder::default()
    }
}

/// A builder for [`ColumnSelector`](crate::types::ColumnSelector).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ColumnSelectorBuilder {
    pub(crate) regex: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl ColumnSelectorBuilder {
    /// <p>A regular expression for selecting a column from a dataset.</p>
    pub fn regex(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.regex = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A regular expression for selecting a column from a dataset.</p>
    pub fn set_regex(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.regex = input;
        self
    }
    /// <p>The name of a column from a dataset.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of a column from a dataset.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`ColumnSelector`](crate::types::ColumnSelector).
    pub fn build(self) -> crate::types::ColumnSelector {
        crate::types::ColumnSelector {
            regex: self.regex,
            name: self.name,
        }
    }
}
