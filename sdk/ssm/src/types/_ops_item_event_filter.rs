// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a filter for a specific list of OpsItem events. You can filter event information by using tags. You specify tags by using a key-value pair mapping. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OpsItemEventFilter {
    /// <p>The name of the filter key. Currently, the only supported value is <code>OpsItemId</code>.</p>
    #[doc(hidden)]
    pub key: ::std::option::Option<crate::types::OpsItemEventFilterKey>,
    /// <p>The values for the filter, consisting of one or more OpsItem IDs.</p>
    #[doc(hidden)]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The operator used by the filter call. Currently, the only supported value is <code>Equal</code>.</p>
    #[doc(hidden)]
    pub operator: ::std::option::Option<crate::types::OpsItemEventFilterOperator>,
}
impl OpsItemEventFilter {
    /// <p>The name of the filter key. Currently, the only supported value is <code>OpsItemId</code>.</p>
    pub fn key(&self) -> ::std::option::Option<&crate::types::OpsItemEventFilterKey> {
        self.key.as_ref()
    }
    /// <p>The values for the filter, consisting of one or more OpsItem IDs.</p>
    pub fn values(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.values.as_deref()
    }
    /// <p>The operator used by the filter call. Currently, the only supported value is <code>Equal</code>.</p>
    pub fn operator(&self) -> ::std::option::Option<&crate::types::OpsItemEventFilterOperator> {
        self.operator.as_ref()
    }
}
impl OpsItemEventFilter {
    /// Creates a new builder-style object to manufacture [`OpsItemEventFilter`](crate::types::OpsItemEventFilter).
    pub fn builder() -> crate::types::builders::OpsItemEventFilterBuilder {
        crate::types::builders::OpsItemEventFilterBuilder::default()
    }
}

/// A builder for [`OpsItemEventFilter`](crate::types::OpsItemEventFilter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OpsItemEventFilterBuilder {
    pub(crate) key: ::std::option::Option<crate::types::OpsItemEventFilterKey>,
    pub(crate) values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) operator: ::std::option::Option<crate::types::OpsItemEventFilterOperator>,
}
impl OpsItemEventFilterBuilder {
    /// <p>The name of the filter key. Currently, the only supported value is <code>OpsItemId</code>.</p>
    pub fn key(mut self, input: crate::types::OpsItemEventFilterKey) -> Self {
        self.key = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the filter key. Currently, the only supported value is <code>OpsItemId</code>.</p>
    pub fn set_key(
        mut self,
        input: ::std::option::Option<crate::types::OpsItemEventFilterKey>,
    ) -> Self {
        self.key = input;
        self
    }
    /// Appends an item to `values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p>The values for the filter, consisting of one or more OpsItem IDs.</p>
    pub fn values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.values.unwrap_or_default();
        v.push(input.into());
        self.values = ::std::option::Option::Some(v);
        self
    }
    /// <p>The values for the filter, consisting of one or more OpsItem IDs.</p>
    pub fn set_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.values = input;
        self
    }
    /// <p>The operator used by the filter call. Currently, the only supported value is <code>Equal</code>.</p>
    pub fn operator(mut self, input: crate::types::OpsItemEventFilterOperator) -> Self {
        self.operator = ::std::option::Option::Some(input);
        self
    }
    /// <p>The operator used by the filter call. Currently, the only supported value is <code>Equal</code>.</p>
    pub fn set_operator(
        mut self,
        input: ::std::option::Option<crate::types::OpsItemEventFilterOperator>,
    ) -> Self {
        self.operator = input;
        self
    }
    /// Consumes the builder and constructs a [`OpsItemEventFilter`](crate::types::OpsItemEventFilter).
    pub fn build(self) -> crate::types::OpsItemEventFilter {
        crate::types::OpsItemEventFilter {
            key: self.key,
            values: self.values,
            operator: self.operator,
        }
    }
}
