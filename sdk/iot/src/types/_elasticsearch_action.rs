// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an action that writes data to an Amazon OpenSearch Service domain.</p> <note>
/// <p>The <code>Elasticsearch</code> action can only be used by existing rule actions. To create a new rule action or to update an existing rule action, use the <code>OpenSearch</code> rule action instead. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/apireference/API_OpenSearchAction.html">OpenSearchAction</a>.</p>
/// </note>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ElasticsearchAction {
    /// <p>The IAM role ARN that has access to OpenSearch.</p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>The endpoint of your OpenSearch domain.</p>
    #[doc(hidden)]
    pub endpoint: ::std::option::Option<::std::string::String>,
    /// <p>The index where you want to store your data.</p>
    #[doc(hidden)]
    pub index: ::std::option::Option<::std::string::String>,
    /// <p>The type of document you are storing.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for the document you are storing.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
}
impl ElasticsearchAction {
    /// <p>The IAM role ARN that has access to OpenSearch.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>The endpoint of your OpenSearch domain.</p>
    pub fn endpoint(&self) -> ::std::option::Option<&str> {
        self.endpoint.as_deref()
    }
    /// <p>The index where you want to store your data.</p>
    pub fn index(&self) -> ::std::option::Option<&str> {
        self.index.as_deref()
    }
    /// <p>The type of document you are storing.</p>
    pub fn r#type(&self) -> ::std::option::Option<&str> {
        self.r#type.as_deref()
    }
    /// <p>The unique identifier for the document you are storing.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl ElasticsearchAction {
    /// Creates a new builder-style object to manufacture [`ElasticsearchAction`](crate::types::ElasticsearchAction).
    pub fn builder() -> crate::types::builders::ElasticsearchActionBuilder {
        crate::types::builders::ElasticsearchActionBuilder::default()
    }
}

/// A builder for [`ElasticsearchAction`](crate::types::ElasticsearchAction).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ElasticsearchActionBuilder {
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) endpoint: ::std::option::Option<::std::string::String>,
    pub(crate) index: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl ElasticsearchActionBuilder {
    /// <p>The IAM role ARN that has access to OpenSearch.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IAM role ARN that has access to OpenSearch.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>The endpoint of your OpenSearch domain.</p>
    pub fn endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.endpoint = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The endpoint of your OpenSearch domain.</p>
    pub fn set_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.endpoint = input;
        self
    }
    /// <p>The index where you want to store your data.</p>
    pub fn index(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The index where you want to store your data.</p>
    pub fn set_index(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index = input;
        self
    }
    /// <p>The type of document you are storing.</p>
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.r#type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of document you are storing.</p>
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The unique identifier for the document you are storing.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the document you are storing.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Consumes the builder and constructs a [`ElasticsearchAction`](crate::types::ElasticsearchAction).
    pub fn build(self) -> crate::types::ElasticsearchAction {
        crate::types::ElasticsearchAction {
            role_arn: self.role_arn,
            endpoint: self.endpoint,
            index: self.index,
            r#type: self.r#type,
            id: self.id,
        }
    }
}
