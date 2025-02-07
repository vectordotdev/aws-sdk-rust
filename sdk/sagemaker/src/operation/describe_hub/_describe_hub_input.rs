// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeHubInput {
    /// <p>The name of the hub to describe.</p>
    #[doc(hidden)]
    pub hub_name: ::std::option::Option<::std::string::String>,
}
impl DescribeHubInput {
    /// <p>The name of the hub to describe.</p>
    pub fn hub_name(&self) -> ::std::option::Option<&str> {
        self.hub_name.as_deref()
    }
}
impl DescribeHubInput {
    /// Creates a new builder-style object to manufacture [`DescribeHubInput`](crate::operation::describe_hub::DescribeHubInput).
    pub fn builder() -> crate::operation::describe_hub::builders::DescribeHubInputBuilder {
        crate::operation::describe_hub::builders::DescribeHubInputBuilder::default()
    }
}

/// A builder for [`DescribeHubInput`](crate::operation::describe_hub::DescribeHubInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeHubInputBuilder {
    pub(crate) hub_name: ::std::option::Option<::std::string::String>,
}
impl DescribeHubInputBuilder {
    /// <p>The name of the hub to describe.</p>
    pub fn hub_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.hub_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the hub to describe.</p>
    pub fn set_hub_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.hub_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeHubInput`](crate::operation::describe_hub::DescribeHubInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_hub::DescribeHubInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_hub::DescribeHubInput {
            hub_name: self.hub_name,
        })
    }
}
