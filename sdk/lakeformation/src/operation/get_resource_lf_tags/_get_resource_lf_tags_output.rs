// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetResourceLfTagsOutput {
    /// <p>A list of LF-tags applied to a database resource.</p>
    #[doc(hidden)]
    pub lf_tag_on_database: ::std::option::Option<::std::vec::Vec<crate::types::LfTagPair>>,
    /// <p>A list of LF-tags applied to a table resource.</p>
    #[doc(hidden)]
    pub lf_tags_on_table: ::std::option::Option<::std::vec::Vec<crate::types::LfTagPair>>,
    /// <p>A list of LF-tags applied to a column resource.</p>
    #[doc(hidden)]
    pub lf_tags_on_columns: ::std::option::Option<::std::vec::Vec<crate::types::ColumnLfTag>>,
    _request_id: Option<String>,
}
impl GetResourceLfTagsOutput {
    /// <p>A list of LF-tags applied to a database resource.</p>
    pub fn lf_tag_on_database(&self) -> ::std::option::Option<&[crate::types::LfTagPair]> {
        self.lf_tag_on_database.as_deref()
    }
    /// <p>A list of LF-tags applied to a table resource.</p>
    pub fn lf_tags_on_table(&self) -> ::std::option::Option<&[crate::types::LfTagPair]> {
        self.lf_tags_on_table.as_deref()
    }
    /// <p>A list of LF-tags applied to a column resource.</p>
    pub fn lf_tags_on_columns(&self) -> ::std::option::Option<&[crate::types::ColumnLfTag]> {
        self.lf_tags_on_columns.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetResourceLfTagsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetResourceLfTagsOutput {
    /// Creates a new builder-style object to manufacture [`GetResourceLfTagsOutput`](crate::operation::get_resource_lf_tags::GetResourceLfTagsOutput).
    pub fn builder(
    ) -> crate::operation::get_resource_lf_tags::builders::GetResourceLfTagsOutputBuilder {
        crate::operation::get_resource_lf_tags::builders::GetResourceLfTagsOutputBuilder::default()
    }
}

/// A builder for [`GetResourceLfTagsOutput`](crate::operation::get_resource_lf_tags::GetResourceLfTagsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetResourceLfTagsOutputBuilder {
    pub(crate) lf_tag_on_database: ::std::option::Option<::std::vec::Vec<crate::types::LfTagPair>>,
    pub(crate) lf_tags_on_table: ::std::option::Option<::std::vec::Vec<crate::types::LfTagPair>>,
    pub(crate) lf_tags_on_columns:
        ::std::option::Option<::std::vec::Vec<crate::types::ColumnLfTag>>,
    _request_id: Option<String>,
}
impl GetResourceLfTagsOutputBuilder {
    /// Appends an item to `lf_tag_on_database`.
    ///
    /// To override the contents of this collection use [`set_lf_tag_on_database`](Self::set_lf_tag_on_database).
    ///
    /// <p>A list of LF-tags applied to a database resource.</p>
    pub fn lf_tag_on_database(mut self, input: crate::types::LfTagPair) -> Self {
        let mut v = self.lf_tag_on_database.unwrap_or_default();
        v.push(input);
        self.lf_tag_on_database = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of LF-tags applied to a database resource.</p>
    pub fn set_lf_tag_on_database(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::LfTagPair>>,
    ) -> Self {
        self.lf_tag_on_database = input;
        self
    }
    /// Appends an item to `lf_tags_on_table`.
    ///
    /// To override the contents of this collection use [`set_lf_tags_on_table`](Self::set_lf_tags_on_table).
    ///
    /// <p>A list of LF-tags applied to a table resource.</p>
    pub fn lf_tags_on_table(mut self, input: crate::types::LfTagPair) -> Self {
        let mut v = self.lf_tags_on_table.unwrap_or_default();
        v.push(input);
        self.lf_tags_on_table = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of LF-tags applied to a table resource.</p>
    pub fn set_lf_tags_on_table(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::LfTagPair>>,
    ) -> Self {
        self.lf_tags_on_table = input;
        self
    }
    /// Appends an item to `lf_tags_on_columns`.
    ///
    /// To override the contents of this collection use [`set_lf_tags_on_columns`](Self::set_lf_tags_on_columns).
    ///
    /// <p>A list of LF-tags applied to a column resource.</p>
    pub fn lf_tags_on_columns(mut self, input: crate::types::ColumnLfTag) -> Self {
        let mut v = self.lf_tags_on_columns.unwrap_or_default();
        v.push(input);
        self.lf_tags_on_columns = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of LF-tags applied to a column resource.</p>
    pub fn set_lf_tags_on_columns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ColumnLfTag>>,
    ) -> Self {
        self.lf_tags_on_columns = input;
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
    /// Consumes the builder and constructs a [`GetResourceLfTagsOutput`](crate::operation::get_resource_lf_tags::GetResourceLfTagsOutput).
    pub fn build(self) -> crate::operation::get_resource_lf_tags::GetResourceLfTagsOutput {
        crate::operation::get_resource_lf_tags::GetResourceLfTagsOutput {
            lf_tag_on_database: self.lf_tag_on_database,
            lf_tags_on_table: self.lf_tags_on_table,
            lf_tags_on_columns: self.lf_tags_on_columns,
            _request_id: self._request_id,
        }
    }
}
