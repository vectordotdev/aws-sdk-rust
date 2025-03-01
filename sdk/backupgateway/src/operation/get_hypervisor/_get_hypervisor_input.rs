// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetHypervisorInput {
    /// <p>The Amazon Resource Name (ARN) of the hypervisor.</p>
    #[doc(hidden)]
    pub hypervisor_arn: ::std::option::Option<::std::string::String>,
}
impl GetHypervisorInput {
    /// <p>The Amazon Resource Name (ARN) of the hypervisor.</p>
    pub fn hypervisor_arn(&self) -> ::std::option::Option<&str> {
        self.hypervisor_arn.as_deref()
    }
}
impl GetHypervisorInput {
    /// Creates a new builder-style object to manufacture [`GetHypervisorInput`](crate::operation::get_hypervisor::GetHypervisorInput).
    pub fn builder() -> crate::operation::get_hypervisor::builders::GetHypervisorInputBuilder {
        crate::operation::get_hypervisor::builders::GetHypervisorInputBuilder::default()
    }
}

/// A builder for [`GetHypervisorInput`](crate::operation::get_hypervisor::GetHypervisorInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetHypervisorInputBuilder {
    pub(crate) hypervisor_arn: ::std::option::Option<::std::string::String>,
}
impl GetHypervisorInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the hypervisor.</p>
    pub fn hypervisor_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.hypervisor_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the hypervisor.</p>
    pub fn set_hypervisor_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.hypervisor_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`GetHypervisorInput`](crate::operation::get_hypervisor::GetHypervisorInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_hypervisor::GetHypervisorInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_hypervisor::GetHypervisorInput {
            hypervisor_arn: self.hypervisor_arn,
        })
    }
}
