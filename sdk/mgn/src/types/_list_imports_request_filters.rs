// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>List imports request filters.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListImportsRequestFilters {
    /// <p>List imports request filters import IDs.</p>
    #[doc(hidden)]
    pub import_i_ds: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ListImportsRequestFilters {
    /// <p>List imports request filters import IDs.</p>
    pub fn import_i_ds(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.import_i_ds.as_deref()
    }
}
impl ListImportsRequestFilters {
    /// Creates a new builder-style object to manufacture [`ListImportsRequestFilters`](crate::types::ListImportsRequestFilters).
    pub fn builder() -> crate::types::builders::ListImportsRequestFiltersBuilder {
        crate::types::builders::ListImportsRequestFiltersBuilder::default()
    }
}

/// A builder for [`ListImportsRequestFilters`](crate::types::ListImportsRequestFilters).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListImportsRequestFiltersBuilder {
    pub(crate) import_i_ds: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ListImportsRequestFiltersBuilder {
    /// Appends an item to `import_i_ds`.
    ///
    /// To override the contents of this collection use [`set_import_i_ds`](Self::set_import_i_ds).
    ///
    /// <p>List imports request filters import IDs.</p>
    pub fn import_i_ds(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.import_i_ds.unwrap_or_default();
        v.push(input.into());
        self.import_i_ds = ::std::option::Option::Some(v);
        self
    }
    /// <p>List imports request filters import IDs.</p>
    pub fn set_import_i_ds(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.import_i_ds = input;
        self
    }
    /// Consumes the builder and constructs a [`ListImportsRequestFilters`](crate::types::ListImportsRequestFilters).
    pub fn build(self) -> crate::types::ListImportsRequestFilters {
        crate::types::ListImportsRequestFilters {
            import_i_ds: self.import_i_ds,
        }
    }
}
