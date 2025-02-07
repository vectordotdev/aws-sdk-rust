// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IsVpcPeeredInput {}
impl IsVpcPeeredInput {
    /// Creates a new builder-style object to manufacture [`IsVpcPeeredInput`](crate::operation::is_vpc_peered::IsVpcPeeredInput).
    pub fn builder() -> crate::operation::is_vpc_peered::builders::IsVpcPeeredInputBuilder {
        crate::operation::is_vpc_peered::builders::IsVpcPeeredInputBuilder::default()
    }
}

/// A builder for [`IsVpcPeeredInput`](crate::operation::is_vpc_peered::IsVpcPeeredInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct IsVpcPeeredInputBuilder {}
impl IsVpcPeeredInputBuilder {
    /// Consumes the builder and constructs a [`IsVpcPeeredInput`](crate::operation::is_vpc_peered::IsVpcPeeredInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::is_vpc_peered::IsVpcPeeredInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::is_vpc_peered::IsVpcPeeredInput {})
    }
}
