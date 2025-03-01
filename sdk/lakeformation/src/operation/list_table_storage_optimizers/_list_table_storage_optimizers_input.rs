// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTableStorageOptimizersInput {
    /// <p>The Catalog ID of the table.</p>
    #[doc(hidden)]
    pub catalog_id: ::std::option::Option<::std::string::String>,
    /// <p>Name of the database where the table is present.</p>
    #[doc(hidden)]
    pub database_name: ::std::option::Option<::std::string::String>,
    /// <p>Name of the table.</p>
    #[doc(hidden)]
    pub table_name: ::std::option::Option<::std::string::String>,
    /// <p>The specific type of storage optimizers to list. The supported value is <code>compaction</code>.</p>
    #[doc(hidden)]
    pub storage_optimizer_type: ::std::option::Option<crate::types::OptimizerType>,
    /// <p>The number of storage optimizers to return on each call.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>A continuation token, if this is a continuation call.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListTableStorageOptimizersInput {
    /// <p>The Catalog ID of the table.</p>
    pub fn catalog_id(&self) -> ::std::option::Option<&str> {
        self.catalog_id.as_deref()
    }
    /// <p>Name of the database where the table is present.</p>
    pub fn database_name(&self) -> ::std::option::Option<&str> {
        self.database_name.as_deref()
    }
    /// <p>Name of the table.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>The specific type of storage optimizers to list. The supported value is <code>compaction</code>.</p>
    pub fn storage_optimizer_type(&self) -> ::std::option::Option<&crate::types::OptimizerType> {
        self.storage_optimizer_type.as_ref()
    }
    /// <p>The number of storage optimizers to return on each call.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>A continuation token, if this is a continuation call.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListTableStorageOptimizersInput {
    /// Creates a new builder-style object to manufacture [`ListTableStorageOptimizersInput`](crate::operation::list_table_storage_optimizers::ListTableStorageOptimizersInput).
    pub fn builder() -> crate::operation::list_table_storage_optimizers::builders::ListTableStorageOptimizersInputBuilder{
        crate::operation::list_table_storage_optimizers::builders::ListTableStorageOptimizersInputBuilder::default()
    }
}

/// A builder for [`ListTableStorageOptimizersInput`](crate::operation::list_table_storage_optimizers::ListTableStorageOptimizersInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListTableStorageOptimizersInputBuilder {
    pub(crate) catalog_id: ::std::option::Option<::std::string::String>,
    pub(crate) database_name: ::std::option::Option<::std::string::String>,
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
    pub(crate) storage_optimizer_type: ::std::option::Option<crate::types::OptimizerType>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListTableStorageOptimizersInputBuilder {
    /// <p>The Catalog ID of the table.</p>
    pub fn catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.catalog_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Catalog ID of the table.</p>
    pub fn set_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.catalog_id = input;
        self
    }
    /// <p>Name of the database where the table is present.</p>
    pub fn database_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.database_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the database where the table is present.</p>
    pub fn set_database_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.database_name = input;
        self
    }
    /// <p>Name of the table.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the table.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>The specific type of storage optimizers to list. The supported value is <code>compaction</code>.</p>
    pub fn storage_optimizer_type(mut self, input: crate::types::OptimizerType) -> Self {
        self.storage_optimizer_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The specific type of storage optimizers to list. The supported value is <code>compaction</code>.</p>
    pub fn set_storage_optimizer_type(
        mut self,
        input: ::std::option::Option<crate::types::OptimizerType>,
    ) -> Self {
        self.storage_optimizer_type = input;
        self
    }
    /// <p>The number of storage optimizers to return on each call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of storage optimizers to return on each call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>A continuation token, if this is a continuation call.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A continuation token, if this is a continuation call.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListTableStorageOptimizersInput`](crate::operation::list_table_storage_optimizers::ListTableStorageOptimizersInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_table_storage_optimizers::ListTableStorageOptimizersInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_table_storage_optimizers::ListTableStorageOptimizersInput {
                catalog_id: self.catalog_id,
                database_name: self.database_name,
                table_name: self.table_name,
                storage_optimizer_type: self.storage_optimizer_type,
                max_results: self.max_results,
                next_token: self.next_token,
            },
        )
    }
}
