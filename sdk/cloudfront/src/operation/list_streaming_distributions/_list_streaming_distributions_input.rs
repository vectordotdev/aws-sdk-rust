// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request to list your streaming distributions.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListStreamingDistributionsInput {
    /// <p>The value that you provided for the <code>Marker</code> request parameter.</p>
    #[doc(hidden)]
    pub marker: ::std::option::Option<::std::string::String>,
    /// <p>The value that you provided for the <code>MaxItems</code> request parameter.</p>
    #[doc(hidden)]
    pub max_items: ::std::option::Option<i32>,
}
impl ListStreamingDistributionsInput {
    /// <p>The value that you provided for the <code>Marker</code> request parameter.</p>
    pub fn marker(&self) -> ::std::option::Option<&str> {
        self.marker.as_deref()
    }
    /// <p>The value that you provided for the <code>MaxItems</code> request parameter.</p>
    pub fn max_items(&self) -> ::std::option::Option<i32> {
        self.max_items
    }
}
impl ListStreamingDistributionsInput {
    /// Creates a new builder-style object to manufacture [`ListStreamingDistributionsInput`](crate::operation::list_streaming_distributions::ListStreamingDistributionsInput).
    pub fn builder() -> crate::operation::list_streaming_distributions::builders::ListStreamingDistributionsInputBuilder{
        crate::operation::list_streaming_distributions::builders::ListStreamingDistributionsInputBuilder::default()
    }
}

/// A builder for [`ListStreamingDistributionsInput`](crate::operation::list_streaming_distributions::ListStreamingDistributionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListStreamingDistributionsInputBuilder {
    pub(crate) marker: ::std::option::Option<::std::string::String>,
    pub(crate) max_items: ::std::option::Option<i32>,
}
impl ListStreamingDistributionsInputBuilder {
    /// <p>The value that you provided for the <code>Marker</code> request parameter.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value that you provided for the <code>Marker</code> request parameter.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// <p>The value that you provided for the <code>MaxItems</code> request parameter.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.max_items = ::std::option::Option::Some(input);
        self
    }
    /// <p>The value that you provided for the <code>MaxItems</code> request parameter.</p>
    pub fn set_max_items(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_items = input;
        self
    }
    /// Consumes the builder and constructs a [`ListStreamingDistributionsInput`](crate::operation::list_streaming_distributions::ListStreamingDistributionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_streaming_distributions::ListStreamingDistributionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_streaming_distributions::ListStreamingDistributionsInput {
                marker: self.marker,
                max_items: self.max_items,
            },
        )
    }
}
