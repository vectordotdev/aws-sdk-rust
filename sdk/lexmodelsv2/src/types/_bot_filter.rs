// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Filters the responses returned by the <code>ListBots</code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BotFilter {
    /// <p>The name of the field to filter the list of bots.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<crate::types::BotFilterName>,
    /// <p>The value to use for filtering the list of bots.</p>
    #[doc(hidden)]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The operator to use for the filter. Specify <code>EQ</code> when the <code>ListBots</code> operation should return only aliases that equal the specified value. Specify <code>CO</code> when the <code>ListBots</code> operation should return aliases that contain the specified value.</p>
    #[doc(hidden)]
    pub operator: ::std::option::Option<crate::types::BotFilterOperator>,
}
impl BotFilter {
    /// <p>The name of the field to filter the list of bots.</p>
    pub fn name(&self) -> ::std::option::Option<&crate::types::BotFilterName> {
        self.name.as_ref()
    }
    /// <p>The value to use for filtering the list of bots.</p>
    pub fn values(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.values.as_deref()
    }
    /// <p>The operator to use for the filter. Specify <code>EQ</code> when the <code>ListBots</code> operation should return only aliases that equal the specified value. Specify <code>CO</code> when the <code>ListBots</code> operation should return aliases that contain the specified value.</p>
    pub fn operator(&self) -> ::std::option::Option<&crate::types::BotFilterOperator> {
        self.operator.as_ref()
    }
}
impl BotFilter {
    /// Creates a new builder-style object to manufacture [`BotFilter`](crate::types::BotFilter).
    pub fn builder() -> crate::types::builders::BotFilterBuilder {
        crate::types::builders::BotFilterBuilder::default()
    }
}

/// A builder for [`BotFilter`](crate::types::BotFilter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BotFilterBuilder {
    pub(crate) name: ::std::option::Option<crate::types::BotFilterName>,
    pub(crate) values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) operator: ::std::option::Option<crate::types::BotFilterOperator>,
}
impl BotFilterBuilder {
    /// <p>The name of the field to filter the list of bots.</p>
    pub fn name(mut self, input: crate::types::BotFilterName) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the field to filter the list of bots.</p>
    pub fn set_name(mut self, input: ::std::option::Option<crate::types::BotFilterName>) -> Self {
        self.name = input;
        self
    }
    /// Appends an item to `values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p>The value to use for filtering the list of bots.</p>
    pub fn values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.values.unwrap_or_default();
        v.push(input.into());
        self.values = ::std::option::Option::Some(v);
        self
    }
    /// <p>The value to use for filtering the list of bots.</p>
    pub fn set_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.values = input;
        self
    }
    /// <p>The operator to use for the filter. Specify <code>EQ</code> when the <code>ListBots</code> operation should return only aliases that equal the specified value. Specify <code>CO</code> when the <code>ListBots</code> operation should return aliases that contain the specified value.</p>
    pub fn operator(mut self, input: crate::types::BotFilterOperator) -> Self {
        self.operator = ::std::option::Option::Some(input);
        self
    }
    /// <p>The operator to use for the filter. Specify <code>EQ</code> when the <code>ListBots</code> operation should return only aliases that equal the specified value. Specify <code>CO</code> when the <code>ListBots</code> operation should return aliases that contain the specified value.</p>
    pub fn set_operator(
        mut self,
        input: ::std::option::Option<crate::types::BotFilterOperator>,
    ) -> Self {
        self.operator = input;
        self
    }
    /// Consumes the builder and constructs a [`BotFilter`](crate::types::BotFilter).
    pub fn build(self) -> crate::types::BotFilter {
        crate::types::BotFilter {
            name: self.name,
            values: self.values,
            operator: self.operator,
        }
    }
}
