// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A transform operation that tags a column with additional information.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TagColumnOperation {
    /// <p>The column that this operation acts on.</p>
    #[doc(hidden)]
    pub column_name: ::std::option::Option<::std::string::String>,
    /// <p>The dataset column tag, currently only used for geospatial type tagging.</p> <note>
    /// <p>This is not tags for the Amazon Web Services tagging feature.</p>
    /// </note>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::ColumnTag>>,
}
impl TagColumnOperation {
    /// <p>The column that this operation acts on.</p>
    pub fn column_name(&self) -> ::std::option::Option<&str> {
        self.column_name.as_deref()
    }
    /// <p>The dataset column tag, currently only used for geospatial type tagging.</p> <note>
    /// <p>This is not tags for the Amazon Web Services tagging feature.</p>
    /// </note>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::ColumnTag]> {
        self.tags.as_deref()
    }
}
impl TagColumnOperation {
    /// Creates a new builder-style object to manufacture [`TagColumnOperation`](crate::types::TagColumnOperation).
    pub fn builder() -> crate::types::builders::TagColumnOperationBuilder {
        crate::types::builders::TagColumnOperationBuilder::default()
    }
}

/// A builder for [`TagColumnOperation`](crate::types::TagColumnOperation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TagColumnOperationBuilder {
    pub(crate) column_name: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::ColumnTag>>,
}
impl TagColumnOperationBuilder {
    /// <p>The column that this operation acts on.</p>
    pub fn column_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.column_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The column that this operation acts on.</p>
    pub fn set_column_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.column_name = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The dataset column tag, currently only used for geospatial type tagging.</p> <note>
    /// <p>This is not tags for the Amazon Web Services tagging feature.</p>
    /// </note>
    pub fn tags(mut self, input: crate::types::ColumnTag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The dataset column tag, currently only used for geospatial type tagging.</p> <note>
    /// <p>This is not tags for the Amazon Web Services tagging feature.</p>
    /// </note>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ColumnTag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`TagColumnOperation`](crate::types::TagColumnOperation).
    pub fn build(self) -> crate::types::TagColumnOperation {
        crate::types::TagColumnOperation {
            column_name: self.column_name,
            tags: self.tags,
        }
    }
}
