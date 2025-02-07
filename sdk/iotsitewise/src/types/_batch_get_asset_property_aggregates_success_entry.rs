// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains success information for an entry that is associated with the <a href="https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/API_BatchGetAssetPropertyAggregates.html">BatchGetAssetPropertyAggregates</a> API.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchGetAssetPropertyAggregatesSuccessEntry {
    /// <p>The ID of the entry.</p>
    #[doc(hidden)]
    pub entry_id: ::std::option::Option<::std::string::String>,
    /// <p>The requested aggregated asset property values (for example, average, minimum, and maximum).</p>
    #[doc(hidden)]
    pub aggregated_values: ::std::option::Option<::std::vec::Vec<crate::types::AggregatedValue>>,
}
impl BatchGetAssetPropertyAggregatesSuccessEntry {
    /// <p>The ID of the entry.</p>
    pub fn entry_id(&self) -> ::std::option::Option<&str> {
        self.entry_id.as_deref()
    }
    /// <p>The requested aggregated asset property values (for example, average, minimum, and maximum).</p>
    pub fn aggregated_values(&self) -> ::std::option::Option<&[crate::types::AggregatedValue]> {
        self.aggregated_values.as_deref()
    }
}
impl BatchGetAssetPropertyAggregatesSuccessEntry {
    /// Creates a new builder-style object to manufacture [`BatchGetAssetPropertyAggregatesSuccessEntry`](crate::types::BatchGetAssetPropertyAggregatesSuccessEntry).
    pub fn builder() -> crate::types::builders::BatchGetAssetPropertyAggregatesSuccessEntryBuilder {
        crate::types::builders::BatchGetAssetPropertyAggregatesSuccessEntryBuilder::default()
    }
}

/// A builder for [`BatchGetAssetPropertyAggregatesSuccessEntry`](crate::types::BatchGetAssetPropertyAggregatesSuccessEntry).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchGetAssetPropertyAggregatesSuccessEntryBuilder {
    pub(crate) entry_id: ::std::option::Option<::std::string::String>,
    pub(crate) aggregated_values:
        ::std::option::Option<::std::vec::Vec<crate::types::AggregatedValue>>,
}
impl BatchGetAssetPropertyAggregatesSuccessEntryBuilder {
    /// <p>The ID of the entry.</p>
    pub fn entry_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.entry_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the entry.</p>
    pub fn set_entry_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.entry_id = input;
        self
    }
    /// Appends an item to `aggregated_values`.
    ///
    /// To override the contents of this collection use [`set_aggregated_values`](Self::set_aggregated_values).
    ///
    /// <p>The requested aggregated asset property values (for example, average, minimum, and maximum).</p>
    pub fn aggregated_values(mut self, input: crate::types::AggregatedValue) -> Self {
        let mut v = self.aggregated_values.unwrap_or_default();
        v.push(input);
        self.aggregated_values = ::std::option::Option::Some(v);
        self
    }
    /// <p>The requested aggregated asset property values (for example, average, minimum, and maximum).</p>
    pub fn set_aggregated_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AggregatedValue>>,
    ) -> Self {
        self.aggregated_values = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchGetAssetPropertyAggregatesSuccessEntry`](crate::types::BatchGetAssetPropertyAggregatesSuccessEntry).
    pub fn build(self) -> crate::types::BatchGetAssetPropertyAggregatesSuccessEntry {
        crate::types::BatchGetAssetPropertyAggregatesSuccessEntry {
            entry_id: self.entry_id,
            aggregated_values: self.aggregated_values,
        }
    }
}
