// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A list of CloudFront functions.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FunctionList {
    /// <p>If there are more items in the list than are in this response, this element is present. It contains the value that you should use in the <code>Marker</code> field of a subsequent request to continue listing functions where you left off.</p>
    #[doc(hidden)]
    pub next_marker: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of functions requested.</p>
    #[doc(hidden)]
    pub max_items: ::std::option::Option<i32>,
    /// <p>The number of functions returned in the response.</p>
    #[doc(hidden)]
    pub quantity: ::std::option::Option<i32>,
    /// <p>Contains the functions in the list.</p>
    #[doc(hidden)]
    pub items: ::std::option::Option<::std::vec::Vec<crate::types::FunctionSummary>>,
}
impl FunctionList {
    /// <p>If there are more items in the list than are in this response, this element is present. It contains the value that you should use in the <code>Marker</code> field of a subsequent request to continue listing functions where you left off.</p>
    pub fn next_marker(&self) -> ::std::option::Option<&str> {
        self.next_marker.as_deref()
    }
    /// <p>The maximum number of functions requested.</p>
    pub fn max_items(&self) -> ::std::option::Option<i32> {
        self.max_items
    }
    /// <p>The number of functions returned in the response.</p>
    pub fn quantity(&self) -> ::std::option::Option<i32> {
        self.quantity
    }
    /// <p>Contains the functions in the list.</p>
    pub fn items(&self) -> ::std::option::Option<&[crate::types::FunctionSummary]> {
        self.items.as_deref()
    }
}
impl FunctionList {
    /// Creates a new builder-style object to manufacture [`FunctionList`](crate::types::FunctionList).
    pub fn builder() -> crate::types::builders::FunctionListBuilder {
        crate::types::builders::FunctionListBuilder::default()
    }
}

/// A builder for [`FunctionList`](crate::types::FunctionList).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FunctionListBuilder {
    pub(crate) next_marker: ::std::option::Option<::std::string::String>,
    pub(crate) max_items: ::std::option::Option<i32>,
    pub(crate) quantity: ::std::option::Option<i32>,
    pub(crate) items: ::std::option::Option<::std::vec::Vec<crate::types::FunctionSummary>>,
}
impl FunctionListBuilder {
    /// <p>If there are more items in the list than are in this response, this element is present. It contains the value that you should use in the <code>Marker</code> field of a subsequent request to continue listing functions where you left off.</p>
    pub fn next_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If there are more items in the list than are in this response, this element is present. It contains the value that you should use in the <code>Marker</code> field of a subsequent request to continue listing functions where you left off.</p>
    pub fn set_next_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_marker = input;
        self
    }
    /// <p>The maximum number of functions requested.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.max_items = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of functions requested.</p>
    pub fn set_max_items(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_items = input;
        self
    }
    /// <p>The number of functions returned in the response.</p>
    pub fn quantity(mut self, input: i32) -> Self {
        self.quantity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of functions returned in the response.</p>
    pub fn set_quantity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.quantity = input;
        self
    }
    /// Appends an item to `items`.
    ///
    /// To override the contents of this collection use [`set_items`](Self::set_items).
    ///
    /// <p>Contains the functions in the list.</p>
    pub fn items(mut self, input: crate::types::FunctionSummary) -> Self {
        let mut v = self.items.unwrap_or_default();
        v.push(input);
        self.items = ::std::option::Option::Some(v);
        self
    }
    /// <p>Contains the functions in the list.</p>
    pub fn set_items(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FunctionSummary>>,
    ) -> Self {
        self.items = input;
        self
    }
    /// Consumes the builder and constructs a [`FunctionList`](crate::types::FunctionList).
    pub fn build(self) -> crate::types::FunctionList {
        crate::types::FunctionList {
            next_marker: self.next_marker,
            max_items: self.max_items,
            quantity: self.quantity,
            items: self.items,
        }
    }
}
