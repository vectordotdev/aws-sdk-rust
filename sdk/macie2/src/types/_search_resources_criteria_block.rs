// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies property- and tag-based conditions that define filter criteria for including or excluding Amazon Web Services resources from the query results.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SearchResourcesCriteriaBlock {
    /// <p>An array of objects, one for each property- or tag-based condition that includes or excludes resources from the query results. If you specify more than one condition, Amazon Macie uses AND logic to join the conditions.</p>
    #[doc(hidden)]
    pub and: ::std::option::Option<::std::vec::Vec<crate::types::SearchResourcesCriteria>>,
}
impl SearchResourcesCriteriaBlock {
    /// <p>An array of objects, one for each property- or tag-based condition that includes or excludes resources from the query results. If you specify more than one condition, Amazon Macie uses AND logic to join the conditions.</p>
    pub fn and(&self) -> ::std::option::Option<&[crate::types::SearchResourcesCriteria]> {
        self.and.as_deref()
    }
}
impl SearchResourcesCriteriaBlock {
    /// Creates a new builder-style object to manufacture [`SearchResourcesCriteriaBlock`](crate::types::SearchResourcesCriteriaBlock).
    pub fn builder() -> crate::types::builders::SearchResourcesCriteriaBlockBuilder {
        crate::types::builders::SearchResourcesCriteriaBlockBuilder::default()
    }
}

/// A builder for [`SearchResourcesCriteriaBlock`](crate::types::SearchResourcesCriteriaBlock).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SearchResourcesCriteriaBlockBuilder {
    pub(crate) and: ::std::option::Option<::std::vec::Vec<crate::types::SearchResourcesCriteria>>,
}
impl SearchResourcesCriteriaBlockBuilder {
    /// Appends an item to `and`.
    ///
    /// To override the contents of this collection use [`set_and`](Self::set_and).
    ///
    /// <p>An array of objects, one for each property- or tag-based condition that includes or excludes resources from the query results. If you specify more than one condition, Amazon Macie uses AND logic to join the conditions.</p>
    pub fn and(mut self, input: crate::types::SearchResourcesCriteria) -> Self {
        let mut v = self.and.unwrap_or_default();
        v.push(input);
        self.and = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of objects, one for each property- or tag-based condition that includes or excludes resources from the query results. If you specify more than one condition, Amazon Macie uses AND logic to join the conditions.</p>
    pub fn set_and(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SearchResourcesCriteria>>,
    ) -> Self {
        self.and = input;
        self
    }
    /// Consumes the builder and constructs a [`SearchResourcesCriteriaBlock`](crate::types::SearchResourcesCriteriaBlock).
    pub fn build(self) -> crate::types::SearchResourcesCriteriaBlock {
        crate::types::SearchResourcesCriteriaBlock { and: self.and }
    }
}
