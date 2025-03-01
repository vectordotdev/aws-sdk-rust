// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetResourceSetInput {
    /// <p>Name of a resource set.</p>
    #[doc(hidden)]
    pub resource_set_name: ::std::option::Option<::std::string::String>,
}
impl GetResourceSetInput {
    /// <p>Name of a resource set.</p>
    pub fn resource_set_name(&self) -> ::std::option::Option<&str> {
        self.resource_set_name.as_deref()
    }
}
impl GetResourceSetInput {
    /// Creates a new builder-style object to manufacture [`GetResourceSetInput`](crate::operation::get_resource_set::GetResourceSetInput).
    pub fn builder() -> crate::operation::get_resource_set::builders::GetResourceSetInputBuilder {
        crate::operation::get_resource_set::builders::GetResourceSetInputBuilder::default()
    }
}

/// A builder for [`GetResourceSetInput`](crate::operation::get_resource_set::GetResourceSetInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetResourceSetInputBuilder {
    pub(crate) resource_set_name: ::std::option::Option<::std::string::String>,
}
impl GetResourceSetInputBuilder {
    /// <p>Name of a resource set.</p>
    pub fn resource_set_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.resource_set_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of a resource set.</p>
    pub fn set_resource_set_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.resource_set_name = input;
        self
    }
    /// Consumes the builder and constructs a [`GetResourceSetInput`](crate::operation::get_resource_set::GetResourceSetInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_resource_set::GetResourceSetInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_resource_set::GetResourceSetInput {
            resource_set_name: self.resource_set_name,
        })
    }
}
