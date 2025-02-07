// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAccountAuthorizationDetailsInput {
    /// <p>A list of entity types used to filter the results. Only the entities that match the types you specify are included in the output. Use the value <code>LocalManagedPolicy</code> to include customer managed policies.</p>
    /// <p>The format for this parameter is a comma-separated (if more than one) list of strings. Each string value in the list must be one of the valid values listed below.</p>
    #[doc(hidden)]
    pub filter: ::std::option::Option<::std::vec::Vec<crate::types::EntityType>>,
    /// <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>
    /// <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
    #[doc(hidden)]
    pub max_items: ::std::option::Option<i32>,
    /// <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    #[doc(hidden)]
    pub marker: ::std::option::Option<::std::string::String>,
}
impl GetAccountAuthorizationDetailsInput {
    /// <p>A list of entity types used to filter the results. Only the entities that match the types you specify are included in the output. Use the value <code>LocalManagedPolicy</code> to include customer managed policies.</p>
    /// <p>The format for this parameter is a comma-separated (if more than one) list of strings. Each string value in the list must be one of the valid values listed below.</p>
    pub fn filter(&self) -> ::std::option::Option<&[crate::types::EntityType]> {
        self.filter.as_deref()
    }
    /// <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>
    /// <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
    pub fn max_items(&self) -> ::std::option::Option<i32> {
        self.max_items
    }
    /// <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    pub fn marker(&self) -> ::std::option::Option<&str> {
        self.marker.as_deref()
    }
}
impl GetAccountAuthorizationDetailsInput {
    /// Creates a new builder-style object to manufacture [`GetAccountAuthorizationDetailsInput`](crate::operation::get_account_authorization_details::GetAccountAuthorizationDetailsInput).
    pub fn builder() -> crate::operation::get_account_authorization_details::builders::GetAccountAuthorizationDetailsInputBuilder{
        crate::operation::get_account_authorization_details::builders::GetAccountAuthorizationDetailsInputBuilder::default()
    }
}

/// A builder for [`GetAccountAuthorizationDetailsInput`](crate::operation::get_account_authorization_details::GetAccountAuthorizationDetailsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetAccountAuthorizationDetailsInputBuilder {
    pub(crate) filter: ::std::option::Option<::std::vec::Vec<crate::types::EntityType>>,
    pub(crate) max_items: ::std::option::Option<i32>,
    pub(crate) marker: ::std::option::Option<::std::string::String>,
}
impl GetAccountAuthorizationDetailsInputBuilder {
    /// Appends an item to `filter`.
    ///
    /// To override the contents of this collection use [`set_filter`](Self::set_filter).
    ///
    /// <p>A list of entity types used to filter the results. Only the entities that match the types you specify are included in the output. Use the value <code>LocalManagedPolicy</code> to include customer managed policies.</p>
    /// <p>The format for this parameter is a comma-separated (if more than one) list of strings. Each string value in the list must be one of the valid values listed below.</p>
    pub fn filter(mut self, input: crate::types::EntityType) -> Self {
        let mut v = self.filter.unwrap_or_default();
        v.push(input);
        self.filter = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of entity types used to filter the results. Only the entities that match the types you specify are included in the output. Use the value <code>LocalManagedPolicy</code> to include customer managed policies.</p>
    /// <p>The format for this parameter is a comma-separated (if more than one) list of strings. Each string value in the list must be one of the valid values listed below.</p>
    pub fn set_filter(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EntityType>>,
    ) -> Self {
        self.filter = input;
        self
    }
    /// <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>
    /// <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.max_items = ::std::option::Option::Some(input);
        self
    }
    /// <p>Use this only when paginating results to indicate the maximum number of items you want in the response. If additional items exist beyond the maximum you specify, the <code>IsTruncated</code> response element is <code>true</code>.</p>
    /// <p>If you do not include this parameter, the number of items defaults to 100. Note that IAM might return fewer results, even when there are more results available. In that case, the <code>IsTruncated</code> response element returns <code>true</code>, and <code>Marker</code> contains a value to include in the subsequent call that tells the service where to continue from.</p>
    pub fn set_max_items(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_items = input;
        self
    }
    /// <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Use this parameter only when paginating results and only after you receive a response indicating that the results are truncated. Set it to the value of the <code>Marker</code> element in the response that you received to indicate where the next call should start.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// Consumes the builder and constructs a [`GetAccountAuthorizationDetailsInput`](crate::operation::get_account_authorization_details::GetAccountAuthorizationDetailsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_account_authorization_details::GetAccountAuthorizationDetailsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_account_authorization_details::GetAccountAuthorizationDetailsInput {
                filter: self.filter
                ,
                max_items: self.max_items
                ,
                marker: self.marker
                ,
            }
        )
    }
}
