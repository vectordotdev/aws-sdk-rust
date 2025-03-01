// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The sort configuration of a tree map.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TreeMapSortConfiguration {
    /// <p>The sort configuration of group by fields.</p>
    #[doc(hidden)]
    pub tree_map_sort: ::std::option::Option<::std::vec::Vec<crate::types::FieldSortOptions>>,
    /// <p>The limit on the number of groups that are displayed.</p>
    #[doc(hidden)]
    pub tree_map_group_items_limit_configuration:
        ::std::option::Option<crate::types::ItemsLimitConfiguration>,
}
impl TreeMapSortConfiguration {
    /// <p>The sort configuration of group by fields.</p>
    pub fn tree_map_sort(&self) -> ::std::option::Option<&[crate::types::FieldSortOptions]> {
        self.tree_map_sort.as_deref()
    }
    /// <p>The limit on the number of groups that are displayed.</p>
    pub fn tree_map_group_items_limit_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::ItemsLimitConfiguration> {
        self.tree_map_group_items_limit_configuration.as_ref()
    }
}
impl TreeMapSortConfiguration {
    /// Creates a new builder-style object to manufacture [`TreeMapSortConfiguration`](crate::types::TreeMapSortConfiguration).
    pub fn builder() -> crate::types::builders::TreeMapSortConfigurationBuilder {
        crate::types::builders::TreeMapSortConfigurationBuilder::default()
    }
}

/// A builder for [`TreeMapSortConfiguration`](crate::types::TreeMapSortConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TreeMapSortConfigurationBuilder {
    pub(crate) tree_map_sort:
        ::std::option::Option<::std::vec::Vec<crate::types::FieldSortOptions>>,
    pub(crate) tree_map_group_items_limit_configuration:
        ::std::option::Option<crate::types::ItemsLimitConfiguration>,
}
impl TreeMapSortConfigurationBuilder {
    /// Appends an item to `tree_map_sort`.
    ///
    /// To override the contents of this collection use [`set_tree_map_sort`](Self::set_tree_map_sort).
    ///
    /// <p>The sort configuration of group by fields.</p>
    pub fn tree_map_sort(mut self, input: crate::types::FieldSortOptions) -> Self {
        let mut v = self.tree_map_sort.unwrap_or_default();
        v.push(input);
        self.tree_map_sort = ::std::option::Option::Some(v);
        self
    }
    /// <p>The sort configuration of group by fields.</p>
    pub fn set_tree_map_sort(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FieldSortOptions>>,
    ) -> Self {
        self.tree_map_sort = input;
        self
    }
    /// <p>The limit on the number of groups that are displayed.</p>
    pub fn tree_map_group_items_limit_configuration(
        mut self,
        input: crate::types::ItemsLimitConfiguration,
    ) -> Self {
        self.tree_map_group_items_limit_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The limit on the number of groups that are displayed.</p>
    pub fn set_tree_map_group_items_limit_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ItemsLimitConfiguration>,
    ) -> Self {
        self.tree_map_group_items_limit_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`TreeMapSortConfiguration`](crate::types::TreeMapSortConfiguration).
    pub fn build(self) -> crate::types::TreeMapSortConfiguration {
        crate::types::TreeMapSortConfiguration {
            tree_map_sort: self.tree_map_sort,
            tree_map_group_items_limit_configuration: self.tree_map_group_items_limit_configuration,
        }
    }
}
