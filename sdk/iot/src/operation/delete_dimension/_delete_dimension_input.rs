// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteDimensionInput {
    /// <p>The unique identifier for the dimension that you want to delete.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl DeleteDimensionInput {
    /// <p>The unique identifier for the dimension that you want to delete.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl DeleteDimensionInput {
    /// Creates a new builder-style object to manufacture [`DeleteDimensionInput`](crate::operation::delete_dimension::DeleteDimensionInput).
    pub fn builder() -> crate::operation::delete_dimension::builders::DeleteDimensionInputBuilder {
        crate::operation::delete_dimension::builders::DeleteDimensionInputBuilder::default()
    }
}

/// A builder for [`DeleteDimensionInput`](crate::operation::delete_dimension::DeleteDimensionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteDimensionInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl DeleteDimensionInputBuilder {
    /// <p>The unique identifier for the dimension that you want to delete.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the dimension that you want to delete.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteDimensionInput`](crate::operation::delete_dimension::DeleteDimensionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_dimension::DeleteDimensionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_dimension::DeleteDimensionInput {
            name: self.name,
        })
    }
}
