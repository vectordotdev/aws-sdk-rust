// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListVpcEndpointAccessInput {
    /// <p>The name of the OpenSearch Service domain to retrieve access information for.</p>
    #[doc(hidden)]
    pub domain_name: ::std::option::Option<::std::string::String>,
    /// <p>If your initial <code>ListVpcEndpointAccess</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListVpcEndpointAccess</code> operations, which returns results in the next page.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListVpcEndpointAccessInput {
    /// <p>The name of the OpenSearch Service domain to retrieve access information for.</p>
    pub fn domain_name(&self) -> ::std::option::Option<&str> {
        self.domain_name.as_deref()
    }
    /// <p>If your initial <code>ListVpcEndpointAccess</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListVpcEndpointAccess</code> operations, which returns results in the next page.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListVpcEndpointAccessInput {
    /// Creates a new builder-style object to manufacture [`ListVpcEndpointAccessInput`](crate::operation::list_vpc_endpoint_access::ListVpcEndpointAccessInput).
    pub fn builder(
    ) -> crate::operation::list_vpc_endpoint_access::builders::ListVpcEndpointAccessInputBuilder
    {
        crate::operation::list_vpc_endpoint_access::builders::ListVpcEndpointAccessInputBuilder::default()
    }
}

/// A builder for [`ListVpcEndpointAccessInput`](crate::operation::list_vpc_endpoint_access::ListVpcEndpointAccessInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListVpcEndpointAccessInputBuilder {
    pub(crate) domain_name: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListVpcEndpointAccessInputBuilder {
    /// <p>The name of the OpenSearch Service domain to retrieve access information for.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the OpenSearch Service domain to retrieve access information for.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_name = input;
        self
    }
    /// <p>If your initial <code>ListVpcEndpointAccess</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListVpcEndpointAccess</code> operations, which returns results in the next page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If your initial <code>ListVpcEndpointAccess</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListVpcEndpointAccess</code> operations, which returns results in the next page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListVpcEndpointAccessInput`](crate::operation::list_vpc_endpoint_access::ListVpcEndpointAccessInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_vpc_endpoint_access::ListVpcEndpointAccessInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_vpc_endpoint_access::ListVpcEndpointAccessInput {
                domain_name: self.domain_name,
                next_token: self.next_token,
            },
        )
    }
}
