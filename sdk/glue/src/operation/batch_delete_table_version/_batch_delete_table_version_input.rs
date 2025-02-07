// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchDeleteTableVersionInput {
    /// <p>The ID of the Data Catalog where the tables reside. If none is provided, the Amazon Web Services account ID is used by default.</p>
    #[doc(hidden)]
    pub catalog_id: ::std::option::Option<::std::string::String>,
    /// <p>The database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    #[doc(hidden)]
    pub database_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the table. For Hive compatibility, this name is entirely lowercase.</p>
    #[doc(hidden)]
    pub table_name: ::std::option::Option<::std::string::String>,
    /// <p>A list of the IDs of versions to be deleted. A <code>VersionId</code> is a string representation of an integer. Each version is incremented by 1.</p>
    #[doc(hidden)]
    pub version_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl BatchDeleteTableVersionInput {
    /// <p>The ID of the Data Catalog where the tables reside. If none is provided, the Amazon Web Services account ID is used by default.</p>
    pub fn catalog_id(&self) -> ::std::option::Option<&str> {
        self.catalog_id.as_deref()
    }
    /// <p>The database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    pub fn database_name(&self) -> ::std::option::Option<&str> {
        self.database_name.as_deref()
    }
    /// <p>The name of the table. For Hive compatibility, this name is entirely lowercase.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>A list of the IDs of versions to be deleted. A <code>VersionId</code> is a string representation of an integer. Each version is incremented by 1.</p>
    pub fn version_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.version_ids.as_deref()
    }
}
impl BatchDeleteTableVersionInput {
    /// Creates a new builder-style object to manufacture [`BatchDeleteTableVersionInput`](crate::operation::batch_delete_table_version::BatchDeleteTableVersionInput).
    pub fn builder(
    ) -> crate::operation::batch_delete_table_version::builders::BatchDeleteTableVersionInputBuilder
    {
        crate::operation::batch_delete_table_version::builders::BatchDeleteTableVersionInputBuilder::default()
    }
}

/// A builder for [`BatchDeleteTableVersionInput`](crate::operation::batch_delete_table_version::BatchDeleteTableVersionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchDeleteTableVersionInputBuilder {
    pub(crate) catalog_id: ::std::option::Option<::std::string::String>,
    pub(crate) database_name: ::std::option::Option<::std::string::String>,
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
    pub(crate) version_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl BatchDeleteTableVersionInputBuilder {
    /// <p>The ID of the Data Catalog where the tables reside. If none is provided, the Amazon Web Services account ID is used by default.</p>
    pub fn catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.catalog_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Data Catalog where the tables reside. If none is provided, the Amazon Web Services account ID is used by default.</p>
    pub fn set_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.catalog_id = input;
        self
    }
    /// <p>The database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    pub fn database_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.database_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The database in the catalog in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    pub fn set_database_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.database_name = input;
        self
    }
    /// <p>The name of the table. For Hive compatibility, this name is entirely lowercase.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the table. For Hive compatibility, this name is entirely lowercase.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// Appends an item to `version_ids`.
    ///
    /// To override the contents of this collection use [`set_version_ids`](Self::set_version_ids).
    ///
    /// <p>A list of the IDs of versions to be deleted. A <code>VersionId</code> is a string representation of an integer. Each version is incremented by 1.</p>
    pub fn version_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.version_ids.unwrap_or_default();
        v.push(input.into());
        self.version_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the IDs of versions to be deleted. A <code>VersionId</code> is a string representation of an integer. Each version is incremented by 1.</p>
    pub fn set_version_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.version_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchDeleteTableVersionInput`](crate::operation::batch_delete_table_version::BatchDeleteTableVersionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_delete_table_version::BatchDeleteTableVersionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::batch_delete_table_version::BatchDeleteTableVersionInput {
                catalog_id: self.catalog_id,
                database_name: self.database_name,
                table_name: self.table_name,
                version_ids: self.version_ids,
            },
        )
    }
}
