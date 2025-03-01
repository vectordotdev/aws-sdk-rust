// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A structure for a data cells filter resource. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DataCellsFilterResource {
    /// <p>The ID of the catalog to which the table belongs.</p>
    #[doc(hidden)]
    pub table_catalog_id: ::std::option::Option<::std::string::String>,
    /// <p>A database in the Glue Data Catalog.</p>
    #[doc(hidden)]
    pub database_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the table.</p>
    #[doc(hidden)]
    pub table_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the data cells filter. </p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl DataCellsFilterResource {
    /// <p>The ID of the catalog to which the table belongs.</p>
    pub fn table_catalog_id(&self) -> ::std::option::Option<&str> {
        self.table_catalog_id.as_deref()
    }
    /// <p>A database in the Glue Data Catalog.</p>
    pub fn database_name(&self) -> ::std::option::Option<&str> {
        self.database_name.as_deref()
    }
    /// <p>The name of the table.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>The name of the data cells filter. </p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl DataCellsFilterResource {
    /// Creates a new builder-style object to manufacture [`DataCellsFilterResource`](crate::types::DataCellsFilterResource).
    pub fn builder() -> crate::types::builders::DataCellsFilterResourceBuilder {
        crate::types::builders::DataCellsFilterResourceBuilder::default()
    }
}

/// A builder for [`DataCellsFilterResource`](crate::types::DataCellsFilterResource).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DataCellsFilterResourceBuilder {
    pub(crate) table_catalog_id: ::std::option::Option<::std::string::String>,
    pub(crate) database_name: ::std::option::Option<::std::string::String>,
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl DataCellsFilterResourceBuilder {
    /// <p>The ID of the catalog to which the table belongs.</p>
    pub fn table_catalog_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.table_catalog_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the catalog to which the table belongs.</p>
    pub fn set_table_catalog_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.table_catalog_id = input;
        self
    }
    /// <p>A database in the Glue Data Catalog.</p>
    pub fn database_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.database_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A database in the Glue Data Catalog.</p>
    pub fn set_database_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.database_name = input;
        self
    }
    /// <p>The name of the table.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the table.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>The name of the data cells filter. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the data cells filter. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`DataCellsFilterResource`](crate::types::DataCellsFilterResource).
    pub fn build(self) -> crate::types::DataCellsFilterResource {
        crate::types::DataCellsFilterResource {
            table_catalog_id: self.table_catalog_id,
            database_name: self.database_name,
            table_name: self.table_name,
            name: self.name,
        }
    }
}
