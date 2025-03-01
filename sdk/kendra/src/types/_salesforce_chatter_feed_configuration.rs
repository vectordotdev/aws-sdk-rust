// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration information for syncing a Salesforce chatter feed. The contents of the object comes from the Salesforce FeedItem table.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SalesforceChatterFeedConfiguration {
    /// <p>The name of the column in the Salesforce FeedItem table that contains the content to index. Typically this is the <code>Body</code> column.</p>
    #[doc(hidden)]
    pub document_data_field_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the column in the Salesforce FeedItem table that contains the title of the document. This is typically the <code>Title</code> column.</p>
    #[doc(hidden)]
    pub document_title_field_name: ::std::option::Option<::std::string::String>,
    /// <p>Maps fields from a Salesforce chatter feed into Amazon Kendra index fields.</p>
    #[doc(hidden)]
    pub field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    /// <p>Filters the documents in the feed based on status of the user. When you specify <code>ACTIVE_USERS</code> only documents from users who have an active account are indexed. When you specify <code>STANDARD_USER</code> only documents for Salesforce standard users are documented. You can specify both.</p>
    #[doc(hidden)]
    pub include_filter_types: ::std::option::Option<
        ::std::vec::Vec<crate::types::SalesforceChatterFeedIncludeFilterType>,
    >,
}
impl SalesforceChatterFeedConfiguration {
    /// <p>The name of the column in the Salesforce FeedItem table that contains the content to index. Typically this is the <code>Body</code> column.</p>
    pub fn document_data_field_name(&self) -> ::std::option::Option<&str> {
        self.document_data_field_name.as_deref()
    }
    /// <p>The name of the column in the Salesforce FeedItem table that contains the title of the document. This is typically the <code>Title</code> column.</p>
    pub fn document_title_field_name(&self) -> ::std::option::Option<&str> {
        self.document_title_field_name.as_deref()
    }
    /// <p>Maps fields from a Salesforce chatter feed into Amazon Kendra index fields.</p>
    pub fn field_mappings(
        &self,
    ) -> ::std::option::Option<&[crate::types::DataSourceToIndexFieldMapping]> {
        self.field_mappings.as_deref()
    }
    /// <p>Filters the documents in the feed based on status of the user. When you specify <code>ACTIVE_USERS</code> only documents from users who have an active account are indexed. When you specify <code>STANDARD_USER</code> only documents for Salesforce standard users are documented. You can specify both.</p>
    pub fn include_filter_types(
        &self,
    ) -> ::std::option::Option<&[crate::types::SalesforceChatterFeedIncludeFilterType]> {
        self.include_filter_types.as_deref()
    }
}
impl SalesforceChatterFeedConfiguration {
    /// Creates a new builder-style object to manufacture [`SalesforceChatterFeedConfiguration`](crate::types::SalesforceChatterFeedConfiguration).
    pub fn builder() -> crate::types::builders::SalesforceChatterFeedConfigurationBuilder {
        crate::types::builders::SalesforceChatterFeedConfigurationBuilder::default()
    }
}

/// A builder for [`SalesforceChatterFeedConfiguration`](crate::types::SalesforceChatterFeedConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SalesforceChatterFeedConfigurationBuilder {
    pub(crate) document_data_field_name: ::std::option::Option<::std::string::String>,
    pub(crate) document_title_field_name: ::std::option::Option<::std::string::String>,
    pub(crate) field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    pub(crate) include_filter_types: ::std::option::Option<
        ::std::vec::Vec<crate::types::SalesforceChatterFeedIncludeFilterType>,
    >,
}
impl SalesforceChatterFeedConfigurationBuilder {
    /// <p>The name of the column in the Salesforce FeedItem table that contains the content to index. Typically this is the <code>Body</code> column.</p>
    pub fn document_data_field_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.document_data_field_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the column in the Salesforce FeedItem table that contains the content to index. Typically this is the <code>Body</code> column.</p>
    pub fn set_document_data_field_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.document_data_field_name = input;
        self
    }
    /// <p>The name of the column in the Salesforce FeedItem table that contains the title of the document. This is typically the <code>Title</code> column.</p>
    pub fn document_title_field_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.document_title_field_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the column in the Salesforce FeedItem table that contains the title of the document. This is typically the <code>Title</code> column.</p>
    pub fn set_document_title_field_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.document_title_field_name = input;
        self
    }
    /// Appends an item to `field_mappings`.
    ///
    /// To override the contents of this collection use [`set_field_mappings`](Self::set_field_mappings).
    ///
    /// <p>Maps fields from a Salesforce chatter feed into Amazon Kendra index fields.</p>
    pub fn field_mappings(mut self, input: crate::types::DataSourceToIndexFieldMapping) -> Self {
        let mut v = self.field_mappings.unwrap_or_default();
        v.push(input);
        self.field_mappings = ::std::option::Option::Some(v);
        self
    }
    /// <p>Maps fields from a Salesforce chatter feed into Amazon Kendra index fields.</p>
    pub fn set_field_mappings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    ) -> Self {
        self.field_mappings = input;
        self
    }
    /// Appends an item to `include_filter_types`.
    ///
    /// To override the contents of this collection use [`set_include_filter_types`](Self::set_include_filter_types).
    ///
    /// <p>Filters the documents in the feed based on status of the user. When you specify <code>ACTIVE_USERS</code> only documents from users who have an active account are indexed. When you specify <code>STANDARD_USER</code> only documents for Salesforce standard users are documented. You can specify both.</p>
    pub fn include_filter_types(
        mut self,
        input: crate::types::SalesforceChatterFeedIncludeFilterType,
    ) -> Self {
        let mut v = self.include_filter_types.unwrap_or_default();
        v.push(input);
        self.include_filter_types = ::std::option::Option::Some(v);
        self
    }
    /// <p>Filters the documents in the feed based on status of the user. When you specify <code>ACTIVE_USERS</code> only documents from users who have an active account are indexed. When you specify <code>STANDARD_USER</code> only documents for Salesforce standard users are documented. You can specify both.</p>
    pub fn set_include_filter_types(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::SalesforceChatterFeedIncludeFilterType>,
        >,
    ) -> Self {
        self.include_filter_types = input;
        self
    }
    /// Consumes the builder and constructs a [`SalesforceChatterFeedConfiguration`](crate::types::SalesforceChatterFeedConfiguration).
    pub fn build(self) -> crate::types::SalesforceChatterFeedConfiguration {
        crate::types::SalesforceChatterFeedConfiguration {
            document_data_field_name: self.document_data_field_name,
            document_title_field_name: self.document_title_field_name,
            field_mappings: self.field_mappings,
            include_filter_types: self.include_filter_types,
        }
    }
}
