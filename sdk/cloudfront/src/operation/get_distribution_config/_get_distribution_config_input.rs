// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request to get a distribution configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetDistributionConfigInput {
    /// <p>The distribution's ID. If the ID is empty, an empty distribution configuration is returned.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
}
impl GetDistributionConfigInput {
    /// <p>The distribution's ID. If the ID is empty, an empty distribution configuration is returned.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl GetDistributionConfigInput {
    /// Creates a new builder-style object to manufacture [`GetDistributionConfigInput`](crate::operation::get_distribution_config::GetDistributionConfigInput).
    pub fn builder(
    ) -> crate::operation::get_distribution_config::builders::GetDistributionConfigInputBuilder
    {
        crate::operation::get_distribution_config::builders::GetDistributionConfigInputBuilder::default()
    }
}

/// A builder for [`GetDistributionConfigInput`](crate::operation::get_distribution_config::GetDistributionConfigInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetDistributionConfigInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl GetDistributionConfigInputBuilder {
    /// <p>The distribution's ID. If the ID is empty, an empty distribution configuration is returned.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The distribution's ID. If the ID is empty, an empty distribution configuration is returned.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetDistributionConfigInput`](crate::operation::get_distribution_config::GetDistributionConfigInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_distribution_config::GetDistributionConfigInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_distribution_config::GetDistributionConfigInput { id: self.id },
        )
    }
}
