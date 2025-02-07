// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an OpenSearch data source configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OpenSearchServiceDataSourceConfig {
    /// <p>The endpoint.</p>
    #[doc(hidden)]
    pub endpoint: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services Region.</p>
    #[doc(hidden)]
    pub aws_region: ::std::option::Option<::std::string::String>,
}
impl OpenSearchServiceDataSourceConfig {
    /// <p>The endpoint.</p>
    pub fn endpoint(&self) -> ::std::option::Option<&str> {
        self.endpoint.as_deref()
    }
    /// <p>The Amazon Web Services Region.</p>
    pub fn aws_region(&self) -> ::std::option::Option<&str> {
        self.aws_region.as_deref()
    }
}
impl OpenSearchServiceDataSourceConfig {
    /// Creates a new builder-style object to manufacture [`OpenSearchServiceDataSourceConfig`](crate::types::OpenSearchServiceDataSourceConfig).
    pub fn builder() -> crate::types::builders::OpenSearchServiceDataSourceConfigBuilder {
        crate::types::builders::OpenSearchServiceDataSourceConfigBuilder::default()
    }
}

/// A builder for [`OpenSearchServiceDataSourceConfig`](crate::types::OpenSearchServiceDataSourceConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OpenSearchServiceDataSourceConfigBuilder {
    pub(crate) endpoint: ::std::option::Option<::std::string::String>,
    pub(crate) aws_region: ::std::option::Option<::std::string::String>,
}
impl OpenSearchServiceDataSourceConfigBuilder {
    /// <p>The endpoint.</p>
    pub fn endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.endpoint = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The endpoint.</p>
    pub fn set_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.endpoint = input;
        self
    }
    /// <p>The Amazon Web Services Region.</p>
    pub fn aws_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.aws_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services Region.</p>
    pub fn set_aws_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.aws_region = input;
        self
    }
    /// Consumes the builder and constructs a [`OpenSearchServiceDataSourceConfig`](crate::types::OpenSearchServiceDataSourceConfig).
    pub fn build(self) -> crate::types::OpenSearchServiceDataSourceConfig {
        crate::types::OpenSearchServiceDataSourceConfig {
            endpoint: self.endpoint,
            aws_region: self.aws_region,
        }
    }
}
